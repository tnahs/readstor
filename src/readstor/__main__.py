import argparse
from typing import List

from readstor.app import main


LOG_LEVELS: List[str] = ["DEBUG", "INFO", "WARNING", "ERROR", "CRITICAL"]
ENVIRONMENTS: List[str] = ["PRODUCTION", "DEVELOPMENT", "TESTING"]

parser = argparse.ArgumentParser(add_help=False)

# fmt:off
parser.add_argument(
    "-l",
    "--log_level",
    choices=LOG_LEVELS,
    default=LOG_LEVELS[2],
)
parser.add_argument(
    "-e",
    "--env",
    choices=ENVIRONMENTS,
    default=ENVIRONMENTS[0],
)
parser.add_argument(
    "-h",
    "--help",
    action="help",
    help="Show this help message.",
    default=argparse.SUPPRESS,
)
# fmt:on

args: argparse.Namespace = parser.parse_args()


if __name__ == "__main__":

    main(args=args)
