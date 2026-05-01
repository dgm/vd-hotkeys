#define AppName "vd-hotkeys"
#ifndef AppVersion
  #define AppVersion "0.1.1"
#endif
#define AppPublisher "David Morton"
#define AppExeName "vd-hotkeys.exe"

[Setup]
AppId={{9172829A-B876-45C0-8DE3-5EEEACF7712C}
AppName={#AppName}
AppVersion={#AppVersion}
AppPublisher={#AppPublisher}
AppPublisherURL=https://github.com/dmorton/vd-hotkeys
DefaultDirName={localappdata}\{#AppName}
DisableDirPage=yes
DefaultGroupName={#AppName}
DisableProgramGroupPage=yes
OutputDir=dist
OutputBaseFilename=vd-hotkeys-v{#AppVersion}-setup
Compression=lzma
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
Source: "target\x86_64-pc-windows-msvc\release\{#AppExeName}"; DestDir: "{app}"; Flags: ignoreversion

[Registry]
; Run at login for the current user
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; \
  ValueType: string; ValueName: "{#AppName}"; \
  ValueData: """{app}\{#AppExeName}"""; \
  Flags: uninsdeletevalue

[Run]
Filename: "{app}\{#AppExeName}"; Description: "Start {#AppName} now"; \
  Flags: nowait postinstall skipifsilent

[UninstallRun]
Filename: "taskkill.exe"; Parameters: "/f /im {#AppExeName}"; \
  Flags: runhidden; RunOnceId: "StopApp"
