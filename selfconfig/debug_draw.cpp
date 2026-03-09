#include "dllmain.h"
#include <iostream>

//freecam notes
//328h -> 24Ch?
// rsi+90h -> 58h
//[[[HgMan]+6F8h]+254h] <-- DbgCamera
//[[[HgMan]+6F8h]+18h]  <-- FrpgPersCam (1st)
//[[[HgMan]+6F8h]+30h]  <-- FrpgPersCam (2nd)
// MoveMapStep+44h <-- freecam mode [rsi+70h in DSR]
//put function at F00AD4h

tDirectInput8Create oDirectInput8Create;
LPVOID cam_address = nullptr;
DWORD pHgMan = 0x137C580;
DWORD fAddDrawPlanBeginTargetScene = 0x4070F0;
DWORD fAddDrawPlanEndTargetScene = 0x407180;
DWORD fAddDrawPlanEndTargetCamera = 0x407080;

DWORD GetProcId(const wchar_t* procname);

using namespace std;

double PCFreq = 0.0;
__int64 CounterStart = 0;
void StartCounter()
{
    LARGE_INTEGER li;
    if (!QueryPerformanceFrequency(&li))
        cout << "QueryPerformanceFrequency failed!\n";

    PCFreq = double(li.QuadPart) / 1000.0;

    QueryPerformanceCounter(&li);
    CounterStart = li.QuadPart;
}
void GetCounter()
{
    LARGE_INTEGER li;
    QueryPerformanceCounter(&li);
    cout << double(li.QuadPart - CounterStart) / PCFreq << "\n";
}

bool Hook(void* toHook, void* ourFunc, int len)
{
    if (len < 5)
    {
        return false;
    }

    DWORD curProtec;
    VirtualProtect(toHook, len, PAGE_EXECUTE_READWRITE, &curProtec);

    memset(toHook, 0x90, len);

    DWORD relativeAddress = ((DWORD)ourFunc - (DWORD)toHook) - 5;

    *(BYTE*)toHook = 0xE9;
    *(DWORD*)((DWORD)toHook + 1) = relativeAddress;

    DWORD temp;
    VirtualProtect(toHook, len, curProtec, &temp);
    
    return true;
}

__declspec(naked) void FFXTimeMeasure()
{
    __asm
    {
        push ecx
        pushad
        sub esp, 28h
        call StartCounter
        add esp, 28h
        popad
        movss xmm0, dword ptr [esp+8]
        push ecx
        mov ecx, [ecx+4]
        mov eax, esp
        mov [esp+4], esp
        movss dword ptr [eax], xmm0
        mov eax, 971870h
        call eax
        call GetCounter
        pop ecx
        retn 4
    }
}

DWORD jmpBackAddress1;
__declspec(naked) void BeginTargetSceneEzDrawDepth()
{
    __asm
    {
        mov ecx,[pHgMan]
        mov ecx, [ecx]
        mov ecx,[ecx+0x50]
        mov eax,[ecx+4]
        push eax
        mov ecx, esi
        mov eax, ebp
        call [fAddDrawPlanBeginTargetScene]
        add esp, 4
        mov ecx, esi
        mov eax, ebp
        call [fAddDrawPlanEndTargetScene]
        mov ecx,esi
        mov eax,ebp
        jmp [jmpBackAddress1]
    }
}

DWORD jmpBackAddress2;
__declspec(naked) void BeginTargetSceneEzDrawNormal1()
{
    __asm
    {
        mov ecx, [pHgMan]
        mov ecx, [ecx]
        mov ecx, [ecx+0x50]
        mov eax, [ecx+4]
        push eax
        mov ecx, esi
        mov eax, ebx
        call [fAddDrawPlanBeginTargetScene]
        add esp, 4
        mov ecx, esi
        mov eax, ebx
        call [fAddDrawPlanEndTargetScene]
        mov ecx, esi
        mov eax, ebx
        call [fAddDrawPlanEndTargetCamera]
        jmp [jmpBackAddress2]
    }
}

DWORD jmpBackAddress3;
__declspec(naked) void BeginTargetSceneEzDrawNormal2()
{
    __asm
    {
        mov ecx, [pHgMan]
        mov ecx, [ecx]
        mov ecx, [ecx+0x50]
        mov eax, [ecx+4]
        push eax
        mov ecx, esi
        mov eax, ebx
        call[fAddDrawPlanBeginTargetScene]
        add esp, 4
        mov ecx, esi
        mov eax, ebx
        call[fAddDrawPlanEndTargetScene]
        mov ecx, esi
        mov eax, ebx
        call [fAddDrawPlanEndTargetCamera]
        jmp [jmpBackAddress3]
    }
}

