from setuptools import setup, find_packages
from pathlib import Path
from platform import system

version = Path("cacophony/version.py").read_text().split("\n")[0].split("=")[1].strip()[1:-1]
install_requires = ['pygame==2.1.3', 'overrides==7.3.1', 'h5py==3.8.0', 'pydub==0.25.1', 'numpy==1.21.6',
                    'chipnumpy==1.0.0', 'pythonnet==3.0.1', 'requests==2.28.2', 'bs4==0.0.1', 'sf2_loader==1.19',
                    'cython-vst-loader==0.3.6', 'pyttsx3==2.90']
if system() == "Darwin":
    install_requires.append("pyobjc")
setup(
    name='cacophony',
    version=version,
    description='ASCII DAW',
    long_description=Path("README.md").read_text(),
    long_description_content_type='text/markdown',
    url='https://github.com/subalterngames/cacophony',
    author_email='subalterngames@gmail.com',
    author='Esther Alter',
    license='MIT',
    classifiers=[
        'Development Status :: 2 - Pre-Alpha',
        'Intended Audience :: Developers',
        'Topic :: Software Development',
        'License :: OSI Approved :: BSD License',
        'Programming Language :: Python :: 3.6',
        'Programming Language :: Python :: 3.7',
        'Programming Language :: Python :: 3.8'
    ],
    packages=find_packages(),
    include_package_data=True,
    keywords='music audio signal daw vst sf2 sf3 composing',
    install_requires=install_requires,
)
