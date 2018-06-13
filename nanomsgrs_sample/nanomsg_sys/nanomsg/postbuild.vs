if not exist "$(OutDir)../../../../target/$(Configuration)/deps" mkdir "$(OutDir)../../../../target/$(Configuration)/deps"
if exist "$(OutDir)$(TargetName)$(TargetExt)" copy "$(OutDir)$(TargetName)$(TargetExt)" "$(OutDir)../../../../target/$(Configuration)/deps"
if exist "$(OutDir)$(TargetName).pdb" copy "$(OutDir)$(TargetName).pdb" "$(OutDir)../../../../target/$(Configuration)/deps"