//DWORD jmpBackAddress1;
//__declspec(naked) void getCameraObj()
//{
//    __asm
//    {
//        mov edx, [eax+0x100]
//        mov [cam_address], edx
//        mov edx, [eax+0x8C]
//        jmp [jmpBackAddress1]
//    }
//}
//
//DWORD jmpBackAddress4;
//__declspec(naked) void getCameraObjAlt()
//{
//    __asm
//    {
//        mov edx, [eax + 0x100]
//        mov[cam_address], edx
//        mov edx, [eax + 0x8C]
//        jmp[jmpBackAddress4]
//    }
//}
//
//// I don't know how much of this is really necessary, and I don't care, so do it all
//DWORD jmpBackAddress2;
//__declspec(naked) void setEzDraw()
//{
//    __asm
//    {
//        mov edx, 0x406E90
//        call edx
//        mov ecx, ebx
//        mov eax, ebp
//        mov edx, 0x406FB0
//        call edx
//        mov ecx, [edi+0x50]
//        mov edx, [ecx+0xC]
//        push edx
//        mov ecx, ebx
//        mov eax, ebp
//        mov edx, 0x406F00
//        call edx
//        add esp, 4
//        mov ecx, ebx
//        mov eax, ebp
//        mov edx, 0x406E90
//        call edx
//        jmp [jmpBackAddress2]
//    }
//}
//
//DWORD jmpBackAddress3;
//__declspec(naked) void setCameraObj()
//{
//    __asm
//    {
//        mov eax, [cam_address]
//        test eax,eax
//        jz zero
//        mov dword ptr[esi + 0x100], eax
//zero:
//        mov eax, [esi+0x100]
//        jmp [jmpBackAddress3]
//    }
//}
//
//__declspec(naked) void freeCam()
//{
//    __asm
//    {
//
//    }
//}

