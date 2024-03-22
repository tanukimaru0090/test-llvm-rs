extern crate llvm_sys as llvm;
use llvm::{
    analysis::LLVMVerifyModule, core::*, initialization::LLVMInitializeIPA,
    object::LLVMDisposeBinary, prelude::*, target::*, LLVMBuilder, LLVMLinkage,
};
use std::{ffi::CString, fs::File, io::Write};
fn main() {
    unsafe {
        // LLVM初期化
        LLVM_InitializeAllTargetInfos();
        LLVM_InitializeAllTargets();
        LLVM_InitializeAllTargetMCs();
        LLVM_InitializeAllAsmParsers();
        LLVM_InitializeAllAsmPrinters();

        // 出力ファイル名
        let ir_file_name = "output.ll";

        // コンテキスト,ビルダー,モジュールを初期化
        let ctx = LLVMContextCreate();
        let builder = LLVMCreateBuilder();
        let module =
            LLVMModuleCreateWithNameInContext(CString::new("my_module").unwrap().as_ptr(), ctx);
        // 整数型のi32のLLVM Typeを作成
        let int_type = LLVMInt32TypeInContext(ctx);
        let int8_type = LLVMInt8TypeInContext(ctx);
        // add関数の型定義
        let add_func_type = LLVMFunctionType(int_type, [int_type, int_type].as_mut_ptr(), 2, 0);
        // add関数の定義
        let add_func =
            LLVMAddFunction(module, CString::new("add").unwrap().as_ptr(), add_func_type);
        // add関数のベーシックブロックを作成
        let add_entry =
            LLVMAppendBasicBlockInContext(ctx, add_func, CString::new("entry").unwrap().as_ptr());
        LLVMPositionBuilderAtEnd(builder, add_entry);

        // add関数の引数を取得
        let x = LLVMGetParam(add_func, 0);
        let y = LLVMGetParam(add_func, 1);
        // 足し算の命令を作成
        let sum = LLVMBuildAdd(builder, x, y, CString::new("sum").unwrap().as_ptr());
        // add関数からリターンする
        LLVMBuildRet(builder, sum);

        // main関数の型定義
        let main_type = LLVMFunctionType(int_type, std::ptr::null_mut(), 0, 0);
        // main関数の定義
        let main_func = LLVMAddFunction(module, CString::new("main").unwrap().as_ptr(), main_type);
        // main関数のベーシックブロックを作成
        let main_entry =
            LLVMAppendBasicBlockInContext(ctx, main_func, CString::new("entry").unwrap().as_ptr());
        LLVMPositionBuilderAtEnd(builder, main_entry);

        // printf関数の型定義
        let printf_type =
            LLVMFunctionType(int_type, [LLVMPointerType(int8_type, 0)].as_mut_ptr(), 1, 1);

        // printf関数の関数定義
        let printf_func = LLVMAddFunction(
            module,
            CString::new("printf").unwrap().as_ptr(),
            printf_type,
        );

        let format_str = CString::new("%d\n").unwrap();
        let global_format_str = LLVMAddGlobal(
            module,
            LLVMArrayType(int8_type, format_str.to_bytes_with_nul().len() as u32),
            CString::new("formatStr").unwrap().as_ptr(),
        );
        
        LLVMSetInitializer(
            global_format_str,
            LLVMConstStringInContext(
                ctx,
                format_str.as_ptr() as *const i8,
                format_str.to_bytes_with_nul().len() as u32 ,
                1,
            ),
        );
        LLVMSetGlobalConstant(global_format_str, 1);
        LLVMSetLinkage(global_format_str, LLVMLinkage::LLVMLinkerPrivateLinkage);

        // add関数を呼ぶための引数の指定
        let arg1 = LLVMConstInt(int_type, 10, 0);
        let arg2 = LLVMConstInt(int_type, 110, 0);
        // 引数を一つのタプルにする
        let mut call_args = [arg1, arg2];

        // add関数をmain関数で呼び出す
        let add_res = LLVMBuildCall(
            builder,
            add_func,
            call_args.as_mut_ptr(),
            2,
            CString::new("addtmp").unwrap().as_ptr(),
        );
        let zero = LLVMConstInt(int_type, 0, 0);
        let mut indices = [zero, zero];
        let format_str_ptr = LLVMBuildGEP(
            builder,
            global_format_str,
            indices.as_mut_ptr(),
            2,
            CString::new("formatStrPtr").unwrap().as_ptr(),
        );

        // printf関数を呼び出す
        let mut printf_args = [format_str_ptr, add_res];
        let printf_res = LLVMBuildCall(
            builder,
            printf_func,
            printf_args.as_mut_ptr(),
            2,
            CString::new("printfTemp").unwrap().as_ptr(),
        );
        // printf関数の戻り値をmain関数の戻り値としてリターンする
        LLVMBuildRet(builder, printf_res);

        // モジュールを検証
        LLVMVerifyModule(module, llvm::analysis::LLVMVerifierFailureAction::LLVMAbortProcessAction,std::ptr::null_mut());
        // IRを出力
        LLVMDumpModule(module);

        // 出力されたIRをファイルとして書き込む
        let ir = LLVMPrintModuleToString(module);
        let c_str = CString::from_raw(ir as *mut i8);
        let ir_string = c_str.to_str().unwrap();
        let mut file = File::create(ir_file_name).unwrap();
        file.write_all(ir_string.as_bytes()).unwrap();

        // 開放
        LLVMDisposeBuilder(builder);
        LLVMDisposeModule(module);
        LLVMContextDispose(ctx);
        
    }
    println!("Hello, world!");
}
