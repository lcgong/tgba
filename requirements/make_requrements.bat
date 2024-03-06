tmp\python\python.exe -m venv tmp\venv
CALL tmp\venv\Scripts\activate.bat
python -m pip install pip -U 
python -m pip download -d tmp\cached -r requirements.txt
python -m pip install -r requirements.txt

python -m pip freeze > tmp\requirements-new.txt
