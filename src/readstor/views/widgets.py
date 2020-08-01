import pathlib
from typing import Callable, List, Optional

import PySide2
import PySide2.QtGui
import PySide2.QtCore
import PySide2.QtWidgets

from readstor.config import config, errors

from . import mixins


class Logo:
    def __init__(self, size: Optional[int] = None) -> None:

        self._pixmap = PySide2.QtGui.QPixmap(str(config.app.LOGO))

        if size is not None:
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

        pixmap__idle = PySide2.QtGui.QPixmap(str(config.app.MENU_BAR_ICON_IDLE))
        pixmap__busy = PySide2.QtGui.QPixmap(str(config.app.MENU_BAR_ICON_BUSY))

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
        """ Creates a customizable "message" dialog.

        message -- Error message. """

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
        """ Creates a customizable "alert" dialog.

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
        https://doc.qt.io/qtforpython/PySide2/QtWidgets/QPushButton.html """

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

        self.setWindowTitle(f"About {config.app.NAME_PRETTY}")

        logo = Logo(size=96).widget()
        logo.setAlignment(PySide2.QtCore.Qt.AlignCenter)

        label__name = PySide2.QtWidgets.QLabel(config.app.NAME_PRETTY_VERSION)
        label__name.setAlignment(PySide2.QtCore.Qt.AlignCenter)

        label__copyright = PySide2.QtWidgets.QLabel(config.app.COPYRIGHT_INFO)
        label__copyright.setAlignment(PySide2.QtCore.Qt.AlignCenter)

        layout = PySide2.QtWidgets.QVBoxLayout()
        layout.setContentsMargins(32, 16, 32, 24)
        layout.setSpacing(0)
        layout.addWidget(logo)
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

        config.modified.connect(self._update_ui)

        self._init_ui()
        self._init_dialogs()
        self._update_ui()

    def _init_dialogs(self) -> None:

        self._bad_directory_error_dialog = ErrorDialog(
            message=f"Error setting {config.app.NAME_PRETTY} directory."
        )

    def _init_ui(self) -> None:

        self.setWindowTitle(f"{config.app.NAME_PRETTY} Preferences")

        label__stor_directory = PySide2.QtWidgets.QLabel()
        label__stor_directory.setText(f"{config.app.NAME_PRETTY} directory:")
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

        label__export_text = PySide2.QtWidgets.QLabel()
        label__export_text.setText("Export:")
        label__export_text.setAttribute(PySide2.QtCore.Qt.WA_MacSmallSize, True)

        self._checkbox__export_markdown = PySide2.QtWidgets.QCheckBox()
        self._checkbox__export_markdown.setText("Markdown")
        self._checkbox__export_markdown.clicked.connect(
            self._action__toggle_export_markdown
        )

        self._checkbox__export_text = PySide2.QtWidgets.QCheckBox("Text")
        self._checkbox__export_text.setText("Text")
        self._checkbox__export_text.clicked.connect(self._action__toggle_export_text)

        layout__bottom = PySide2.QtWidgets.QVBoxLayout()
        layout__bottom.setContentsMargins(0, 0, 0, 0)
        layout__bottom.setSpacing(0)
        layout__bottom.addWidget(label__export_text)
        layout__bottom.addSpacing(3)
        layout__bottom.addWidget(self._checkbox__export_markdown)
        layout__bottom.addSpacing(6)
        layout__bottom.addWidget(self._checkbox__export_text)

        #

        layout = PySide2.QtWidgets.QVBoxLayout()
        layout.setContentsMargins(20, 20, 20, 20)
        layout.setSpacing(0)
        layout.addLayout(layout__top)
        layout.addSpacing(6)
        layout.addLayout(layout__bottom)

        self.setLayout(layout)

    def _update_ui(self) -> None:
        self._lineedit__stor_directory.setText(str(config.user.path_stor))
        self._checkbox__export_markdown.setChecked(config.user.export_markdown)
        self._checkbox__export_text.setChecked(config.user.export_text)

    def _action__toggle_export_markdown(self) -> None:
        config.user.export_markdown = self._checkbox__export_markdown.isChecked()

    def _action__toggle_export_text(self) -> None:
        config.user.export_text = self._checkbox__export_text.isChecked()

    def _action__change_stor(self) -> None:

        dialog = SelectDirectoryDialog(directory=config.user.path_stor)

        if not dialog.exec_():
            return

        try:
            config.user.path_stor = dialog.selected_directory
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
