@echo off
echo ==========================================
echo Compilation de Saesth et Generation MSI
echo ==========================================

echo [1/5] Nettoyage...
if exist dist rd /s /q dist
if exist heat-dist.wxs del heat-dist.wxs
if exist heat-sounds.wxs del heat-sounds.wxs
if exist *.wixobj del *.wixobj
if exist saesth-installer.msi del saesth-installer.msi

echo [2/5] Build de l'application (Frontend + Backend)...
call npm run build
if %ERRORLEVEL% neq 0 (
    echo Erreur lors du build frontend.
    exit /b %ERRORLEVEL%
)

if not exist src-tauri\target\release\saesth.exe (
    echo Erreur : saesth.exe absent. Veuillez lancer 'npm run tauri build' manuellement d'abord.
    exit /b 1
)

echo [3/5] Recolte des fichiers (Harvesting)...
rem Recolte du dossier dist
heat dir dist -dr DistDir -cg DistFilesGroup -srd -gg -sfrag -suid -win64 -o heat-dist.wxs
if %ERRORLEVEL% neq 0 (
    echo Erreur lors du heat pour dist.
    exit /b %ERRORLEVEL%
)

rem Recolte du dossier sounds
heat dir src-tauri\sounds -dr SoundsDir -cg SoundsFilesGroup -srd -gg -sfrag -suid -win64 -o heat-sounds.wxs
if %ERRORLEVEL% neq 0 (
    echo Erreur lors du heat pour sounds.
    exit /b %ERRORLEVEL%
)

echo [4/5] Compilation WiX (Candle)...
candle -arch x64 msi-installer.wxs heat-dist.wxs heat-sounds.wxs
if %ERRORLEVEL% neq 0 (
    echo Erreur lors de la compilation WiX.
    exit /b %ERRORLEVEL%
)

echo [5/5] Liaison WiX (Light)...
light -ext WixUIExtension -sice:ICE43 -sice:ICE57 msi-installer.wixobj heat-dist.wixobj heat-sounds.wixobj -b dist -b src-tauri\sounds -o saesth-installer.msi
if %ERRORLEVEL% neq 0 (
    echo Erreur lors de la liaison WiX.
    exit /b %ERRORLEVEL%
)

echo ==========================================
echo Termine ! Votre installateur est : saesth-installer.msi
echo ==========================================
