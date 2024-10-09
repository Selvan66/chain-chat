import subprocess
import os
import sys

import docker

REDIS_CONTAINER_NAME = "redis_chain_chat"


def check_if_command_exist(command):
    output = subprocess.getstatusoutput(f"{command} --version")
    if output[0] == 0:
        print("\t", output[1], sep="")
        return True
    print("\tCommand {commnad} doesn't exists")
    return False


def check_commands():
    print("Checking commands:")
    commands = ["docker"]
    return all(check_if_command_exist(c) for c in commands)


def get_redis_env():
    env = {"REDIS_PORT": os.getenv("REDIS_PORT", "6379")}
    return env


def check_if_redis_container_exists(docker_client):
    return docker_client.containers.list(
        all=True, filters={"name": REDIS_CONTAINER_NAME}
    )


def create_redis_container(docker_client):
    print("\tStart creating Redis container")

    env = get_redis_env()
    port = env["REDIS_PORT"]
    try:
        container = docker_client.containers.run(
            "redis:7.4.1-bookworm",
            name=REDIS_CONTAINER_NAME,
            detach=True,
            ports={f"{port}/tcp": f"{port}"},
        )
    except docker.errors.APIError as e:
        print("\tDocker is probably running\n", e)
    else:
        print(
            f"\tRedis container created successfully. Container ID: {container.short_id}"
        )


def run_redis_container():
    print("Run redis container")

    try:
        client = docker.from_env()
    except docker.errors.DockerException as e:
        print("\tDocker engine is not running", e)
        sys.exit(1)

    if check_if_redis_container_exists(client):
        print("\tContainer exists")
        container = client.containers.get(REDIS_CONTAINER_NAME)
        container.start()
        print(f"\tContainer found. Container ID: {container.short_id}")
    else:
        create_redis_container(client)

    print("\tRedis container started successfully.")


def main():
    check_commands()
    run_redis_container()


if __name__ == "__main__":
    main()
