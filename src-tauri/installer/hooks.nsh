; Opens the blocked addresses before the program goes away. The uninstaller already has
; the administrator's permission, so nothing is asked.
!macro NSIS_HOOK_PREUNINSTALL
  ${If} $UpdateMode <> 1
    ExecWait '"$INSTDIR\${MAINBINARYNAME}.exe" --clear'
  ${EndIf}
!macroend
