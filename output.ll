; ModuleID = 'my_module'
source_filename = "my_module"

@formatStr = private constant [4 x i8] c"%d\0A\00"

define i32 @add(i32 %0, i32 %1) {
entry:
  %sum = add i32 %0, %1
  ret i32 %sum
}

define i32 @main() {
entry:
  %addtmp = call i32 @add(i32 10, i32 110)
  %printfTemp = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @formatStr, i32 0, i32 0), i32 %addtmp)
  ret i32 %printfTemp
}

declare i32 @printf(i8*, ...)
