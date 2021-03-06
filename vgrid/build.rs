// Copyright © Jordan Singh 2022

use winres::WindowsResource;

fn main() {
    let mut res = WindowsResource::new();
    res.set_icon_with_id("vgrid.ico", "vgrid");
    res.set("CompanyName", "Jordan Singh");
    res.set("FileDescription", "VGrid Binary");
    res.set("FileVersion", "0.2.0.0");
    res.set("InternalName", "VGrid");
    res.set("LegalCopyright", "Copyright © Jordan Singh 2022");
    res.set("LegalTrademarks1", "Do not distribute.");
    res.set("OriginalFilename", "vgrid.exe");
    res.set("ProductName", "VGrid");
    res.set("ProductVersion", "0.2.0.0");
    res.set_version_info(winres::VersionInfo::PRODUCTVERSION, 0x0000000200000000);
    res.set_version_info(winres::VersionInfo::FILEVERSION, 0x0000000200000000);

    res.set_manifest(r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0" xmlns:asmv3="urn:schemas-microsoft-com:asm.v3">
	<compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">
		<application>
            <!-- Windows 10 and Windows 11 -->
			<supportedOS Id="{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}"/>
			<!-- Windows 8.1 -->
			<supportedOS Id="{1f676c76-80e1-4239-95bb-83d0f6d0da78}"/>
			<!-- Windows 8 -->
			<supportedOS Id="{4a2f28e3-53b9-4441-ba9c-d69d4a4a6e38}"/>
			<!-- Windows 7 -->
			<supportedOS Id="{35138b9a-5d96-4fbd-8e2d-a2440225f93a}"/>
		</application>
	</compatibility>
	<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
		<security>
			<requestedPrivileges>
				<requestedExecutionLevel level="requireAdministrator" uiAccess="false"></requestedExecutionLevel>
			</requestedPrivileges>
		</security>
	</trustInfo>
	<asmv3:application>
		<asmv3:windowsSettings>
			<dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true/pm</dpiAware>
			<dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2</dpiAwareness>
		</asmv3:windowsSettings>
	</asmv3:application>
</assembly>
"#);
    res.compile().unwrap();
}