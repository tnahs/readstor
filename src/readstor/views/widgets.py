import pathlib
from typing import Callable, List, Optional

import PySide2
import PySide2.QtCore
import PySide2.QtGui
import PySide2.QtWidgets

from readstor import errors
from readstor.config import GlobalConfig

from . import mixins


class Logo:
    def __init__(self, size: Optional[int] = None) -> None:

        self._pixmap = PySide2.QtGui.QPixmap(str(GlobalConfig.app.LOGO))

        # Enables HiDPI images to scale properly.
        self._pixmap.setDevicePixelRatio(GlobalConfig.app.pixel_ratio)

        if size is not None:

            # Normalizes the size so that all pixel values passed to Qt's
            # public API follow the same convention. i.e. When adding spacing,
            # a value of '5' passed to <layout>.addSpacing() yields a spacing
            # of 10 if the pixel ratio is 2.
            size = size * GlobalConfig.app.pixel_ratio

            self._pixmap = self._pixmap.scaled(
                size,
                size,
                PySide2.QtCore.Qt.KeepAspectRatio,
                PySide2.QtCore.Qt.SmoothTransformation,
            )

        self._icon = PySide2.QtGui.QIcon(self._pixmap)

        self._widget = PySide2.QtWidgets.QLabel()
        self._widget.setPixmap(self._pixmap)

    def icon(self) -> PySide2.QtGui.QIcon:
        return self._icon

    def pixmap(self) -> PySide2.QtGui.QPixmap:
        return self._pixmap

    def widget(self) -> PySide2.QtWidgets.QLabel:
        return self._widget


class MenuBarIcon:
    def __init__(self) -> None:

        pixmap__idle = PySide2.QtGui.QPixmap(str(GlobalConfig.app.MENUBAR_ICON_IDLE))
        pixmap__busy = PySide2.QtGui.QPixmap(str(GlobalConfig.app.MENUBAR_ICON_BUSY))

        self._icon_idle = PySide2.QtGui.QIcon(pixmap__idle)
        self._icon_busy = PySide2.QtGui.QIcon(pixmap__busy)

    @property
    def idle(self) -> PySide2.QtGui.QIcon:
        return self._icon_idle

    @property
    def busy(self) -> PySide2.QtGui.QIcon:
        return self._icon_busy


class MenuItem(PySide2.QtWidgets.QAction):
    def __init__(
        self,
        label: str,
        actions: Optional[List[Callable]] = None,
        parent=None,
        **kwargs,
    ) -> None:
        super().__init__(parent=parent)

        self.setText(label)

        if actions is not None:
            for action in actions:
                self.triggered.connect(action)

        enabled = kwargs.get("enabled", None)
        shortcut = kwargs.get("shortcut", None)
        tooltip = kwargs.get("tooltip", None)
        visible = kwargs.get("visible", None)

        if enabled is not None:
            self.setEnabled(enabled)

        if shortcut is not None:
            self.setShortcut(shortcut)

        if tooltip is not None:
            self.setToolTip(tooltip)

        if visible is not None:
            self.setVisible(visible)


class SelectDirectoryDialog(PySide2.QtWidgets.QFileDialog):
    def __init__(self, directory: pathlib.Path) -> None:
        super().__init__(parent=None)

        self.setFileMode(PySide2.QtWidgets.QFileDialog.Directory)
        self.setOption(PySide2.QtWidgets.QFileDialog.DontResolveSymlinks)

        self._directory = directory

        self.setDirectory(str(self._directory))

    @property
    def selected_directory(self) -> pathlib.Path:
        try:
            return pathlib.Path(self.selectedFiles()[0])
        except IndexError:
            return self._directory


class ErrorDialog(PySide2.QtWidgets.QMessageBox, mixins.QtShowMixin):
    def __init__(self, message: str) -> None:
        """Creates a customizable "message" dialog.

        message -- Error message."""

        super().__init__(parent=None)

        logo = Logo(size=64).pixmap()

        self.setStandardButtons(PySide2.QtWidgets.QMessageBox.Ok)
        self.setDefaultButton(PySide2.QtWidgets.QMessageBox.Ok)
        self.setWindowFlags(
            PySide2.QtCore.Qt.CustomizeWindowHint
            | PySide2.QtCore.Qt.WindowTitleHint
            | PySide2.QtCore.Qt.WindowStaysOnTopHint
        )

        self.setIconPixmap(logo)
        self.setText(message)

    def set_details(self, details: str) -> None:
        self.setInformativeText(details)

    def show(self) -> None:

        if self.isVisible():
            self.raise_()
            self.raise_()
            return

        super().show()
        self.center_window(offset_y=0.35)


