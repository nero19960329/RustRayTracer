#!/usr/bin/env python3
import io
import os
from pathlib import Path
import subprocess

import argparse
from dataclasses import dataclass
import git
from loguru import logger
from PIL import Image
import requests
import time
import toml
import yaml


def upload_to_imgur(
    imgur_access_token: str,
    imgur_album_id: str,
    imgur_title: str,
    imgur_description: str,
    imgur_name: str,
    imgur_type: str,
    imgur_file_io: io.BytesIO,
    request_timeout: int = 10,
):
    return requests.request(
        "POST",
        "https://api.imgur.com/3/image",
        headers={
            "Authorization": f"Bearer {imgur_access_token}",
        },
        data={
            "album": imgur_album_id,
            "title": imgur_title,
            "description": imgur_description,
            "name": imgur_name,
            "type": imgur_type,
        },
        files={
            "image": imgur_file_io,
        },
        timeout=request_timeout,
    )


def run_executable(
    args: list[str],
    log_file: Path,
):
    with open(log_file, "w", encoding="utf-8") as f:
        subprocess.run(
            args,
            stdout=f,
            stderr=f,
            env={
                **os.environ,
                "RUST_BACKTRACE": "1",
                "RUST_LOG": "info",
            },
            check=True,
        )
        f.flush()


def convert_png_to_jpg(
    png_path: Path,
    jpg_path: Path,
):
    im = Image.open(png_path)
    im = im.convert("RGB")
    im.save(jpg_path)


class Timer:
    def __init__(self):
        self.start_time = None
        self.end_time = None

    def __enter__(self):
        self.start_time = time.time()
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        self.end_time = time.time()

    def elapsed(self):
        return self.end_time - self.start_time


@dataclass
class Task:
    name: str

    image_links: list[str]
    image_captions: list[str]
    log_links: list[str]

    # measurements
    time_costs: list[float]


@dataclass
class Report:
    commit_hash: str
    commit_description: str

    tasks: list[Task]

    id: str | None = None

    def html(self):
        title = f"Report - {self.id}" if self.id else "Report"

        tasks_html = ""
        for task in self.tasks:
            # image -> horizontal middle
            # log, time, ... -> bullet list
            for image_link, image_caption, log_link, time_cost in zip(
                task.image_links,
                task.image_captions,
                task.log_links,
                task.time_costs,
            ):
                tasks_html += f"""
                <h2>{task.name}</h2>
                <p><img src="{image_link}" /></p>
                <p>{image_caption}</p>
                <p><a href="{log_link}">Log</a></p>
                <p>Time cost: {time_cost:.2f}s</p>
                """

        return f"""
        <html>
            <head>
                <title>{title}</title>
            </head>
            <body>
                <h1>{title}</h1>
                <p>Commit: {self.commit_hash}</p>
                <p>Commit description: {self.commit_description}</p>
                {tasks_html}
            </body>
        </html>
        """


def main(
    executable: Path,
    config: Path,
    output_dir: Path,
    id: str | None = None,
    upload: bool = False,
    compress: bool = True,
):
    config = yaml.safe_load(open(config, "r", encoding="utf-8"))
    repo = git.Repo()
    report = Report(
        commit_hash=repo.head.commit.hexsha,
        commit_description=repo.head.commit.message,
        tasks=[],
        id=id,
    )

    for task in config["tasks"]:
        name = task["name"]
        scene_config = Path(task["scene_config"])
        render_configs = task["render_configs"]

        report.tasks.append(
            Task(
                name=name,
                image_links=[],
                image_captions=[],
                log_links=[],
                time_costs=[],
            )
        )
        task_data = report.tasks[-1]

        output_dir.mkdir(parents=True, exist_ok=True)
        for render_config in render_configs:
            render_config = Path(render_config)
            image_path = output_dir / name / f"{render_config.stem}.png"
            log_path = output_dir / name / f"{render_config.stem}.log"
            (output_dir / name).mkdir(parents=True, exist_ok=True)
            with Timer() as timer:
                run_executable(
                    [
                        str(executable),
                        "--scene-config",
                        str(scene_config),
                        "--render-config",
                        str(render_config),
                        "--output",
                        str(image_path),
                    ],
                    log_file=log_path,
                )
            task_data.time_costs.append(timer.elapsed())

            if compress:
                jpg_path = output_dir / name / f"{render_config.stem}.jpg"
                convert_png_to_jpg(
                    png_path=image_path,
                    jpg_path=jpg_path,
                )
                if upload:
                    os.remove(image_path)
                image_path = jpg_path

            if not upload:
                image_link = str(Path(*image_path.parts[1:]))
            else:
                resp = upload_to_imgur(
                    imgur_access_token=os.environ["IMGUR_ACCESS_TOKEN"],
                    imgur_album_id=os.environ["IMGUR_ALBUM_ID"],
                    imgur_title=f"{report.commit_hash} - {name} ({render_config.stem})",
                    imgur_description=report.commit_description,
                    imgur_name=f"{name}.{image_path.suffix[1:]}",
                    imgur_type=image_path.suffix[1:],
                    imgur_file_io=open(image_path, "rb"),
                )
                resp.raise_for_status()
                image_link = resp.json()["data"]["link"]
                os.remove(image_path)
            task_data.image_links.append(image_link)

            render_config_data = toml.load(open(render_config, "r", encoding="utf-8"))
            width = render_config_data["image"]["width"]
            height = render_config_data["image"]["height"]
            sampler = render_config_data["sampler"]["type"]
            spp = render_config_data["sampler"]["samples_per_pixel"]
            image_caption = f"{name}@{sampler} Sampler x {spp}spp@{width} x {height}"
            task_data.image_captions.append(image_caption)

            if not upload:
                log_link = str(Path(name) / f"{render_config.stem}.log")
            else:
                assert report.id is not None, "id is None"
                log_link = f"https://storage.cloud.google.com/rust-ray-tracer/{report.id}/{name}/{log_path.name}"
            task_data.log_links.append(log_link)

    with open(output_dir / "report.html", "w", encoding="utf-8") as f:
        f.write(report.html())
    logger.info("Report generated at {}", output_dir)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--bin", type=str, required=True)
    parser.add_argument("--config", type=str, required=True)
    parser.add_argument("--output_dir", type=str, required=True)
    parser.add_argument("--id", type=str)
    parser.add_argument("--upload", action="store_true")
    parser.add_argument("--no_compress", action="store_true")
    args = parser.parse_args()

    main(
        executable=Path(args.bin),
        config=Path(args.config),
        output_dir=Path(args.output_dir),
        id=args.id,
        upload=args.upload,
        compress=not args.no_compress,
    )
