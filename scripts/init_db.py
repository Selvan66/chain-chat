import subprocess
import sys

import docker


def check_if_command_exist(command):
    output = subprocess.getstatusoutput(f"{command} --version")
    if output[0] == 0:
        print(output[1])
        return True
    print("Command {commnad} doesn't exists")
    return False


def check_commands():
    """
    Check if docker, sqlx, mysql command exists
    """
    print("Checking commands:")
    commands = ["docker", "sqlx", "mysql"]
    return all(check_if_command_exist(c) for c in commands)


def create_mysql_container():
    print("Start creating MySQL container.")

    try:
        client = docker.from_env()
    except docker.errors.DockerException as e:
        print("Docker engine is not running.", e)
        sys.exit(1)

    container = client.containers.run(
        "mysql",
        detach=True,
        ports={"3306/tcp": "3306"},
        environment={
            "MYSQL_ROOT_PASSWORD": "password",
            "MYSQL_DATABASE": "mydb",
            "MYSQL_USER": "user",
            "MYSQL_PASSWORD": "password",
        },
    )

    print(f"MySQL container started successfully. Container ID: {container.short_id}")


def wait_until_mysql_start():
    pass


def main():
    check_commands()
    create_mysql_container()
    wait_until_mysql_start()


if __name__ == "__main__":
    main()
