import argparse
import logging
import logging.config
import sys

import PySide2
import PySide2.QtCore
import PySide2.QtWidgets

from .config import GlobalConfig
from .views import views


logger = logging.getLogger(__name__)


def main(args: argparse.Namespace) -> None:

    GlobalConfig.env = args.env

    GlobalConfig.app.setup()
    GlobalConfig.app.init_logger(log_level=args.log_level)

    GlobalConfig.user.setup()
    GlobalConfig.user.load()

    logger.info(f"Running in {GlobalConfig.env} mode.")
    logger.info(f"Application root directory: `{GlobalConfig.app.PATH_HOME}`.")
    logger.info(f"User data directory: `{GlobalConfig.user.path_stor}`.")
    logger.info(
        f"Database directory: `{GlobalConfig.applebooks.PATH_SOURCE_DATABASES}`."
    )

    app = PySide2.QtWidgets.QApplication([])
    app.setQuitOnLastWindowClosed(False)

    view = views.MenuBarView()
    view.show()

    sys.exit(app.exec_())
