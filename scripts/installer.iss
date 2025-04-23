; installer.iss
[Setup]
AppName=DosDisassm
AppVersion=1.0
DefaultDirName={autopf}\DosDisassm
DefaultGroupName=DosDisassm
OutputDir=.
OutputBaseFilename=dosdisassm-setup
ArchitecturesInstallIn64BitMode=x64
DisableDirPage=no
DisableProgramGroupPage=no

[Files]
Source: "target\release\dosdisassm.exe"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\DosDisassm"; Filename: "{app}\dosdisassm.exe"
Name: "{commondesktop}\DosDisassm"; Filename: "{app}\dosdisassm.exe"; Tasks: desktopicon

[Tasks]
Name: "desktopicon"; Description: "Create a &desktop icon"; GroupDescription: "Additional icons:"
Name: "addtopath"; Description: "Add to &PATH environment variable"; GroupDescription: "Additional tasks:"

[Run]
Filename: "{app}\dosdisassm.exe"; Description: "Run DosDisassm"; Flags: nowait postinstall skipifsilent

[Code]
function AddToPath(Path: string): Boolean;
var
  EnvPath: string;
begin
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', EnvPath) then
    EnvPath := '';
  if Pos(Path, EnvPath) = 0 then begin
    EnvPath := EnvPath + ';' + Path;
    RegWriteStringValue(HKEY_LOCAL_MACHINE, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', EnvPath);
    Result := True;
  end else
    Result := False;
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  if CurStep = ssPostInstall then begin
    if WizardIsTaskSelected('addtopath') then begin
      AddToPath(ExpandConstant('{app}'));
    end;
  end;
end;
