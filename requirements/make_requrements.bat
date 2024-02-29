tmp\python\python.exe -m venv venv
CALL tmp\venv\Scripts\activate.bat
python -m pip install -r ..\requirements.txt
python -m pip freeze > requirements.txt
