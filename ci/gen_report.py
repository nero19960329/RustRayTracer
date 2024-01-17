#!/usr/bin/env python3
import io
import os
from pathlib import Path
import subprocess

import argparse
import git
from loguru import logger
from PIL import Image
import requests
import toml


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


def main(
    executable: Path,
    report_html: Path,
):
    # render scenes
    pwd = Path.cwd()
    artifact = pwd / "output.png"
    if os.path.exists(artifact):
        os.remove(artifact)
    if not os.path.exists(executable):
        raise RuntimeError(f"executable {executable} does not exist")

    scene_config_path = "configs/scene_cornell_box.toml"
    render_config_path = "configs/render_debug_cornell_box.toml"
    render_config = toml.load(open(render_config_path, "r", encoding="utf-8"))
    log = open("log.txt", "w", encoding="utf-8")
    subprocess.run(
        [
            str(executable),
            "--scene-config",
            scene_config_path,
            "--render-config",
            render_config_path,
            "--output",
            "output.png",
        ],
        stdout=log,
        stderr=log,
        env={
            **os.environ,
            "RUST_BACKTRACE": "1",
            "RUST_LOG": "info",
        },
        check=True,
    )
    log.flush()
    log.close()

    # turn output.png -> output.jpg
    im = Image.open(artifact)
    im = im.convert("RGB")
    im.save("output.jpg")
    artifact = pwd / "output.jpg"

    # upload to imgur
    repo = git.Repo()
    commit = repo.head.commit
    commit_hash = commit.hexsha
    commit_description = commit.message

    width = render_config["image"]["width"]
    height = render_config["image"]["height"]
    spp = render_config["image"]["samples_per_pixel"]

    resp = upload_to_imgur(
        imgur_access_token=os.environ["IMGUR_ACCESS_TOKEN"],
        imgur_album_id=os.environ["IMGUR_ALBUM_ID"],
        imgur_title=f"{commit_hash} - cornell_box ({width}x{height}, {spp}spp)",
        imgur_description=commit_description,
        imgur_name="cornell_box.jpg",
        imgur_type="jpg",
        imgur_file_io=open(artifact, "rb"),
    )
    resp.raise_for_status()
    logger.info(f"uploaded to imgur: {resp.json()['data']['title']}")

    # generate static html
    github_run_number = os.getenv("GITHUB_RUN_NUMBER")
    if not github_run_number:
        log_link = "log.txt"
    else:
        log_link = f"https://storage.cloud.google.com/rust-ray-tracer/{github_run_number}/log.txt"
    html = f"""
    <html>
        <head>
            <title>Report</title>
        </head>
        <body>
            <h1>Report</h1>
            <p>Commit: {commit_hash}</p>
            <p>Commit description: {commit_description}</p>
            <p>Image: <a href="{resp.json()['data']['link']}">{resp.json()['data']['link']}</a></p>
            <img src="{resp.json()['data']['link']}">
            <p>Log: <a href="{log_link}">log.txt</a></p>
        </body>
    </html>
    """
    with open(report_html, "w", encoding="utf-8") as f:
        f.write(html)
    logger.info("generated report.html")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--bin", type=str, required=True)
    parser.add_argument("--report-html", type=str, default="report.html")
    args = parser.parse_args()

    main(
        executable=Path(args.bin),
        report_html=Path(args.report_html),
    )
