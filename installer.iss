#define MyAppName "DosDisassm"
#define MyAppExeName "dosdisassm.exe"
#define MyAppVersion "0.1.0"
#define MyAppPublisher "sk337"
#define MyLicenseFile "LICENSE"
#define MyIconFile "assets\\icon.ico"

[Setup]
AppId={{A9F1629D-23BA-46E1-9390-13DA3F4C14F2}} ; Unique GUID
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
DefaultDirName={code:GetInstallDir}
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=no
OutputDir=.
OutputBaseFilename=dosdisassm-setup
SetupIconFile={#MyIconFile}
Compression=lzma
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64
PrivilegesRequiredOverridesAllowed=dialog

LicenseFile={#MyLicenseFile}

[Files]
Source: "target\\release\\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\\{#MyAppName}"; Filename: "{app}\\{#MyAppExeName}"; IconFilename: "{app}\\{#MyAppExeName}"
Name: "{commondesktop}\\{#MyAppName}"; Filename: "{app}\\{#MyAppExeName}"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop icon"; GroupDescription: "Additional icons:"
Name: "addtopath"; Description: "Add to &PATH"; GroupDescription: "Environment Options"

[Run]
Filename: "{app}\\{#MyAppExeName}"; Description: "Run {#MyAppName}"; Flags: nowait postinstall skipifsilent

[Code]
function GetInstallDir(Default: String): String;
var
  IsAdmin: Boolean;
begin
  IsAdmin := IsActuallyAdmin;
  if IsAdmin then
    Result := ExpandConstant('{pf64}\\{#MyAppName}')
  else
    Result := ExpandConstant('{userappdata}\\{#MyAppName}');
end;

function AddToPath(Path: string; HKRoot: Integer): Boolean;
var
  EnvPath: string;
begin
  if not RegQueryStringValue(HKRoot, 'Environment', 'Path', EnvPath) then
    EnvPath := '';
  if Pos(Path, EnvPath) = 0 then begin
    if Copy(EnvPath, Length(EnvPath), 1) <> ';' then
      EnvPath := EnvPath + ';';
    EnvPath := EnvPath + Path;
    RegWriteStringValue(HKRoot, 'Environment', 'Path', EnvPath);
    Result := True;
  end else
    Result := False;
end;

procedure CurStepChanged(CurStep: TSetupStep);
var
  RootKey: Integer;
begin
  if CurStep = ssPostInstall then begin
    if WizardIsTaskSelected('addtopath') then begin
      if IsActuallyAdmin then
        RootKey := HKEY_LOCAL_MACHINE
      else
        RootKey := HKEY_CURRENT_USER;
      AddToPath(ExpandConstant('{app}'), RootKey);
    end;
  end;
end;

function IsActuallyAdmin: Boolean;
begin
  Result := (GetShellFolder('commonappdata') <> '');
end;