class AlertDialog(PySide2.QtWidgets.QMessageBox, mixins.QtShowMixin):
    def __init__(
        self,
        message: str,
        details: Optional[str] = None,
        action_button: Optional[dict] = None,
    ) -> None:
        """Creates a customizable "alert" dialog.

        message -- Message to alert.
        details -- Optional message details.
        action_button -- Optional list of buttons to append to dialog. Each
            button must be passed in as a dictionary with three key:value
            pairs.

            label: str -- The button's label.
            action: Callable -- The buttons action.
            default: bool -- Set as dialog's default button. If more than one
                button is marked as default, the last one in the list will be
                marked as default. (The default button in a dialog can
                generally be "clicked" using the `Enter` or `Return` key.)

        https://doc.qt.io/qtforpython/PySide2/QtWidgets/QMessageBox.html
        https://doc.qt.io/qtforpython/PySide2/QtWidgets/QPushButton.html"""

        super().__init__(parent=None)

        logo = Logo(size=64).pixmap()

        self.setStandardButtons(PySide2.QtWidgets.QMessageBox.Cancel)
        self.setDefaultButton(PySide2.QtWidgets.QMessageBox.Cancel)
        self.setWindowFlags(
            PySide2.QtCore.Qt.CustomizeWindowHint
            | PySide2.QtCore.Qt.WindowTitleHint
            | PySide2.QtCore.Qt.WindowStaysOnTopHint
        )

        self.setIconPixmap(logo)
        self.setText(message)

        if details is not None:
            self.setInformativeText(details)

        if action_button is not None:
            self._setup_action_button(action_button=action_button)

    def _setup_action_button(self, action_button: dict) -> None:

        label: Optional[str] = action_button.get("label", None)
        action: Optional[Callable] = action_button.get("action", None)
        default: bool = action_button.get("default", False)

        if label is None or action is None:
            return

        button = self.addButton(label, PySide2.QtWidgets.QMessageBox.AcceptRole)
        button.clicked.connect(action)

        if default is True:
            self.setDefaultButton(button)

    def show(self) -> None:

        if self.isVisible():
            self.raise_()
            self.raise_()
            return

        super().show()
        self.center_window(offset_y=0.35)


class AboutDialog(PySide2.QtWidgets.QDialog, mixins.QtShowMixin):
    def __init__(self) -> None:
        super().__init__(parent=None)

        self._init_ui()

    def _init_ui(self) -> None:

        self.setWindowTitle(f"About {GlobalConfig.app.NAME_PRETTY}")

        logo = Logo(size=96).widget()
        logo.setAlignment(PySide2.QtCore.Qt.AlignCenter)

        label__name = PySide2.QtWidgets.QLabel(GlobalConfig.app.NAME_PRETTY_VERSION)
        label__name.setAlignment(PySide2.QtCore.Qt.AlignCenter)

        label__copyright = PySide2.QtWidgets.QLabel(GlobalConfig.app.COPYRIGHT_INFO)
        label__copyright.setAlignment(PySide2.QtCore.Qt.AlignCenter)

        layout = PySide2.QtWidgets.QVBoxLayout()
        layout.setContentsMargins(32, 16, 32, 24)
        layout.setSpacing(0)
        layout.addWidget(logo)
        layout.addSpacing(5)
        layout.addWidget(label__name)
        layout.addSpacing(20)
        layout.addWidget(label__copyright)

        self.setLayout(layout)

    def show(self) -> None:

        if self.isVisible():
            self.raise_()
            self.raise_()
            return

        super().show()
        self.center_window(offset_y=0.35)
        self.setFixedSize(self.size())


