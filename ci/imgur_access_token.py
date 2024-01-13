#!/usr/bin/env python3
import base64
import os

import argparse
from loguru import logger
from nacl import encoding, public
import requests


def encrypt_secret(public_key: str, secret_value: str) -> str:
    public_key_bytes = base64.b64decode(public_key)
    public_key = public.PublicKey(public_key_bytes, encoding.RawEncoder)
    sealed_box = public.SealedBox(public_key)
    encrypted = sealed_box.encrypt(secret_value.encode("utf-8"))
    encrypted_base64 = base64.b64encode(encrypted).decode("utf-8")
    return encrypted_base64


def get_github_public_key(
    request_timeout: int = 10,
):
    return requests.request(
        "GET",
        "https://api.github.com/repos/nero19960329/RustRayTracer/actions/secrets/public-key",
        headers={
            "Accept": "application/vnd.github+json",
            "Authroization": f"Bearer {os.environ['GITHUB_TOKEN']}",
            "X-GitHub-Api-Version": "2022-11-28",
        },
        timeout=request_timeout,
    )


def get_imgur_access_token(
    refresh_token: str,
    client_id: str,
    client_secret: str,
    requests_timeout: int = 10,
):
    resp = requests.request(
        "POST",
        "https://api.imgur.com/oauth2/token",
        data={
            "refresh_token": refresh_token,
            "client_id": client_id,
            "client_secret": client_secret,
            "grant_type": "refresh_token",
        },
        timeout=requests_timeout,
    )
    resp.raise_for_status()
    return resp.json()["access_token"]


def touch_imgur_access_token(
    access_token: str,
    requests_timeout: int = 10,
) -> bool:
    try:
        resp = requests.request(
            "GET",
            "https://api.imgur.com/3/account/me/settings",
            headers={
                "Authorization": f"Bearer {access_token}",
            },
            timeout=requests_timeout,
        )
        resp.raise_for_status()
        return True
    except requests.exceptions.HTTPError:
        logger.info(f"touch imgur access token failed: {resp.json()}")
        return False


def update_github_secret(
    secret_name: str,
    secret_value: str,
    request_timeout: int = 10,
):
    # get repo public key
    resp = get_github_public_key(request_timeout=request_timeout)
    resp.raise_for_status()
    key_id = resp.json()["key_id"]
    key = resp.json()["key"]

    return requests.request(
        "PUT",
        f"https://api.github.com/repos/nero19960329/RustRayTracer/actions/secrets/{secret_name}",
        headers={
            "Accept": "application/vnd.github+json",
            "Authroization": f"Bearer {os.environ['GITHUB_TOKEN']}",
            "X-GitHub-Api-Version": "2022-11-28",
        },
        json={
            "key_id": key_id,
            "encrypted_value": encrypt_secret(key, secret_value),
        },
        timeout=request_timeout,
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--gha", action="store_true")
    args = parser.parse_args()

    imgur_access_token = os.getenv("IMGUR_ACCESS_TOKEN")
    if not imgur_access_token or (
        imgur_access_token and not touch_imgur_access_token(imgur_access_token)
    ):
        logger.info("imgur access token expired, refreshing ...")
        imgur_access_token = get_imgur_access_token(
            refresh_token=os.environ["IMGUR_REFRESH_TOKEN"],
            client_id=os.environ["IMGUR_CLIENT_ID"],
            client_secret=os.environ["IMGUR_CLIENT_SECRET"],
        )
        logger.info("imgur access token refreshed")

        if not args.gha:
            logger.info(imgur_access_token)
        else:
            update_github_secret(
                "IMGUR_ACCESS_TOKEN",
                imgur_access_token,
            )
            logger.info("imgur access token updated")
    else:
        logger.info("imgur access token still valid")
