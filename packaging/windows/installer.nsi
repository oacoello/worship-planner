; NSIS Modern User Interface Installer for Granco Planner (Native Rust)
!include "MUI2.nsh"

Name "Granco Planner"
OutFile "..\..\release\Granco-Planner-Setup-1.0.0-x64.exe"
InstallDir "$PROGRAMFILES64\Granco Planner"
InstallDirRegKey HKLM "Software\GrancoPlanner" "Install_Dir"
RequestExecutionLevel admin

!define MUI_ABORTWARNING
!define MUI_ICON "${NSISDIR}\Contrib\Graphics\Icons\modern-install.ico"
!define MUI_UNICON "${NSISDIR}\Contrib\Graphics\Icons\modern-uninstall.ico"

; Pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!define MUI_FINISHPAGE_RUN "$INSTDIR\granco-planner.exe"
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

; Languages
!insertmacro MUI_LANGUAGE "Spanish"
!insertmacro MUI_LANGUAGE "English"

Section "Granco Planner" SecMain
  SetOutPath "$INSTDIR"
  File "..\..\target\x86_64-pc-windows-msvc\release\granco-planner.exe"
  File "..\..\assets\icon.ico"

  ; Store installation folder
  WriteRegStr HKLM "Software\GrancoPlanner" "Install_Dir" "$INSTDIR"

  ; Write Uninstaller keys for Add/Remove Programs
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\GrancoPlanner" "DisplayName" "Granco Planner"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\GrancoPlanner" "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\GrancoPlanner" "DisplayIcon" '"$INSTDIR\granco-planner.exe"'
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\GrancoPlanner" "Publisher" "Oscar Coello"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\GrancoPlanner" "DisplayVersion" "1.0.0"
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\GrancoPlanner" "NoModify" 1
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\GrancoPlanner" "NoRepair" 1
  WriteUninstaller "$INSTDIR\uninstall.exe"

  ; Create Start Menu & Desktop Shortcuts
  CreateDirectory "$SMPROGRAMS\Granco Planner"
  CreateShortCut "$SMPROGRAMS\Granco Planner\Granco Planner.lnk" "$INSTDIR\granco-planner.exe" "" "$INSTDIR\granco-planner.exe" 0
  CreateShortCut "$SMPROGRAMS\Granco Planner\Uninstall.lnk" "$INSTDIR\uninstall.exe"
  CreateShortCut "$DESKTOP\Granco Planner.lnk" "$INSTDIR\granco-planner.exe" "" "$INSTDIR\granco-planner.exe" 0
SectionEnd

Section "Uninstall"
  Delete "$INSTDIR\granco-planner.exe"
  Delete "$INSTDIR\icon.ico"
  Delete "$INSTDIR\uninstall.exe"

  Delete "$SMPROGRAMS\Granco Planner\Granco Planner.lnk"
  Delete "$SMPROGRAMS\Granco Planner\Uninstall.lnk"
  RMDir "$SMPROGRAMS\Granco Planner"
  Delete "$DESKTOP\Granco Planner.lnk"

  RMDir "$INSTDIR"

  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\GrancoPlanner"
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\GrancoPlanner"
SectionEnd