class PreferencesDialog(PySide2.QtWidgets.QDialog, mixins.QtShowMixin):
    def __init__(self) -> None:
        super().__init__(parent=None)

        GlobalConfig.modified.connect(self._update_ui)

        self._init_ui()
        self._init_dialogs()
        self._update_ui()

    def _init_dialogs(self) -> None:

        self._bad_directory_error_dialog = ErrorDialog(
            message=f"Error setting {GlobalConfig.app.NAME_PRETTY} directory."
        )

    def _init_ui(self) -> None:

        self.setWindowTitle(f"{GlobalConfig.app.NAME_PRETTY} Preferences")

        label__stor_directory = PySide2.QtWidgets.QLabel()
        label__stor_directory.setText(f"{GlobalConfig.app.NAME_PRETTY} directory:")
        label__stor_directory.setAttribute(PySide2.QtCore.Qt.WA_MacSmallSize, True)

        self._lineedit__stor_directory = PySide2.QtWidgets.QLineEdit()
        self._lineedit__stor_directory.setFixedWidth(256)
        self._lineedit__stor_directory.setReadOnly(True)

        button__change_stor = PySide2.QtWidgets.QPushButton()
        button__change_stor.setText("...")
        button__change_stor.clicked.connect(self._action__change_stor)

        layout__change_stor = PySide2.QtWidgets.QHBoxLayout()
        layout__change_stor.setContentsMargins(0, 0, 0, 0)
        layout__change_stor.setSpacing(3)
        layout__change_stor.addWidget(self._lineedit__stor_directory)
        layout__change_stor.addWidget(button__change_stor)

        layout__top = PySide2.QtWidgets.QVBoxLayout()
        layout__top.setContentsMargins(0, 0, 0, 0)
        layout__top.setSpacing(0)
        layout__top.addWidget(label__stor_directory)
        layout__top.addLayout(layout__change_stor)

        #

        label__export = PySide2.QtWidgets.QLabel()
        label__export.setText("Export on Stor:")
        label__export.setAttribute(PySide2.QtCore.Qt.WA_MacSmallSize, True)

        self._checkbox__export_flat = PySide2.QtWidgets.QCheckBox()
        self._checkbox__export_flat.setText("Flat")
        self._checkbox__export_flat.clicked.connect(self._action__toggle_export_flat)

        # fmt:off
        self._checkbox__export_nested = PySide2.QtWidgets.QCheckBox("Nested")
        self._checkbox__export_nested.setText("Nested")
        self._checkbox__export_nested.clicked.connect(self._action__toggle_export_nested)
        # fmt:on

        layout__bottom = PySide2.QtWidgets.QVBoxLayout()
        layout__bottom.setContentsMargins(0, 0, 0, 0)
        layout__bottom.setSpacing(0)
        layout__bottom.addWidget(label__export)
        layout__bottom.addSpacing(3)
        layout__bottom.addWidget(self._checkbox__export_flat)
        layout__bottom.addSpacing(6)
        layout__bottom.addWidget(self._checkbox__export_nested)

        #

        layout = PySide2.QtWidgets.QVBoxLayout()
        layout.setContentsMargins(20, 20, 20, 20)
        layout.setSpacing(0)
        layout.addLayout(layout__top)
        layout.addSpacing(6)
        layout.addLayout(layout__bottom)

        self.setLayout(layout)

    def _update_ui(self) -> None:
        self._lineedit__stor_directory.setText(str(GlobalConfig.user.path_stor))
        self._checkbox__export_flat.setChecked(GlobalConfig.user.export_flat)
        self._checkbox__export_nested.setChecked(GlobalConfig.user.export_nested)

    def _action__toggle_export_flat(self) -> None:
        GlobalConfig.user.export_flat = self._checkbox__export_flat.isChecked()

    def _action__toggle_export_nested(self) -> None:
        GlobalConfig.user.export_nested = self._checkbox__export_nested.isChecked()

    def _action__change_stor(self) -> None:

        dialog = SelectDirectoryDialog(directory=GlobalConfig.user.path_stor)

        if not dialog.exec_():
            return

        try:
            GlobalConfig.user.path_stor = dialog.selected_directory
        except errors.ConfigurationError as error:
            self._bad_directory_error_dialog.set_details(details=str(error))
            self._bad_directory_error_dialog.show()

    def show(self) -> None:

        if self.isVisible():
            self.raise_()
            self.raise_()
            return

        super().show()
        self.center_window(offset_y=0.35)
        self.setFixedSize(self.size())
