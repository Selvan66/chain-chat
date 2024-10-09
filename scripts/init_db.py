import subprocess
import os
import sys
import time

import docker
import mysql.connector

MYSQL_CONTAINER_NAME = "mysql_chain_chat"


def check_if_command_exist(command):
    output = subprocess.getstatusoutput(f"{command} --version")
    if output[0] == 0:
        print("\t", output[1], sep="")
        return True
    print("\tCommand {commnad} doesn't exists")
    return False


def check_commands():
    print("Checking commands:")
    commands = ["docker", "sqlx"]
    return all(check_if_command_exist(c) for c in commands)


def get_mysql_env():
    env = {
        "MYSQL_ROOT_PASSWORD": os.getenv("MYSQL_ROOT_PASSWORD", "password"),
        "MYSQL_DATABASE": os.getenv("MYSQL_DATABASE", "mydb"),
        "MYSQL_USER": os.getenv("MYSQL_USER", "user"),
        "MYSQL_PASSWORD": os.getenv("MYSQL_PASSWORD", "password"),
        "MYSQL_PORT": os.getenv("MYSQL_PORT", "3306"),
        "MYSQL_HOST": os.getenv("MYSQL_HOST", "localhost"),
        "DATABASE_URL": os.getenv("DATABASE_URL", ""),
    }

    env["DATABASE_URL"] = os.getenv(
        "DATABASE_URL",
        "mysql://{}:{}@{}:{}/{}".format(
            env["MYSQL_USER"],
            env["MYSQL_PASSWORD"],
            env["MYSQL_HOST"],
            env["MYSQL_PORT"],
            env["MYSQL_DATABASE"],
        ),
    )

    return env


def check_if_mysql_container_exists(docker_client):
    return docker_client.containers.list(
        all=True, filters={"name": MYSQL_CONTAINER_NAME}
    )


def create_mysql_container(docker_client):
    print("\tStart creating MySQL container")

    env = get_mysql_env()
    port = env["MYSQL_PORT"]
    try:
        container = docker_client.containers.run(
            "mysql:8.0.39-bookworm",
            name=MYSQL_CONTAINER_NAME,
            detach=True,
            ports={f"{port}/tcp": f"{port}"},
            environment={
                "MYSQL_ROOT_PASSWORD": env["MYSQL_ROOT_PASSWORD"],
                "MYSQL_DATABASE": env["MYSQL_DATABASE"],
                "MYSQL_USER": env["MYSQL_USER"],
                "MYSQL_PASSWORD": env["MYSQL_PASSWORD"],
            },
        )
    except docker.errors.APIError:
        print("\tDocker is probably running")
    else:
        print(
            f"\tMySQL container created successfully. Container ID: {container.short_id}"
        )


def run_mysql_container():
    print("Run mysql container")

    try:
        client = docker.from_env()
    except docker.errors.DockerException as e:
        print("\tDocker engine is not running", e)
        sys.exit(1)

    if check_if_mysql_container_exists(client):
        print("\tContainer exists")
        container = client.containers.get(MYSQL_CONTAINER_NAME)
        container.start()
        print(f"\tContainer found. Container ID: {container.short_id}")
    else:
        create_mysql_container(client)

    print("\tMySQL container started successfully.")


def wait_until_mysql_start():
    print("Checking if mysql is running")
    env = get_mysql_env()

    while True:
        try:
            connection = mysql.connector.connect(
                user=env["MYSQL_USER"],
                password=env["MYSQL_PASSWORD"],
                host=env["MYSQL_HOST"],
                database=env["MYSQL_DATABASE"],
            )
            connection.close()
            print("\tMySQL is running")
            return True
        except mysql.connector.errors.OperationalError:
            print("\tWaiting for MySQL to start")
            time.sleep(2)

    return False


def run_sqlx_migration():
    print("Start sqlx migration")
    env = get_mysql_env()

    result = subprocess.run(
        f"sqlx database create --database_url {env["DATABASE_URL"]}",
        shell=True,
        check=False,
        capture_output=True,
        text=True,
    )

    if result.returncode != 0:
        print(f"\tSqlx cannot create database\n\t Error: {result.stderr}")
        return False

    print("\n".join([f"\t{line}" for line in result.stdout.split("\n")]))

    result = subprocess.run(
        f"sqlx migrate run --database_url {env["DATABASE_URL"]}",
        shell=True,
        check=False,
        capture_output=True,
        text=True,
    )

    if result.returncode != 0:
        print(f"\tSqlx cannot run migrations\n\t Error: {result.stderr}")
        return False

    print("\n".join([f"\t{line}" for line in result.stdout.split("\n")]))

    print("\tMigrations run successfully")
    return True


def main():
    check_commands()
    run_mysql_container()
    wait_until_mysql_start()
    run_sqlx_migration()


if __name__ == "__main__":
    main()
