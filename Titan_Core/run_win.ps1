Start-Process go -ArgumentList "run go\main.go" -NoNewWindow
Start-Process python -ArgumentList "vision\brain.py" -NoNewWindow
New-Item -ItemType Directory -Force -Path "jobs" | Out-Null
echo "TITAN SYSTEM ACTIVE."
echo "Drop text files into 'jobs' folder to trigger."
