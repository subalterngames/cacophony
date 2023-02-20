from cacophony.render.ui_element.options import Options


class I127(Options):
    """
    A range of options from 0 to 127.
    """

    def __init__(self, title: str, index: int):
        """
        :param title: The title.
        :param index: The index.
        """

        values = list(range(128))
        super().__init__(title=title, options=[str(v) for v in values], index=index)
