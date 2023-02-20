from configparser import ConfigParser
from cacophony.paths import USER_DIRECTORY, DATA_DIRECTORY


def get() -> ConfigParser:
    """
    :return: The config parser.
    """

    parser = ConfigParser()
    local_config_path = USER_DIRECTORY.joinpath("config.init")
    # Read a user-defined config file.
    if local_config_path.exists():
        parser.read(str(local_config_path))
    # Read the default config file.
    else:
        parser.read(str(DATA_DIRECTORY.joinpath("config.ini")))
    return parser
