; Inno Setup Script for PDF Viewer Commercial Edition
; Requires Inno Setup 6.0 or later: https://jrsoftware.org/isdl.php

#define MyAppName "DocLens Pro"
#define MyAppVersion "1.0.0"
#define MyAppPublisher "Your Company Name"
#define MyAppURL "https://www.yourcompany.com"
#define MyAppExeName "DocLensPro.exe"

[Setup]
AppId={{YOUR-GUID-HERE}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DefaultGroupName={#MyAppName}
AllowNoIcons=yes
LicenseFile=LICENSE_COMMERCIAL.txt
SetupIconFile=icon\icon.ico
UninstallDisplayIcon={app}\{#MyAppExeName}
OutputDir=dist
OutputBaseFilename={#MyAppName}_Setup_{#MyAppVersion}
Compression=lzma
SolidCompression=yes
WizardStyle=modern
PrivilegesRequired=admin
ArchitecturesInstallIn64BitMode=x64

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"
Name: "thai"; MessagesFile: "compiler:Languages\Thai.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked
Name: "quicklaunchicon"; Description: "{cm:CreateQuickLaunchIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked; OnlyBelowVersion: 6.1; Check: not IsAdminInstallMode

[Files]
Source: "dist\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "dist\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs
Source: "LICENSE_COMMERCIAL.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: "README_COMMERCIAL.md"; DestDir: "{app}"; Flags: ignoreversion isreadme
Source: "icon\icon.ico"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{group}\{cm:UninstallProgram,{#MyAppName}}"; Filename: "{uninstallexe}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon
Name: "{userappdata}\Microsoft\Internet Explorer\Quick Launch\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: quicklaunchicon

[Run]
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#StringChange(MyAppName, '&', '&&')}}"; Flags: nowait postinstall skipifsilent

[Code]
var
  LicenseKeyPage: TInputQueryWizardPage;
  EmailPage: TInputQueryWizardPage;

procedure InitializeWizard;
begin
  // Create license key input page
  LicenseKeyPage := CreateInputQueryPage(wpLicense,
    'License Activation', 'Enter your license information',
    'Please enter your license key. If you don''t have one, you can start a 30-day trial.');
  LicenseKeyPage.Add('License Key (leave empty for trial):', False);
  
  // Create email input page
  EmailPage := CreateInputQueryPage(LicenseKeyPage.ID,
    'Contact Information', 'Enter your email address',
    'Please enter your email address for license registration.');
  EmailPage.Add('Email Address:', False);
end;

function NextButtonClick(CurPageID: Integer): Boolean;
var
  LicenseKey: String;
  Email: String;
begin
  Result := True;
  
  if CurPageID = EmailPage.ID then
  begin
    LicenseKey := LicenseKeyPage.Values[0];
    Email := EmailPage.Values[0];
    
    // Validate email
    if (Email = '') or (Pos('@', Email) = 0) then
    begin
      MsgBox('Please enter a valid email address.', mbError, MB_OK);
      Result := False;
      Exit;
    end;
    
    // Save license info for post-install activation
    SaveStringToFile(ExpandConstant('{tmp}\license_key.txt'), LicenseKey, False);
    SaveStringToFile(ExpandConstant('{tmp}\email.txt'), Email, False);
  end;
end;

procedure CurStepChanged(CurStep: TSetupStep);
var
  ResultCode: Integer;
  LicenseKey: String;
  Email: String;
begin
  if CurStep = ssPostInstall then
  begin
    // Activate license after installation
    if LoadStringFromFile(ExpandConstant('{tmp}\license_key.txt'), LicenseKey) and
       LoadStringFromFile(ExpandConstant('{tmp}\email.txt'), Email) then
    begin
      if LicenseKey <> '' then
      begin
        // Activate with license key
        Exec(ExpandConstant('{app}\{#MyAppExeName}'), 
             '--activate "' + LicenseKey + '" "' + Email + '"',
             '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
      end
      else
      begin
        // Start trial
        Exec(ExpandConstant('{app}\{#MyAppExeName}'), 
             '--start-trial "' + Email + '"',
             '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
      end;
    end;
  end;
end;
