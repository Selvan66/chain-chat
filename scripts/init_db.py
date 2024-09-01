import subprocess
import os
import sys
import time

import docker
import mysql.connector


def check_if_command_exist(command):
    output = subprocess.getstatusoutput(f"{command} --version")
    if output[0] == 0:
        print(output[1])
        return True
    print("Command {commnad} doesn't exists")
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


def create_mysql_container():
    print("Start creating MySQL container")

    try:
        client = docker.from_env()
    except docker.errors.DockerException as e:
        print("Docker engine is not running", e)
        sys.exit(1)

    env = get_mysql_env()
    port = env["MYSQL_PORT"]
    try:
        container = client.containers.run(
            "mysql",
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
        print("Docker is probably running")
    else:
        print(
            f"MySQL container started successfully. Container ID: {container.short_id}"
        )


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
            print("MySQL is running")
            return True
        except mysql.connector.errors.OperationalError:
            print("Waiting for MySQL to start")
            time.sleep(2)

    return False


def run_sqlx_migration():
    print("Start sqlx migration")
    env = get_mysql_env()
    sqlx_env = os.environ
    sqlx_env["DATABASE_URL"]: env["DATABASE_URL"]

    result = subprocess.run(
        "sqlx database create", shell=True, check=False, env=sqlx_env
    )

    if result.returncode != 0:
        print("Sqlx cannot create database")
        return False

    result = subprocess.run("sqlx migrate run", shell=True, check=False, env=sqlx_env)

    if result.returncode != 0:
        print("Sqlx cannot run migrations")
        return False

    print("Migrations run successfully")
    return True


if __name__ == "__main__":
    check_commands()
    create_mysql_container()
    wait_until_mysql_start()
    run_sqlx_migration()