BOOL APIENTRY DllMain( HMODULE hModule, DWORD ul_reason_for_call, LPVOID lpReserved)
{
    static HMODULE dinput8dll = nullptr;
    HANDLE hProcess = NULL;
    LPVOID dbgfix_mem = nullptr;
    LPVOID dbgfix_mem_temp = nullptr;
    UINT FuncSize = 0;
    DWORD* pFunc = nullptr;
    const char nop = 0x90;
    int hookLength = 0;
    DWORD hookAddress;
    char zwritefix[] = { 0x68, 0x5A, 0x16, 0x01 };
    DWORD curProtec;
    DWORD temp;
    //c0000 testing
    int newMax = 0x50;
    int newMaxSize = 0x140;
    FILE* fp = NULL;

    switch (ul_reason_for_call)
    {
    case DLL_PROCESS_ATTACH:
        char path[MAX_PATH];
        GetSystemDirectoryA(path, MAX_PATH);
        strcat_s(path, "\\dinput8.dll");
        dinput8dll = LoadLibraryA(path);

        // Get function addresses
        oDirectInput8Create = (tDirectInput8Create)GetProcAddress(dinput8dll, "DirectInput8Create");

        hProcess = OpenProcess(PROCESS_ALL_ACCESS, FALSE, GetProcId(L"DARKSOULS.exe"));
        //AllocConsole();
        //freopen_s(&fp, "CONIN$", "r", stdin);
       // freopen_s(&fp, "CONOUT$", "w", stdout);
        //Sleep(10000);
        //dbgfix_mem = VirtualAllocEx(hProcess, NULL, 0x1000, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
        ///////////////////STUFF//////////////////////////
        /*DWORD curProtec;
        if (VirtualProtect((LPVOID)0x10033DA, 4, PAGE_EXECUTE_READWRITE, &curProtec)) {
            memcpy((void*)0x10033DA, zwritefix, 4);

            DWORD temp;
            VirtualProtect((LPVOID)0x10033DA, 4, curProtec, &temp);
        }*/

        /*hookLength = 6;
        hookAddress = 0xF2D849;
        jmpBackAddress1 = hookAddress + hookLength;

        Hook((void*)hookAddress, getCameraObj, hookLength);

        hookLength = 6;
        hookAddress = 0xF35789;
        jmpBackAddress4 = hookAddress + hookLength;

        Hook((void*)hookAddress, getCameraObjAlt, hookLength);

        hookLength = 5;
        hookAddress = 0xF929A8;
        jmpBackAddress2 = hookAddress + hookLength;

        Hook((void*)hookAddress, setEzDraw, hookLength);

        hookLength = 6;
        hookAddress = 0x10033A1;
        jmpBackAddress3 = hookAddress + hookLength;

        Hook((void*)hookAddress, setCameraObj, hookLength);*/
        ///////////////////STUFF//////////////////////////
        /*dbgfix_mem_temp = (LPVOID)((UINT)dbgfix_mem + FuncSize);

        FuncSize = 0;
        pFunc = (DWORD*)getCameraObj;

        while (*pFunc != 0x90909090)
        {
            ++pFunc;
            FuncSize += 4;
        }*/

        //WriteProcessMemory(hProcess, (LPVOID)0xF35789, &dbgfix_mem_temp, 4, NULL);
        //WriteProcessMemory(hProcess, (LPVOID)0xF3578E, &nop, 1, NULL);

        //CloseHandle(hProcess);
        
        hookLength = 5;
        hookAddress = 0xEB3798;
        jmpBackAddress1 = hookAddress + hookLength;
        Hook((void*)hookAddress, BeginTargetSceneEzDrawDepth, hookLength);

        hookLength = 9;
        hookAddress = 0xEB437D;
        jmpBackAddress2 = hookAddress + hookLength;
        Hook((void*)hookAddress, BeginTargetSceneEzDrawNormal1, hookLength);
        
        hookLength = 9;
        hookAddress = 0xEB4483;
        jmpBackAddress3 = hookAddress + hookLength;
        Hook((void*)hookAddress, BeginTargetSceneEzDrawNormal2, hookLength);

        /*hookLength = 7;
        hookAddress = 0x54AAB0;
        Hook((void*)hookAddress, FFXTimeMeasure, hookLength);*/

        VirtualProtect((LPVOID)0xFD6529, 19, PAGE_EXECUTE_READWRITE, &curProtec);
        memset((void*)0xFD6529, 0x90, 19);
        VirtualProtect((LPVOID)0xFD6529, 19, curProtec, &temp);
        VirtualProtect((LPVOID)0xFD6546, 3, PAGE_EXECUTE_READWRITE, &curProtec);
        memset((void*)0xFD6546, 0x90, 3);
        VirtualProtect((LPVOID)0xFD6546, 3, curProtec, &temp);

        VirtualProtect((LPVOID)0xC03EE0, 1, PAGE_EXECUTE_READWRITE, &curProtec);
        *(char*)0xC03F1B = 0x50;
        VirtualProtect((LPVOID)0xC03EE0, 1, curProtec, &temp);
        VirtualProtect((LPVOID)0xC03180, 1, PAGE_EXECUTE_READWRITE, &curProtec);
        *(char*)0xC03180 = 0x50;
        VirtualProtect((LPVOID)0xC03180, 1, curProtec, &temp);
        VirtualProtect((LPVOID)0xC064E4, 4, PAGE_EXECUTE_READWRITE, &curProtec);
        *(int*)0xC064E4 = 0x140;
        VirtualProtect((LPVOID)0xC064E4, 4, curProtec, &temp);
        VirtualProtect((LPVOID)0xC064EB, 4, PAGE_EXECUTE_READWRITE, &curProtec);
        *(int*)0xC064EB = 0x140;
        VirtualProtect((LPVOID)0xC064EB, 4, curProtec, &temp);
        CloseHandle(hProcess);
        /**(char*)0xC03EE0 = 0x50;
        *(char*)0xC03180 = 0x50;
        *(int*)0xC064E4 = 0x140;
        *(int*)0xC064EB = 0x140;*/


        break;
    case DLL_THREAD_ATTACH:
        break;
    case DLL_THREAD_DETACH:
        break;
    case DLL_PROCESS_DETACH:
        //VirtualFreeEx(hProcess, dbgfix_mem, 0, MEM_RELEASE);
        FreeLibrary(dinput8dll);

        FreeConsole();

        if (fp)
            fclose(fp);
        break;
    }
    return TRUE;
}

DWORD GetProcId(const wchar_t* procname)
{
    PROCESSENTRY32 pe;
    HANDLE hSnap;
    pe.dwSize = sizeof(PROCESSENTRY32);
    hSnap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, NULL);
    if (Process32First(hSnap, &pe)) {
        do {
            if (wcscmp(pe.szExeFile, procname) == 0) {
                break;
            }
        } while (Process32Next(hSnap, &pe));
    }
    return pe.th32ProcessID;
}