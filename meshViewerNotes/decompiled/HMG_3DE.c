#include "HMG_3DE.h"



void FUN_10001010(void)

{
                    // WARNING: Could not recover jumptable at 0x10001015. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::Init((Init *)&DAT_100384b1);
  return;
}



void FUN_10001020(void)

{
  FUN_100214c2(FUN_10001030);
  return;
}



void FUN_10001030(void)

{
                    // WARNING: Could not recover jumptable at 0x10001035. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::~Init((Init *)&DAT_100384b1);
  return;
}



void FUN_10001050(void)

{
                    // WARNING: Could not recover jumptable at 0x10001055. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::_Winit((_Winit *)&DAT_100384b0);
  return;
}



void FUN_10001060(void)

{
  FUN_100214c2(FUN_10001070);
  return;
}



void FUN_10001070(void)

{
                    // WARNING: Could not recover jumptable at 0x10001075. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::~_Winit((_Winit *)&DAT_100384b0);
  return;
}



void __fastcall FUN_10001080(undefined4 *param_1)

{
  *param_1 = 0xffffffff;
  return;
}



void __fastcall FUN_10001090(undefined4 *param_1)

{
  *param_1 = 0xffffffff;
  return;
}



undefined4 __fastcall FUN_100010a0(int *param_1)

{
  undefined4 *puVar1;
  
  if (DAT_100384b4 != (void *)0x0) {
    puVar1 = FUN_10001230(DAT_100384b4,*param_1);
    if (puVar1 != (undefined4 *)0x0) {
      return puVar1[5];
    }
  }
  return 0;
}



undefined4 __fastcall FUN_100010d0(int *param_1)

{
  undefined4 *puVar1;
  undefined4 uVar2;
  
  if (DAT_100384b4 != (void *)0x0) {
    puVar1 = FUN_10001230(DAT_100384b4,*param_1);
    if (puVar1 != (undefined4 *)0x0) {
      uVar2 = (**(code **)(puVar1[1] + 0x58))();
      return uVar2;
    }
  }
  return 0;
}



int * __cdecl FUN_10001120(int *param_1,undefined4 *param_2)

{
  (**(code **)(*param_1 + 0x24))(*param_2);
  (**(code **)(*param_1 + 0x24))(param_2[1]);
  return param_1;
}



int * __cdecl FUN_10001150(int *param_1,int param_2)

{
  (**(code **)(*param_1 + 0x34))(param_2);
  (**(code **)(*param_1 + 0x34))(param_2 + 4);
  return param_1;
}



void __fastcall FUN_10001180(undefined4 *param_1)

{
  *param_1 = 0;
  param_1[1] = 0;
  return;
}



void __fastcall FUN_10001190(undefined4 *param_1)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  
  puVar1 = (undefined4 *)*param_1;
  while (puVar2 = puVar1, puVar2 != (undefined4 *)0x0) {
    puVar1 = (undefined4 *)*puVar2;
    if (puVar2 != (undefined4 *)0x0) {
      FUN_100012f0((int)puVar2);
      FUN_100212a4(puVar2);
    }
  }
  return;
}



void __thiscall FUN_100011c0(void *this,undefined4 *param_1)

{
  undefined4 uVar1;
  undefined4 *puVar2;
  int *piVar3;
  undefined4 *puVar4;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puVar2 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_100218c8;
  local_c = ExceptionList;
  if (param_1 != (undefined4 *)0x0) {
    ExceptionList = &local_c;
    piVar3 = (int *)FUN_10001310(param_1,&param_1);
    local_4 = 0;
    puVar4 = FUN_10001230(this,*piVar3);
    FUN_10001090(&param_1);
    if (puVar4 == (undefined4 *)0x0) {
                    // WARNING: Load size is inaccurate
      uVar1 = *this;
      *(undefined4 **)this = puVar2;
      *puVar2 = uVar1;
    }
  }
  ExceptionList = local_c;
  return;
}



undefined4 * __thiscall FUN_10001230(void *this,int param_1)

{
  undefined4 *this_00;
  int iVar1;
  int iVar2;
  int *piVar3;
  void *local_4;
  
  local_4 = this;
  if (param_1 != -1) {
    iVar2 = param_1;
                    // WARNING: Load size is inaccurate
    for (this_00 = *this; param_1 = iVar2, this_00 != (undefined4 *)0x0;
        this_00 = (undefined4 *)*this_00) {
      piVar3 = (int *)FUN_10001310(this_00,&local_4);
      iVar1 = *piVar3;
      FUN_10001090(&local_4);
      if (iVar1 == iVar2) {
        FUN_10001090(&param_1);
        return this_00;
      }
      iVar2 = param_1;
    }
  }
  FUN_10001090(&param_1);
  return (undefined4 *)0x0;
}



undefined4 * FUN_10001290(char *param_1)

{
  char cVar1;
  undefined4 *puVar2;
  uint uVar3;
  int iVar4;
  char *pcVar5;
  
  puVar2 = FUN_10001330();
  uVar3 = 0xffffffff;
  pcVar5 = param_1;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar1 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar1 != '\0');
  iVar4 = 0;
  if (0 < (int)~uVar3) {
    do {
      (**(code **)(puVar2[1] + 0x2c))(param_1[iVar4]);
      iVar4 = iVar4 + 1;
    } while (iVar4 < (int)~uVar3);
    return puVar2;
  }
  return puVar2;
}



void __fastcall FUN_100012f0(int param_1)

{
  FUN_10001090((undefined4 *)(param_1 + 0x18));
  FUN_10003210((undefined4 *)(param_1 + 4));
  return;
}



void __thiscall FUN_10001310(void *this,undefined4 *param_1)

{
  *param_1 = *(undefined4 *)((int)this + 0x18);
  return;
}



undefined4 * FUN_10001330(void)

{
  undefined4 *puVar1;
  int *piVar2;
  int local_14;
  undefined4 *local_10;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_100218eb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)operator_new(0x1c);
  local_4 = 0;
  local_10 = puVar1;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_100031a0(puVar1 + 1,0x10);
    FUN_10001080(puVar1 + 6);
    local_14 = -1;
    puVar1[6] = 0xffffffff;
    FUN_10001090(&local_14);
    *puVar1 = 0;
  }
  local_4 = 0xffffffff;
  piVar2 = FUN_100013f0(DAT_100384b4,(int *)&local_10);
  local_14 = *piVar2;
  puVar1[6] = local_14;
  FUN_10001090(&local_14);
  FUN_10001090(&local_10);
  FUN_100011c0(DAT_100384b4,puVar1);
  ExceptionList = local_c;
  return puVar1;
}



int * __thiscall FUN_100013f0(void *this,int *param_1)

{
  int local_4;
  
  local_4 = *(int *)((int)this + 4);
  *(int *)((int)this + 4) = local_4 + 1;
  *param_1 = local_4;
  FUN_10001090(&local_4);
  return param_1;
}



void FUN_10001430(void)

{
                    // WARNING: Could not recover jumptable at 0x10001435. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::Init((Init *)&DAT_100384b9);
  return;
}



void FUN_10001440(void)

{
  FUN_100214c2(FUN_10001450);
  return;
}



void FUN_10001450(void)

{
                    // WARNING: Could not recover jumptable at 0x10001455. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::~Init((Init *)&DAT_100384b9);
  return;
}



void FUN_10001470(void)

{
                    // WARNING: Could not recover jumptable at 0x10001475. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::_Winit((_Winit *)&DAT_100384b8);
  return;
}



void FUN_10001480(void)

{
  FUN_100214c2(FUN_10001490);
  return;
}



void FUN_10001490(void)

{
                    // WARNING: Could not recover jumptable at 0x10001495. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::~_Winit((_Winit *)&DAT_100384b8);
  return;
}



undefined4 * __fastcall FUN_100014a0(undefined4 *param_1)

{
  undefined4 local_18;
  undefined4 *local_14;
  undefined4 *local_10;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021929;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_14 = param_1;
  local_10 = param_1 + 9;
  FUN_100031a0(param_1 + 10,0x10);
  local_4 = 0;
  FUN_10001080(param_1 + 0xf);
  local_4 = 1;
  local_18 = 0xffffffff;
  param_1[0xf] = 0xffffffff;
  FUN_10001090(&local_18);
  param_1[9] = 0;
  local_4 = 2;
  param_1[6] = 0;
  param_1[7] = 0;
  param_1[3] = 0;
  *param_1 = s_default_appname_10035090;
  param_1[2] = s_default_file_desc_100350a0;
  param_1[1] = s_default_dbName_100350b4;
  local_10 = (undefined4 *)operator_new(8);
  local_4 = CONCAT31(local_4._1_3_,3);
  if (local_10 == (undefined4 *)0x0) {
    DAT_100384b4 = 0;
  }
  else {
    DAT_100384b4 = FUN_10001180(local_10);
  }
  ExceptionList = local_c;
  return param_1;
}



void __fastcall FUN_10001570(int param_1)

{
  undefined4 *puVar1;
  int local_14;
  int local_10;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_10021951;
  local_c = ExceptionList;
  local_4 = 0;
  ExceptionList = &local_c;
  local_14 = param_1;
  if (*(undefined4 **)(param_1 + 0xc) != (undefined4 *)0x0) {
    ExceptionList = &local_c;
    FUN_10001ad0(*(undefined4 **)(param_1 + 0xc));
  }
  puVar1 = DAT_100384b4;
  if (DAT_100384b4 != (undefined4 *)0x0) {
    FUN_10001190(DAT_100384b4);
    FUN_100212a4(puVar1);
  }
  local_10 = param_1 + 0x24;
  DAT_100384b4 = (undefined4 *)0x0;
  local_4 = 1;
  local_14 = -1;
  *(undefined4 *)(param_1 + 0x3c) = 0xffffffff;
  FUN_10001090(&local_14);
  local_4 = 2;
  FUN_10001090((undefined4 *)(param_1 + 0x3c));
  local_4 = 0xffffffff;
  FUN_10003210((undefined4 *)(param_1 + 0x28));
  ExceptionList = local_c;
  return;
}



void __thiscall FUN_10001690(void *this,int param_1)

{
  int iVar1;
  undefined4 local_28;
  undefined4 local_24;
  undefined4 local_20;
  int local_1c;
  int local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_10021963;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (param_1 != 0) {
    do {
      local_28 = *(undefined4 *)(param_1 + 4);
      local_10 = *(undefined4 *)(param_1 + 0x14);
      local_24 = *(undefined4 *)(param_1 + 8);
      local_20 = *(undefined4 *)(param_1 + 0xc);
      local_14 = *(undefined4 *)(param_1 + 0x10);
      local_4 = 0;
      if (*(int *)(param_1 + 0x18) == 0) {
        local_1c = 0;
      }
      else if (*(int *)(param_1 + 0x1c) == 0) {
        local_1c = *(int *)((int)this + 0x1c) + 0x1c;
      }
      else {
        iVar1 = FUN_10001ab0(this,*(int *)(param_1 + 0x1c));
        local_1c = *(int *)((int)this + 0x1c) + (iVar1 + 1) * 0x1c;
      }
      if (*(int *)(param_1 + 0x1c) == 0) {
        local_18 = 0;
      }
      else {
        local_18 = *(int *)((int)this + 0x1c) + 0x1c;
      }
      FUN_10003820(*(int **)((int)this + 0x14),&local_28);
      *(int *)((int)this + 0x1c) = *(int *)((int)this + 0x1c) + 0x1c;
      if (*(int *)(param_1 + 0x1c) != 0) {
        FUN_10001690(this,*(int *)(param_1 + 0x1c));
      }
      param_1 = *(int *)(param_1 + 0x18);
    } while (param_1 != 0);
  }
  ExceptionList = local_c;
  return;
}



void __fastcall FUN_10001780(int param_1)

{
  void **ppvVar1;
  void *pvVar2;
  undefined4 *puVar3;
  int iVar4;
  int iVar5;
  int iVar6;
  int local_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_1002197d;
  pvStack_c = ExceptionList;
  iVar5 = 0;
  iVar6 = 0;
  ExceptionList = &pvStack_c;
  ppvVar1 = &pvStack_c;
  if (0 < *(int *)(DAT_100384b4 + 4)) {
    do {
      local_4 = 0xffffffff;
      iVar4 = iVar6;
      pvVar2 = (void *)FUN_10001e70();
      puVar3 = FUN_10001230(pvVar2,iVar4);
      local_14 = *(int *)(param_1 + 0x18) + iVar5;
      uStack_10 = (**(code **)(puVar3[1] + 0x58))();
      FUN_10001120(*(int **)(param_1 + 0x14),&local_14);
      iVar4 = (**(code **)(puVar3[1] + 0x58))();
      iVar5 = iVar5 + iVar4;
      iVar6 = iVar6 + 1;
      ppvVar1 = (void **)ExceptionList;
    } while (iVar6 < *(int *)(DAT_100384b4 + 4));
  }
  ExceptionList = ppvVar1;
  iVar5 = 0;
  if (0 < *(int *)(DAT_100384b4 + 4)) {
    do {
      local_4 = 0xffffffff;
      iVar6 = iVar5;
      pvVar2 = (void *)FUN_10001e70();
      puVar3 = FUN_10001230(pvVar2,iVar6);
      (**(code **)(puVar3[1] + 0x58))();
      (**(code **)(**(int **)(param_1 + 0x14) + 0x44))(puVar3[5]);
      iVar5 = iVar5 + 1;
    } while (iVar5 < *(int *)(DAT_100384b4 + 4));
  }
  ExceptionList = pvStack_c;
  return;
}



void __thiscall FUN_10001870(void *this,int *param_1,int param_2)

{
  bool bVar1;
  int *piVar2;
  void *pvVar3;
  undefined4 local_28;
  int *local_24;
  int local_1c;
  int local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined *puStack_8;
  int local_4;
  
  piVar2 = param_1;
  puStack_8 = &LAB_1002199a;
  local_c = ExceptionList;
  bVar1 = true;
  ExceptionList = &local_c;
  do {
    local_4 = 0;
    FUN_10003880(*(int **)((int)this + 0x10),(int)&local_28);
    if (local_1c == 0) {
      bVar1 = false;
    }
    param_1 = (int *)operator_new(0x20);
    local_4._0_1_ = 1;
    if (param_1 == (int *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_100038e0(param_1,local_28);
    }
    *(undefined4 *)((int)pvVar3 + 0x10) = local_14;
    *(undefined4 *)((int)pvVar3 + 0x14) = local_10;
    local_4 = (uint)local_4._1_3_ << 8;
    param_1 = local_24;
    *(int **)((int)pvVar3 + 8) = local_24;
    FUN_10001090(&param_1);
    FUN_10003a00(piVar2,(int)pvVar3);
    if (local_18 != 0) {
      FUN_10001870(this,(int *)((int)pvVar3 + 0x1c),param_2 + 1);
    }
  } while (bVar1);
  ExceptionList = local_c;
  return;
}



void __thiscall FUN_10001960(void *this,undefined4 param_1,int param_2)

{
  undefined4 *puVar1;
  void *pvVar2;
  undefined4 unaff_EBP;
  int iVar3;
  undefined4 unaff_ESI;
  int local_1c;
  undefined4 *local_18;
  uint local_14 [2];
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_100219ba;
  local_c = ExceptionList;
  iVar3 = 0;
  ExceptionList = &local_c;
  if (0 < param_2) {
    do {
      puVar1 = (undefined4 *)operator_new(0x1c);
      local_4 = 0;
      local_18 = puVar1;
      if (puVar1 == (undefined4 *)0x0) {
        puVar1 = (undefined4 *)0x0;
      }
      else {
        FUN_100031a0(puVar1 + 1,0x10);
        local_4 = CONCAT31(local_4._1_3_,1);
        FUN_10001080(puVar1 + 6);
      }
      local_4 = 0xffffffff;
      puVar1[6] = iVar3;
      local_1c = iVar3;
      FUN_10001090(&local_1c);
      FUN_10001150(*(int **)((int)this + 0x10),(int)local_14);
      local_18 = (undefined4 *)(**(code **)(**(int **)((int)this + 0x10) + 0x54))();
      (**(code **)(**(int **)((int)this + 0x10) + 0x48))(local_14[0]);
      pvVar2 = operator_new(local_14[0]);
      (**(code **)(**(int **)((int)this + 0x10) + 0x40))(pvVar2,local_14[0]);
      (**(code **)(**(int **)((int)this + 0x10) + 0x48))(unaff_ESI);
      (**(code **)(puVar1[1] + 0x44))(pvVar2,unaff_EBP);
      FUN_100011c0(DAT_100384b4,puVar1);
      FUN_100212a4(pvVar2);
      iVar3 = iVar3 + 1;
    } while (iVar3 < param_2);
  }
  ExceptionList = local_c;
  return;
}



void __thiscall FUN_10001a80(void *this,int param_1)

{
  if (param_1 != 0) {
    do {
      *(int *)((int)this + 0x20) = *(int *)((int)this + 0x20) + 1;
      if (*(int *)(param_1 + 0x1c) != 0) {
        FUN_10001a80(this,*(int *)(param_1 + 0x1c));
      }
      param_1 = *(int *)(param_1 + 0x18);
    } while (param_1 != 0);
  }
  return;
}



undefined4 __thiscall FUN_10001ab0(void *this,int param_1)

{
  *(undefined4 *)((int)this + 0x20) = 0;
  FUN_10001a80(this,param_1);
  return *(undefined4 *)((int)this + 0x20);
}



void FUN_10001ad0(undefined4 *param_1)

{
  undefined4 *puVar1;
  
  if (param_1 != (undefined4 *)0x0) {
    do {
      if ((undefined4 *)param_1[7] != (undefined4 *)0x0) {
        FUN_10001ad0((undefined4 *)param_1[7]);
      }
      puVar1 = (undefined4 *)param_1[6];
      if (param_1 != (undefined4 *)0x0) {
        FUN_100039e0(param_1);
        FUN_100212a4(param_1);
      }
      param_1 = puVar1;
    } while (puVar1 != (undefined4 *)0x0);
  }
  return;
}



void __thiscall FUN_10001c00(void *this,int *param_1)

{
  char cVar1;
  uint uVar2;
  uint uVar3;
  int iVar4;
  char *pcVar5;
  char *pcVar6;
  char *pcVar7;
  char local_1ba [2];
  undefined4 local_1b8 [16];
  undefined4 local_178;
  undefined4 local_174;
  undefined4 local_170;
  undefined4 local_15c;
  undefined4 local_158;
  int local_154;
  int local_150;
  char local_14c [32];
  char local_12c [32];
  char local_10c [256];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_100219cf;
  pvStack_c = ExceptionList;
  local_1ba[0] = '\x1a';
  local_1ba[1] = 0;
  ExceptionList = &pvStack_c;
  *(int **)((int)this + 0x14) = param_1;
  uVar2 = 0xffffffff;
  pcVar5 = s_SurRender_database_file_100350c4;
  do {
    pcVar7 = pcVar5;
    if (uVar2 == 0) break;
    uVar2 = uVar2 - 1;
    pcVar7 = pcVar5 + 1;
    cVar1 = *pcVar5;
    pcVar5 = pcVar7;
  } while (cVar1 != '\0');
  uVar2 = ~uVar2;
  local_4 = 0;
  pcVar5 = pcVar7 + -uVar2;
  pcVar7 = (char *)local_1b8;
  for (uVar3 = uVar2 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
    *(undefined4 *)pcVar7 = *(undefined4 *)pcVar5;
    pcVar5 = pcVar5 + 4;
    pcVar7 = pcVar7 + 4;
  }
  for (uVar2 = uVar2 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
    *pcVar7 = *pcVar5;
    pcVar5 = pcVar5 + 1;
    pcVar7 = pcVar7 + 1;
  }
  uVar2 = 0xffffffff;
  pcVar5 = local_1ba;
  do {
    pcVar7 = pcVar5;
    if (uVar2 == 0) break;
    uVar2 = uVar2 - 1;
    pcVar7 = pcVar5 + 1;
    cVar1 = *pcVar5;
    pcVar5 = pcVar7;
  } while (cVar1 != '\0');
  uVar2 = ~uVar2;
  iVar4 = -1;
  pcVar5 = (char *)local_1b8;
  do {
    pcVar6 = pcVar5;
    if (iVar4 == 0) break;
    iVar4 = iVar4 + -1;
    pcVar6 = pcVar5 + 1;
    cVar1 = *pcVar5;
    pcVar5 = pcVar6;
  } while (cVar1 != '\0');
  iVar4 = *(int *)((int)this + 0xc);
  pcVar5 = pcVar7 + -uVar2;
  pcVar7 = pcVar6 + -1;
  for (uVar3 = uVar2 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
    *(undefined4 *)pcVar7 = *(undefined4 *)pcVar5;
    pcVar5 = pcVar5 + 4;
    pcVar7 = pcVar7 + 4;
  }
  for (uVar2 = uVar2 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
    *pcVar7 = *pcVar5;
    pcVar5 = pcVar5 + 1;
    pcVar7 = pcVar7 + 1;
  }
  local_178 = 0xbebaadde;
  local_174 = 0xefbe01c0;
  local_170 = 0x100;
  local_15c = 0x1ac;
  local_158 = FUN_10001ab0(this,iVar4);
  uVar2 = 0xffffffff;
  pcVar5 = *(char **)((int)this + 4);
  do {
    pcVar7 = pcVar5;
    if (uVar2 == 0) break;
    uVar2 = uVar2 - 1;
    pcVar7 = pcVar5 + 1;
    cVar1 = *pcVar5;
    pcVar5 = pcVar7;
  } while (cVar1 != '\0');
  uVar2 = ~uVar2;
  pcVar5 = pcVar7 + -uVar2;
  pcVar7 = local_14c;
  for (uVar3 = uVar2 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
    *(undefined4 *)pcVar7 = *(undefined4 *)pcVar5;
    pcVar5 = pcVar5 + 4;
    pcVar7 = pcVar7 + 4;
  }
  for (uVar2 = uVar2 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
    *pcVar7 = *pcVar5;
    pcVar5 = pcVar5 + 1;
    pcVar7 = pcVar7 + 1;
  }
                    // WARNING: Load size is inaccurate
  uVar2 = 0xffffffff;
  pcVar5 = *this;
  do {
    pcVar7 = pcVar5;
    if (uVar2 == 0) break;
    uVar2 = uVar2 - 1;
    pcVar7 = pcVar5 + 1;
    cVar1 = *pcVar5;
    pcVar5 = pcVar7;
  } while (cVar1 != '\0');
  uVar2 = ~uVar2;
  pcVar5 = pcVar7 + -uVar2;
  pcVar7 = local_12c;
  for (uVar3 = uVar2 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
    *(undefined4 *)pcVar7 = *(undefined4 *)pcVar5;
    pcVar5 = pcVar5 + 4;
    pcVar7 = pcVar7 + 4;
  }
  for (uVar2 = uVar2 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
    *pcVar7 = *pcVar5;
    pcVar5 = pcVar5 + 1;
    pcVar7 = pcVar7 + 1;
  }
  uVar2 = 0xffffffff;
  pcVar5 = *(char **)((int)this + 8);
  do {
    pcVar7 = pcVar5;
    if (uVar2 == 0) break;
    uVar2 = uVar2 - 1;
    pcVar7 = pcVar5 + 1;
    cVar1 = *pcVar5;
    pcVar5 = pcVar7;
  } while (cVar1 != '\0');
  uVar2 = ~uVar2;
  pcVar5 = pcVar7 + -uVar2;
  pcVar7 = local_10c;
  for (uVar3 = uVar2 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
    *(undefined4 *)pcVar7 = *(undefined4 *)pcVar5;
    pcVar5 = pcVar5 + 4;
    pcVar7 = pcVar7 + 4;
  }
  for (uVar2 = uVar2 & 3; uVar2 != 0; uVar2 = uVar2 - 1) {
    *pcVar7 = *pcVar5;
    pcVar5 = pcVar5 + 1;
    pcVar7 = pcVar7 + 1;
  }
  *(undefined4 *)((int)this + 0x1c) = 0x1ac;
  iVar4 = FUN_10001ab0(this,*(int *)((int)this + 0xc));
  local_154 = *(int *)((int)this + 0x1c) + iVar4 * 0x1c;
  local_150 = *(int *)(DAT_100384b4 + 4);
  *(int *)((int)this + 0x18) = local_154 + local_150 * 8;
  FUN_100036c0(param_1,(int)local_1b8);
  FUN_10001690(this,*(int *)((int)this + 0xc));
  FUN_10001780((int)this);
  (**(code **)(*param_1 + 0x1c))();
  ExceptionList = pvStack_c;
  return;
}



void __thiscall FUN_10001dc0(void *this,int *param_1)

{
  int *piVar1;
  undefined local_1b8 [92];
  undefined4 local_15c;
  int iStack_154;
  void *pvStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_100219e4;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(int **)((int)this + 0x10) = param_1;
  piVar1 = (int *)((int)this + 0xc);
  local_4 = 0;
  *piVar1 = 0;
  FUN_10003770(param_1,(int)local_1b8);
  (**(code **)(*param_1 + 0x48))(local_15c);
  FUN_10001870(this,piVar1,1);
  FUN_10001960(this,*piVar1,iStack_154);
  (**(code **)(*param_1 + 0x1c))();
  ExceptionList = pvStack_10;
  return;
}



undefined4 FUN_10001e70(void)

{
  return DAT_100384b4;
}



void __fastcall FUN_10001e90(int param_1)

{
  undefined4 local_14;
  int local_10;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_10021a03;
  local_c = ExceptionList;
  local_4 = 0;
  local_14 = 0xffffffff;
  ExceptionList = &local_c;
  *(undefined4 *)(param_1 + 0x18) = 0xffffffff;
  local_10 = param_1;
  FUN_10001090(&local_14);
  local_4 = 1;
  FUN_10001090((undefined4 *)(param_1 + 0x18));
  local_4 = 0xffffffff;
  FUN_10003210((undefined4 *)(param_1 + 4));
  ExceptionList = local_c;
  return;
}



void FUN_10001f20(void)

{
                    // WARNING: Could not recover jumptable at 0x10001f25. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::Init((Init *)&DAT_100384bd);
  return;
}



void FUN_10001f30(void)

{
  FUN_100214c2(FUN_10001f40);
  return;
}



void FUN_10001f40(void)

{
                    // WARNING: Could not recover jumptable at 0x10001f45. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::~Init((Init *)&DAT_100384bd);
  return;
}



void FUN_10001f60(void)

{
                    // WARNING: Could not recover jumptable at 0x10001f65. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::_Winit((_Winit *)&DAT_100384bc);
  return;
}



void FUN_10001f70(void)

{
  FUN_100214c2(FUN_10001f80);
  return;
}



void FUN_10001f80(void)

{
                    // WARNING: Could not recover jumptable at 0x10001f85. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::~_Winit((_Winit *)&DAT_100384bc);
  return;
}



undefined4 * __fastcall FUN_10001f90(undefined4 *param_1)

{
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021a18;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_10003960(param_1);
  local_4 = 0;
  *param_1 = &PTR_LAB_100257c0;
  param_1[1] = 7;
  FUN_10003a50(param_1,s_pointlight_10035120,4);
  ExceptionList = local_c;
  return param_1;
}



void __thiscall FUN_10002040(void *this,undefined4 *param_1)

{
  void *this_00;
  int *piVar1;
  undefined4 uVar2;
  void *local_10;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021a2a;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = this;
  this_00 = (void *)FUN_10003c50(this,8);
  if (this_00 == (void *)0x0) {
    *param_1 = 0;
    ExceptionList = local_c;
    return;
  }
  piVar1 = (int *)FUN_10002180(this_00,&local_10);
  local_4 = 0;
  uVar2 = FUN_100010a0(piVar1);
  *param_1 = uVar2;
  local_4 = 0xffffffff;
  FUN_10001090(&local_10);
  ExceptionList = local_c;
  return;
}



void __thiscall FUN_10002100(void *this,undefined4 *param_1)

{
  void *this_00;
  int *piVar1;
  undefined4 uVar2;
  void *local_10;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021a3c;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = this;
  this_00 = (void *)FUN_10003c50(this,9);
  if (this_00 == (void *)0x0) {
    *param_1 = 0;
    ExceptionList = local_c;
    return;
  }
  piVar1 = (int *)FUN_10002180(this_00,&local_10);
  local_4 = 0;
  uVar2 = FUN_100010a0(piVar1);
  *param_1 = uVar2;
  local_4 = 0xffffffff;
  FUN_10001090(&local_10);
  ExceptionList = local_c;
  return;
}



void __thiscall FUN_10002180(void *this,undefined4 *param_1)

{
  *param_1 = *(undefined4 *)((int)this + 8);
  return;
}



undefined4 * FUN_100021a0(void)

{
  undefined4 *puVar1;
  int *piVar2;
  int local_14;
  undefined4 *local_10;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021a76;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)operator_new(0x1c);
  local_4 = 0;
  local_10 = puVar1;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_100031a0(puVar1 + 1,0x10);
    local_4._0_1_ = 1;
    FUN_10001080(puVar1 + 6);
    local_4 = CONCAT31(local_4._1_3_,2);
    local_14 = -1;
    puVar1[6] = 0xffffffff;
    FUN_10001090(&local_14);
    *puVar1 = 0;
  }
  local_4 = 0xffffffff;
  piVar2 = FUN_100013f0(DAT_100384b4,(int *)&local_10);
  local_14 = *piVar2;
  local_4 = 3;
  puVar1[6] = local_14;
  FUN_10001090(&local_14);
  local_4 = 0xffffffff;
  FUN_10001090(&local_10);
  FUN_100011c0(DAT_100384b4,puVar1);
  ExceptionList = local_c;
  return puVar1;
}



void FUN_10002290(void)

{
                    // WARNING: Could not recover jumptable at 0x10002295. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::Init((Init *)&DAT_100384c1);
  return;
}



void FUN_100022a0(void)

{
  FUN_100214c2(FUN_100022b0);
  return;
}



void FUN_100022b0(void)

{
                    // WARNING: Could not recover jumptable at 0x100022b5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::~Init((Init *)&DAT_100384c1);
  return;
}



void FUN_100022d0(void)

{
                    // WARNING: Could not recover jumptable at 0x100022d5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::_Winit((_Winit *)&DAT_100384c0);
  return;
}



void FUN_100022e0(void)

{
  FUN_100214c2(FUN_100022f0);
  return;
}



void FUN_100022f0(void)

{
                    // WARNING: Could not recover jumptable at 0x100022f5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::~_Winit((_Winit *)&DAT_100384c0);
  return;
}



int * __cdecl FUN_10002300(int *param_1,undefined4 *param_2)

{
  (**(code **)(*param_1 + 0x24))(*param_2);
  FUN_10007d40(param_1,param_2[1]);
  FUN_10007d40(param_1,param_2[4]);
  FUN_10007d40(param_1,param_2[7]);
  FUN_10007d40(param_1,param_2[10]);
  (**(code **)(*param_1 + 0x20))(param_2[0xd]);
  (**(code **)(*param_1 + 0x20))(param_2[0xe]);
  return param_1;
}



int * __cdecl FUN_10002460(int *param_1,undefined4 *param_2)

{
  (**(code **)(*param_1 + 0x24))(*param_2);
  (**(code **)(*param_1 + 0x24))(param_2[1]);
  return param_1;
}



undefined4 * __fastcall FUN_100024c0(undefined4 *param_1)

{
  FUN_10003960(param_1);
  *param_1 = &PTR_LAB_100257c8;
  return param_1;
}



undefined4 * __fastcall FUN_100024f0(undefined4 *param_1)

{
  void *this;
  void *local_c;
  undefined *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021a93;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_10003960(param_1);
  local_4 = 0;
  *param_1 = &PTR_LAB_100257d0;
  param_1[1] = 0xb;
  this = operator_new(0x20);
  local_4._0_1_ = 1;
  if (this != (void *)0x0) {
    FUN_100038e0(this,4);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_10003a50(param_1,&DAT_1003516c,4);
  ExceptionList = local_c;
  return param_1;
}



void FUN_10002580(undefined4 param_1)

{
  undefined4 *puVar1;
  int *piVar2;
  int iVar3;
  undefined4 *puVar4;
  undefined4 *puVar5;
  void *local_60;
  undefined4 local_5c;
  int local_58;
  int local_54;
  undefined4 *local_50;
  void *local_4c;
  undefined4 local_48 [15];
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021acb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_50 = (undefined4 *)operator_new(0x20);
  local_4 = 0;
  if (local_50 == (undefined4 *)0x0) {
    local_60 = (void *)0x0;
  }
  else {
    local_60 = FUN_100038e0(local_50,0xc);
  }
  local_4 = 0xffffffff;
  puVar1 = (undefined4 *)operator_new(0x1c);
  local_4 = 1;
  local_50 = puVar1;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_10002990((int)puVar1);
    local_4 = CONCAT31(local_4._1_3_,2);
    local_5c = 0xffffffff;
    puVar1[6] = 0xffffffff;
    FUN_10001090(&local_5c);
    *puVar1 = 0;
  }
  local_4 = 0xffffffff;
  piVar2 = FUN_100013f0(DAT_100384b4,&local_54);
  local_58 = *piVar2;
  local_4 = 3;
  puVar1[6] = local_58;
  FUN_10001090(&local_58);
  local_4 = 0xffffffff;
  FUN_10001090(&local_54);
  FUN_100011c0(DAT_100384b4,puVar1);
  puVar4 = &param_1;
  puVar5 = local_48;
  for (iVar3 = 0xf; iVar3 != 0; iVar3 = iVar3 + -1) {
    *puVar5 = *puVar4;
    puVar4 = puVar4 + 1;
    puVar5 = puVar5 + 1;
  }
  FUN_10002300(puVar1 + 1,local_48);
  puVar1 = (undefined4 *)FUN_10001310(puVar1,&local_50);
  param_1 = *puVar1;
  local_4 = 4;
  *(undefined4 *)((int)local_60 + 8) = param_1;
  FUN_10001090(&param_1);
  local_4 = 0xffffffff;
  FUN_10001090(&local_50);
  FUN_100039f0(local_4c,(int)local_60);
  ExceptionList = local_c;
  return;
}



void __thiscall FUN_100026d0(void *this,undefined4 *param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  iVar1 = FUN_10003c50(this,0xc);
  if (iVar1 == 0) {
    *param_1 = 0;
    return;
  }
  uVar2 = FUN_100010a0((int *)(iVar1 + 8));
  *param_1 = uVar2;
  return;
}



undefined4 * __fastcall FUN_10002700(undefined4 *param_1)

{
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021add;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_10003960(param_1);
  local_4 = 0;
  *param_1 = &PTR_LAB_100257d8;
  param_1[1] = 0xd;
  FUN_10003a50(param_1,&DAT_10035170,4);
  ExceptionList = local_c;
  return param_1;
}



void __thiscall FUN_10002770(void *this,int param_1,undefined4 param_2)

{
  undefined4 *puVar1;
  int *piVar2;
  void *pvVar3;
  undefined4 local_20;
  int local_1c;
  int local_18;
  undefined4 local_14;
  undefined4 *local_10;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021b15;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = (undefined4 *)operator_new(0x20);
  pvVar3 = (void *)0x0;
  local_4 = 0;
  if (local_10 != (undefined4 *)0x0) {
    pvVar3 = FUN_100038e0(local_10,0xe);
  }
  local_4 = 0xffffffff;
  puVar1 = (undefined4 *)operator_new(0x1c);
  local_4 = 1;
  local_10 = puVar1;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_10002990((int)puVar1);
    local_4 = CONCAT31(local_4._1_3_,2);
    local_20 = 0xffffffff;
    puVar1[6] = 0xffffffff;
    FUN_10001090(&local_20);
    *puVar1 = 0;
  }
  local_4 = 0xffffffff;
  piVar2 = FUN_100013f0(DAT_100384b4,&local_18);
  local_1c = *piVar2;
  local_4 = 3;
  puVar1[6] = local_1c;
  FUN_10001090(&local_1c);
  local_4 = 0xffffffff;
  FUN_10001090(&local_18);
  FUN_100011c0(DAT_100384b4,puVar1);
  local_18 = param_1;
  local_14 = param_2;
  FUN_10002460(puVar1 + 1,&local_18);
  piVar2 = (int *)FUN_10001310(puVar1,&local_10);
  param_1 = *piVar2;
  local_4 = 4;
  *(int *)((int)pvVar3 + 8) = param_1;
  FUN_10001090(&param_1);
  local_4 = 0xffffffff;
  FUN_10001090(&local_10);
  FUN_100039f0(this,(int)pvVar3);
  ExceptionList = local_c;
  return;
}



void __thiscall FUN_100028c0(void *this,undefined4 *param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  iVar1 = FUN_10003c50(this,0xe);
  if (iVar1 == 0) {
    *param_1 = 0;
    return;
  }
  uVar2 = FUN_100010a0((int *)(iVar1 + 8));
  *param_1 = uVar2;
  return;
}



void __thiscall FUN_10002900(void *this,undefined4 *param_1)

{
  void *this_00;
  int *piVar1;
  undefined4 uVar2;
  void *local_10;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021b27;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = this;
  this_00 = (void *)FUN_10003c50(this,0xf);
  if (this_00 == (void *)0x0) {
    *param_1 = 0;
    ExceptionList = local_c;
    return;
  }
  piVar1 = (int *)FUN_10002180(this_00,&local_10);
  local_4 = 0;
  uVar2 = FUN_100010a0(piVar1);
  local_4 = 0xffffffff;
  FUN_10001090(&local_10);
  *param_1 = uVar2;
  ExceptionList = local_c;
  return;
}



int __fastcall FUN_10002990(int param_1)

{
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021b4b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_100031a0((void *)(param_1 + 4),0x10);
  local_4 = 0;
  FUN_10001080((undefined4 *)(param_1 + 0x18));
  ExceptionList = local_c;
  return param_1;
}



void FUN_100029f0(void)

{
                    // WARNING: Could not recover jumptable at 0x100029f5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::Init((Init *)&DAT_100384c5);
  return;
}



void FUN_10002a00(void)

{
  FUN_100214c2(FUN_10002a10);
  return;
}



void FUN_10002a10(void)

{
                    // WARNING: Could not recover jumptable at 0x10002a15. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::~Init((Init *)&DAT_100384c5);
  return;
}



void FUN_10002a30(void)

{
                    // WARNING: Could not recover jumptable at 0x10002a35. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::_Winit((_Winit *)&DAT_100384c4);
  return;
}



void FUN_10002a40(void)

{
  FUN_100214c2(FUN_10002a50);
  return;
}



void FUN_10002a50(void)

{
                    // WARNING: Could not recover jumptable at 0x10002a55. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::~_Winit((_Winit *)&DAT_100384c4);
  return;
}



undefined4 * __fastcall FUN_10002a60(undefined4 *param_1)

{
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021b68;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_10003960(param_1);
  local_4 = 0;
  *param_1 = &PTR_LAB_100257e0;
  param_1[1] = 0x11;
  FUN_10003a50(param_1,s_object_100351b4,4);
  ExceptionList = local_c;
  return param_1;
}



int * __cdecl FUN_10002ad0(int *param_1,undefined4 *param_2)

{
  (**(code **)(*param_1 + 0x24))(*param_2);
  return param_1;
}



void __thiscall FUN_10002b10(void *this,undefined4 param_1)

{
  undefined4 *puVar1;
  int *piVar2;
  void *pvVar3;
  undefined4 local_20;
  int local_1c;
  int local_18;
  undefined4 local_14;
  undefined4 *local_10;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021ba0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = (undefined4 *)operator_new(0x20);
  pvVar3 = (void *)0x0;
  local_4 = 0;
  if (local_10 != (undefined4 *)0x0) {
    pvVar3 = FUN_100038e0(local_10,0x12);
  }
  local_4 = 0xffffffff;
  puVar1 = (undefined4 *)operator_new(0x1c);
  local_4 = 1;
  local_10 = puVar1;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_10002990((int)puVar1);
    local_4 = CONCAT31(local_4._1_3_,2);
    local_20 = 0xffffffff;
    puVar1[6] = 0xffffffff;
    FUN_10001090(&local_20);
    *puVar1 = 0;
  }
  local_4 = 0xffffffff;
  piVar2 = FUN_100013f0(DAT_100384b4,&local_18);
  local_1c = *piVar2;
  local_4 = 3;
  puVar1[6] = local_1c;
  FUN_10001090(&local_1c);
  local_4 = 0xffffffff;
  FUN_10001090(&local_18);
  FUN_100011c0(DAT_100384b4,puVar1);
  FUN_10002ad0(puVar1 + 1,&param_1);
  puVar1 = (undefined4 *)FUN_10001310(puVar1,&local_10);
  local_14 = *puVar1;
  local_4 = 4;
  *(undefined4 *)((int)pvVar3 + 8) = local_14;
  FUN_10001090(&local_14);
  local_4 = 0xffffffff;
  FUN_10001090(&local_10);
  FUN_100039f0(this,(int)pvVar3);
  ExceptionList = local_c;
  return;
}



undefined4 * __fastcall FUN_10002cc0(undefined4 *param_1)

{
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021bb2;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_10003960(param_1);
  local_4 = 0;
  *param_1 = &PTR_LAB_100257e8;
  param_1[1] = 0;
  FUN_10003a50(param_1,&DAT_100351bc,4);
  ExceptionList = local_c;
  return param_1;
}



void __thiscall FUN_10002dc0(void *this,uint *param_1,undefined4 *param_2)

{
  int iVar1;
  uint uVar2;
  undefined4 uVar3;
  
  iVar1 = FUN_10003c50(this,1);
  if (iVar1 == 0) {
    *param_1 = 0;
    *param_2 = 0;
    return;
  }
  uVar2 = FUN_100010d0((int *)(iVar1 + 8));
  *param_1 = uVar2 / 0xc;
  uVar3 = FUN_100010a0((int *)(iVar1 + 8));
  *param_2 = uVar3;
  return;
}



void __thiscall FUN_10002e10(void *this,uint *param_1,undefined4 *param_2)

{
  int iVar1;
  uint uVar2;
  undefined4 uVar3;
  
  iVar1 = FUN_10003c50(this,2);
  if (iVar1 == 0) {
    *param_1 = 0;
    *param_2 = 0;
    return;
  }
  uVar2 = FUN_100010d0((int *)(iVar1 + 8));
  *param_1 = uVar2 / 0xc;
  uVar3 = FUN_100010a0((int *)(iVar1 + 8));
  *param_2 = uVar3;
  return;
}



void __thiscall FUN_10002e60(void *this,uint *param_1,undefined4 *param_2)

{
  int iVar1;
  uint uVar2;
  undefined4 uVar3;
  
  iVar1 = FUN_10003c50(this,5);
  if (iVar1 == 0) {
    *param_1 = 0;
    *param_2 = 0;
    return;
  }
  uVar2 = FUN_100010d0((int *)(iVar1 + 8));
  *param_1 = uVar2 / 0x18;
  uVar3 = FUN_100010a0((int *)(iVar1 + 8));
  *param_2 = uVar3;
  return;
}



void __thiscall FUN_10002ed0(void *this,undefined4 *param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  iVar1 = FUN_10003c50(this,6);
  if (iVar1 != 0) {
    uVar2 = FUN_100010a0((int *)(iVar1 + 8));
    *param_1 = uVar2;
  }
  return;
}



void __thiscall FUN_10002f10(void *this,uint *param_1,undefined4 *param_2)

{
  int iVar1;
  uint uVar2;
  undefined4 uVar3;
  
  iVar1 = FUN_10003c50(this,10);
  if (iVar1 == 0) {
    *param_1 = 0;
    *param_2 = 0;
    return;
  }
  uVar2 = FUN_100010d0((int *)(iVar1 + 8));
  *param_1 = uVar2 >> 2;
  uVar3 = FUN_100010a0((int *)(iVar1 + 8));
  *param_2 = uVar3;
  return;
}



void __thiscall FUN_10002ff0(void *this,uint *param_1,undefined4 *param_2)

{
  int iVar1;
  uint uVar2;
  undefined4 uVar3;
  
  iVar1 = FUN_10003c50(this,0x15);
  if (iVar1 == 0) {
    *param_1 = 0;
    *param_2 = 0;
    return;
  }
  uVar2 = FUN_100010d0((int *)(iVar1 + 8));
  *param_1 = uVar2 >> 2;
  uVar3 = FUN_100010a0((int *)(iVar1 + 8));
  *param_2 = uVar3;
  return;
}



void __fastcall FUN_10003040(int param_1)

{
  *(undefined4 *)(param_1 + 4) = 0;
  return;
}



void __fastcall FUN_10003050(int param_1)

{
  *(undefined4 *)(param_1 + 4) = 0;
  return;
}



int __fastcall FUN_100030a0(int param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021bcf;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_10003040(param_1);
  local_4 = 0;
  *(undefined4 *)(param_1 + 8) = 0;
  *(undefined4 *)(param_1 + 0xc) = 0;
  *(undefined4 *)(param_1 + 0x10) = 0;
  puVar1 = (undefined4 *)operator_new(0x20);
  local_4 = CONCAT31(local_4._1_3_,1);
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_10002cc0(puVar1);
  }
  *(undefined4 **)(param_1 + 0x14) = puVar1;
  ExceptionList = local_c;
  return param_1;
}



void FUN_10003130(void)

{
                    // WARNING: Could not recover jumptable at 0x10003135. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::Init((Init *)&DAT_100384c9);
  return;
}



void FUN_10003140(void)

{
  FUN_100214c2(FUN_10003150);
  return;
}



void FUN_10003150(void)

{
                    // WARNING: Could not recover jumptable at 0x10003155. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::~Init((Init *)&DAT_100384c9);
  return;
}



void FUN_10003170(void)

{
                    // WARNING: Could not recover jumptable at 0x10003175. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::_Winit((_Winit *)&DAT_100384c8);
  return;
}



void FUN_10003180(void)

{
  FUN_100214c2(FUN_10003190);
  return;
}



void FUN_10003190(void)

{
                    // WARNING: Could not recover jumptable at 0x10003195. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::~_Winit((_Winit *)&DAT_100384c8);
  return;
}



void * __thiscall FUN_100031a0(void *this,uint param_1)

{
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021be8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_10007740((undefined4 *)this);
  local_4 = 0;
  *(undefined ***)this = &PTR_LAB_100257f0;
  if ((int)param_1 < 8) {
    param_1 = 8;
  }
  *(undefined4 *)((int)this + 0x10) = 0;
  *(undefined4 *)((int)this + 8) = 0;
  *(uint *)((int)this + 0xc) = param_1;
  FUN_10003230(this,param_1);
  *(undefined4 *)((int)this + 4) = 0;
  ExceptionList = local_c;
  return this;
}



void __fastcall FUN_10003210(undefined4 *param_1)

{
  *param_1 = &PTR_LAB_100257f0;
  FUN_100212a4((void *)param_1[4]);
  FUN_10007750(param_1);
  return;
}



void __thiscall FUN_10003230(void *this,uint param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  uint uVar3;
  undefined4 *puVar4;
  undefined4 *puVar5;
  
  puVar1 = (undefined4 *)operator_new(param_1);
  if (*(undefined4 **)((int)this + 0x10) != (undefined4 *)0x0) {
    uVar3 = *(uint *)((int)this + 0xc);
    puVar4 = *(undefined4 **)((int)this + 0x10);
    puVar5 = puVar1;
    for (uVar2 = uVar3 >> 2; uVar2 != 0; uVar2 = uVar2 - 1) {
      *puVar5 = *puVar4;
      puVar4 = puVar4 + 1;
      puVar5 = puVar5 + 1;
    }
    for (uVar3 = uVar3 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
      *(undefined *)puVar5 = *(undefined *)puVar4;
      puVar4 = (undefined4 *)((int)puVar4 + 1);
      puVar5 = (undefined4 *)((int)puVar5 + 1);
    }
  }
  FUN_100212a4(*(void **)((int)this + 0x10));
  *(undefined4 **)((int)this + 0x10) = puVar1;
  *(uint *)((int)this + 0xc) = param_1;
  return;
}



void * __thiscall FUN_10003290(void *this,undefined4 param_1)

{
                    // WARNING: Load size is inaccurate
  (**(code **)(*this + 0x14))(param_1);
  return this;
}



void * __thiscall FUN_100032b0(void *this,undefined4 param_1)

{
                    // WARNING: Load size is inaccurate
  (**(code **)(*this + 0x10))(param_1);
  return this;
}



void * __thiscall FUN_100032d0(void *this,undefined4 param_1)

{
                    // WARNING: Load size is inaccurate
  (**(code **)(*this + 0xc))(param_1);
  return this;
}



void __thiscall FUN_10003370(void *this,undefined param_1)

{
  if (*(uint *)((int)this + 0xc) < *(int *)((int)this + 4) + 1U) {
    FUN_10003230(this,*(uint *)((int)this + 0xc) * 2);
  }
  *(undefined *)(*(int *)((int)this + 0x10) + *(int *)((int)this + 4)) = param_1;
                    // WARNING: Load size is inaccurate
  (**(code **)(*this + 0x4c))(1);
  return;
}



void __thiscall FUN_100033b0(void *this,undefined2 param_1)

{
  if (*(uint *)((int)this + 0xc) < *(int *)((int)this + 4) + 2U) {
    FUN_10003230(this,*(uint *)((int)this + 0xc) * 2);
  }
  *(undefined2 *)(*(int *)((int)this + 0x10) + *(int *)((int)this + 4)) = param_1;
                    // WARNING: Load size is inaccurate
  (**(code **)(*this + 0x4c))(2);
  return;
}



void __thiscall FUN_100033f0(void *this,undefined4 param_1)

{
  if (*(uint *)((int)this + 0xc) < *(int *)((int)this + 4) + 4U) {
    FUN_10003230(this,*(uint *)((int)this + 0xc) * 2);
  }
  *(undefined4 *)(*(int *)((int)this + 0x10) + *(int *)((int)this + 4)) = param_1;
                    // WARNING: Load size is inaccurate
  (**(code **)(*this + 0x4c))(4);
  return;
}



void * __thiscall FUN_10003430(void *this,undefined *param_1)

{
  undefined uVar1;
  
                    // WARNING: Load size is inaccurate
  uVar1 = (**(code **)(*this + 8))();
  *param_1 = uVar1;
  return this;
}



void * __thiscall FUN_10003450(void *this,undefined2 *param_1)

{
  undefined2 uVar1;
  
                    // WARNING: Load size is inaccurate
  uVar1 = (**(code **)(*this + 4))();
  *param_1 = uVar1;
  return this;
}



void * __thiscall FUN_10003470(void *this,undefined4 *param_1)

{
  undefined4 uVar1;
  
                    // WARNING: Load size is inaccurate
  uVar1 = (*(code *)**this)();
  *param_1 = uVar1;
  return this;
}



void __thiscall FUN_10003550(void *this,int param_1)

{
  if (*(int *)((int)this + 8) < param_1) {
    *(int *)((int)this + 8) = param_1;
  }
  if (*(int *)((int)this + 0xc) <= param_1) {
    FUN_10003230(this,*(int *)((int)this + 0xc) + param_1);
  }
  *(int *)((int)this + 4) = param_1;
  return;
}



void __thiscall FUN_10003580(void *this,int param_1)

{
  int iVar1;
  
  iVar1 = *(int *)((int)this + 4) + param_1;
  if (*(int *)((int)this + 8) < iVar1) {
    *(int *)((int)this + 8) = iVar1;
  }
  if (*(int *)((int)this + 0xc) <= iVar1) {
    FUN_10003230(this,*(int *)((int)this + 0xc) + param_1 + *(int *)((int)this + 8));
  }
  *(int *)((int)this + 4) = *(int *)((int)this + 4) + param_1;
  return;
}



void __thiscall FUN_100035c0(void *this,int param_1)

{
  int iVar1;
  
  iVar1 = *(int *)((int)this + 8) + param_1;
  if (*(int *)((int)this + 8) < iVar1) {
    *(int *)((int)this + 8) = iVar1;
  }
  if (*(int *)((int)this + 0xc) <= *(int *)((int)this + 8) + param_1) {
    FUN_10003230(this,*(int *)((int)this + 0xc) + *(int *)((int)this + 8) + param_1);
  }
  *(int *)((int)this + 4) = param_1 + *(int *)((int)this + 8);
  return;
}



void FUN_10003650(void)

{
                    // WARNING: Could not recover jumptable at 0x10003655. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::Init((Init *)&DAT_100384cd);
  return;
}



void FUN_10003660(void)

{
  FUN_100214c2(FUN_10003670);
  return;
}



void FUN_10003670(void)

{
                    // WARNING: Could not recover jumptable at 0x10003675. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::~Init((Init *)&DAT_100384cd);
  return;
}



void FUN_10003690(void)

{
                    // WARNING: Could not recover jumptable at 0x10003695. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::_Winit((_Winit *)&DAT_100384cc);
  return;
}



void FUN_100036a0(void)

{
  FUN_100214c2(FUN_100036b0);
  return;
}



void FUN_100036b0(void)

{
                    // WARNING: Could not recover jumptable at 0x100036b5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::~_Winit((_Winit *)&DAT_100384cc);
  return;
}



int * __cdecl FUN_100036c0(int *param_1,int param_2)

{
  int *piVar1;
  
  (**(code **)(*param_1 + 0x44))(param_2,0x40);
  piVar1 = (int *)(**(code **)(*param_1 + 0x24))(*(undefined4 *)(param_2 + 0x40));
  (**(code **)(*piVar1 + 0x24))(*(undefined4 *)(param_2 + 0x44));
  (**(code **)(*param_1 + 0x24))(*(undefined4 *)(param_2 + 0x48));
  (**(code **)(*param_1 + 0x44))(param_2 + 0x4c,8);
  (**(code **)(*param_1 + 0x44))(param_2 + 0x54,8);
  (**(code **)(*param_1 + 0x24))(*(undefined4 *)(param_2 + 0x5c));
  (**(code **)(*param_1 + 0x24))(*(undefined4 *)(param_2 + 0x60));
  (**(code **)(*param_1 + 0x24))(*(undefined4 *)(param_2 + 100));
  (**(code **)(*param_1 + 0x24))(*(undefined4 *)(param_2 + 0x68));
  (**(code **)(*param_1 + 0x44))(param_2 + 0x6c,0x20);
  (**(code **)(*param_1 + 0x44))(param_2 + 0x8c,0x20);
  (**(code **)(*param_1 + 0x44))(param_2 + 0xac,0x100);
  return param_1;
}



int * __cdecl FUN_10003770(int *param_1,int param_2)

{
  int *piVar1;
  
  (**(code **)(*param_1 + 0x40))(param_2,0x40);
  piVar1 = (int *)(**(code **)(*param_1 + 0x34))(param_2 + 0x40);
  (**(code **)(*piVar1 + 0x34))(param_2 + 0x44);
  (**(code **)(*param_1 + 0x34))(param_2 + 0x48);
  (**(code **)(*param_1 + 0x40))(param_2 + 0x4c,8);
  (**(code **)(*param_1 + 0x40))(param_2 + 0x54,8);
  (**(code **)(*param_1 + 0x34))(param_2 + 0x5c);
  (**(code **)(*param_1 + 0x34))(param_2 + 0x60);
  (**(code **)(*param_1 + 0x34))(param_2 + 100);
  (**(code **)(*param_1 + 0x34))(param_2 + 0x68);
  (**(code **)(*param_1 + 0x40))(param_2 + 0x6c,0x20);
  (**(code **)(*param_1 + 0x40))(param_2 + 0x8c,0x20);
  (**(code **)(*param_1 + 0x40))(param_2 + 0xac,0x100);
  return param_1;
}



int * __cdecl FUN_10003820(int *param_1,undefined4 *param_2)

{
  int *piVar1;
  
  (**(code **)(*param_1 + 0x24))(*param_2);
  (**(code **)(*param_1 + 0x24))(param_2[1]);
  (**(code **)(*param_1 + 0x24))(param_2[2]);
  (**(code **)(*param_1 + 0x24))(param_2[3]);
  (**(code **)(*param_1 + 0x24))(param_2[4]);
  piVar1 = (int *)(**(code **)(*param_1 + 0x24))(param_2[5]);
  (**(code **)(*piVar1 + 0x24))(param_2[6]);
  return param_1;
}



int * __cdecl FUN_10003880(int *param_1,int param_2)

{
  int *piVar1;
  
  (**(code **)(*param_1 + 0x34))(param_2);
  (**(code **)(*param_1 + 0x34))(param_2 + 4);
  (**(code **)(*param_1 + 0x34))(param_2 + 8);
  (**(code **)(*param_1 + 0x34))(param_2 + 0xc);
  (**(code **)(*param_1 + 0x34))(param_2 + 0x10);
  piVar1 = (int *)(**(code **)(*param_1 + 0x34))(param_2 + 0x14);
  (**(code **)(*piVar1 + 0x34))(param_2 + 0x18);
  return param_1;
}



void * __thiscall FUN_100038e0(void *this,undefined4 param_1)

{
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021c0b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_10001080((undefined4 *)((int)this + 8));
  local_4 = 0;
  *(undefined ***)this = &PTR_LAB_10025850;
  *(undefined4 *)((int)this + 4) = param_1;
  param_1 = 0xffffffff;
  *(undefined4 *)((int)this + 8) = 0xffffffff;
  FUN_10001090(&param_1);
  *(undefined4 *)((int)this + 0xc) = 0;
  *(undefined4 *)((int)this + 0x18) = 0;
  *(undefined4 *)((int)this + 0x1c) = 0;
  *(undefined4 *)((int)this + 0x10) = 0;
  *(undefined4 *)((int)this + 0x14) = 0;
  ExceptionList = local_c;
  return this;
}



undefined4 * __fastcall FUN_10003960(undefined4 *param_1)

{
  undefined4 local_14;
  undefined4 *local_10;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021c20;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_10 = param_1;
  FUN_10001080(param_1 + 2);
  local_4 = 0;
  *param_1 = &PTR_LAB_10025850;
  param_1[1] = 0;
  local_14 = 0xffffffff;
  param_1[2] = 0xffffffff;
  FUN_10001090(&local_14);
  param_1[3] = 0;
  param_1[6] = 0;
  param_1[7] = 0;
  param_1[4] = 0;
  param_1[5] = 0;
  ExceptionList = local_c;
  return param_1;
}



void __fastcall FUN_100039e0(undefined4 *param_1)

{
  *param_1 = &PTR_LAB_10025850;
  FUN_10001090(param_1 + 2);
  return;
}



void __thiscall FUN_100039f0(void *this,int param_1)

{
  *(undefined4 *)(param_1 + 0x18) = *(undefined4 *)((int)this + 0x1c);
  *(int *)((int)this + 0x1c) = param_1;
  return;
}



void FUN_10003a00(int *param_1,int param_2)

{
  *(int *)(param_2 + 0x18) = *param_1;
  *param_1 = param_2;
  return;
}



void __thiscall FUN_10003a50(void *this,char *param_1,undefined4 param_2)

{
  undefined4 *puVar1;
  
  puVar1 = FUN_10001290(param_1);
  FUN_10003b20(this,puVar1,param_2);
  return;
}



void __thiscall FUN_10003b20(void *this,void *param_1,undefined4 param_2)

{
  void *this_00;
  undefined4 *puVar1;
  void *pvVar2;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021c3d;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this_00 = operator_new(0x20);
  pvVar2 = (void *)0x0;
  local_4 = 0;
  if (this_00 != (void *)0x0) {
    pvVar2 = FUN_100038e0(this_00,param_2);
  }
  local_4 = 0xffffffff;
  puVar1 = (undefined4 *)FUN_10001310(param_1,&param_1);
  param_2 = *puVar1;
  local_4 = 1;
  *(undefined4 *)((int)pvVar2 + 8) = param_2;
  FUN_10001090(&param_2);
  local_4 = 0xffffffff;
  FUN_10001090(&param_1);
  FUN_100039f0(this,(int)pvVar2);
  ExceptionList = local_c;
  return;
}



void __thiscall FUN_10003bc0(void *this,undefined4 param_1,undefined4 param_2)

{
  void *this_00;
  void *pvVar1;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021c52;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  this_00 = operator_new(0x20);
  pvVar1 = (void *)0x0;
  local_4 = 0;
  if (this_00 != (void *)0x0) {
    pvVar1 = FUN_100038e0(this_00,param_2);
  }
  local_4 = 0xffffffff;
  param_2 = param_1;
  *(undefined4 *)((int)pvVar1 + 8) = param_1;
  FUN_10001090(&param_2);
  FUN_100039f0(this,(int)pvVar1);
  ExceptionList = local_c;
  return;
}



int __thiscall FUN_10003c50(void *this,int param_1)

{
  int iVar1;
  
  iVar1 = *(int *)((int)this + 0x1c);
  while( true ) {
    if (iVar1 == 0) {
      return 0;
    }
    if (*(int *)(iVar1 + 4) == param_1) break;
    iVar1 = *(int *)(iVar1 + 0x18);
  }
  return iVar1;
}



undefined4 * FUN_10003c70(void)

{
  undefined4 *puVar1;
  int *piVar2;
  int local_14;
  undefined4 *local_10;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021c86;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)operator_new(0x1c);
  local_4 = 0;
  local_10 = puVar1;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    FUN_100031a0(puVar1 + 1,0x10);
    local_4._0_1_ = 1;
    FUN_10001080(puVar1 + 6);
    local_4 = CONCAT31(local_4._1_3_,2);
    local_14 = -1;
    puVar1[6] = 0xffffffff;
    FUN_10001090(&local_14);
    *puVar1 = 0;
  }
  local_4 = 0xffffffff;
  piVar2 = FUN_100013f0(DAT_100384b4,(int *)&local_10);
  local_14 = *piVar2;
  local_4 = 3;
  puVar1[6] = local_14;
  FUN_10001090(&local_14);
  local_4 = 0xffffffff;
  FUN_10001090(&local_10);
  FUN_100011c0(DAT_100384b4,puVar1);
  ExceptionList = local_c;
  return puVar1;
}



void FUN_10003d60(void)

{
                    // WARNING: Could not recover jumptable at 0x10003d65. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::Init((Init *)&DAT_100384d1);
  return;
}



void FUN_10003d70(void)

{
  FUN_100214c2(FUN_10003d80);
  return;
}



void FUN_10003d80(void)

{
                    // WARNING: Could not recover jumptable at 0x10003d85. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::~Init((Init *)&DAT_100384d1);
  return;
}



void FUN_10003da0(void)

{
                    // WARNING: Could not recover jumptable at 0x10003da5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::_Winit((_Winit *)&DAT_100384d0);
  return;
}



void FUN_10003db0(void)

{
  FUN_100214c2(FUN_10003dc0);
  return;
}



void FUN_10003dc0(void)

{
                    // WARNING: Could not recover jumptable at 0x10003dc5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::~_Winit((_Winit *)&DAT_100384d0);
  return;
}



void __thiscall FUN_10003dd0(void *this,basic_ostream<> *param_1)

{
  basic_ostream<> *pbVar1;
  float fVar2;
  char cVar3;
  uint uVar4;
  uint uVar5;
  float fVar6;
  char cVar7;
  char *pcVar8;
  float fVar9;
  char cVar10;
  char *pcVar11;
  
  uVar5 = *(uint *)((int)this + 0xc);
  uVar4 = *(uint *)((int)this + 4);
  pcVar11 = &DAT_10035224;
  pcVar8 = s_result_vertex_count_10035228;
  pbVar1 = std::operator<<(param_1,s_Original_vertex_count_10035240);
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,uVar4);
  pbVar1 = std::operator<<(pbVar1,pcVar8);
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,uVar5);
  std::operator<<(pbVar1,pcVar11);
                    // WARNING: Load size is inaccurate
  uVar5 = *this;
  uVar4 = *(uint *)((int)this + 8);
  if (uVar5 == uVar4) {
    pcVar11 = &DAT_1003528c;
    pcVar8 = s_Polygon_count_10035290;
    pbVar1 = param_1;
  }
  else {
    pcVar11 = &DAT_10035258;
    pcVar8 = s_result_polygon_count_1003525c;
    pbVar1 = std::operator<<(param_1,s_Original_polygon_count_10035274);
    pbVar1 = std::basic_ostream<>::operator<<(pbVar1,uVar5);
    uVar5 = uVar4;
  }
  pbVar1 = std::operator<<(pbVar1,pcVar8);
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,uVar5);
  std::operator<<(pbVar1,pcVar11);
  if (*(char *)((int)this + 0x10) != '\0') {
    std::operator<<(param_1,s_Smoothing_groups_used_100352a0);
  }
  if (*(char *)((int)this + 0x11) != '\0') {
    std::operator<<(param_1,s_Uv_array_defined_100352b8);
  }
  if (*(char *)((int)this + 0x12) != '\0') {
    std::operator<<(param_1,s_Has_more_than_one_material_100352cc);
  }
  if (*(char *)((int)this + 0x13) != '\0') {
    std::operator<<(param_1,s_Has_material_array_100352e8);
  }
  if (*(char *)((int)this + 0x14) != '\0') {
    std::operator<<(param_1,s_Is_solid_model_100352fc);
  }
  if (*(char *)((int)this + 0x15) != '\0') {
    std::operator<<(param_1,s_Base_right_handed_1003530c);
  }
  pbVar1 = std::operator<<(param_1,s_Bound_min_10035320);
  fVar9 = *(float *)((int)this + 0x2c);
  fVar6 = *(float *)((int)this + 0x28);
  cVar10 = '}';
  fVar2 = *(float *)((int)this + 0x24);
  cVar7 = ',';
  cVar3 = ',';
  pbVar1 = std::operator<<(pbVar1,'{');
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar2);
  pbVar1 = std::operator<<(pbVar1,cVar3);
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar6);
  pbVar1 = std::operator<<(pbVar1,cVar7);
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar9);
  std::operator<<(pbVar1,cVar10);
  pbVar1 = std::operator<<(param_1,s_Bound_max_1003532c);
  fVar9 = *(float *)((int)this + 0x38);
  fVar6 = *(float *)((int)this + 0x34);
  cVar10 = '}';
  fVar2 = *(float *)((int)this + 0x30);
  cVar7 = ',';
  cVar3 = ',';
  pbVar1 = std::operator<<(pbVar1,'{');
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar2);
  pbVar1 = std::operator<<(pbVar1,cVar3);
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar6);
  pbVar1 = std::operator<<(pbVar1,cVar7);
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar9);
  std::operator<<(pbVar1,cVar10);
  pbVar1 = std::operator<<(param_1,s_Pivot_10035338);
  fVar9 = *(float *)((int)this + 0x20);
  fVar6 = *(float *)((int)this + 0x1c);
  cVar10 = '}';
  fVar2 = *(float *)((int)this + 0x18);
  cVar7 = ',';
  cVar3 = ',';
  pbVar1 = std::operator<<(pbVar1,'{');
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar2);
  pbVar1 = std::operator<<(pbVar1,cVar3);
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar6);
  pbVar1 = std::operator<<(pbVar1,cVar7);
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar9);
  std::operator<<(pbVar1,cVar10);
  pbVar1 = std::operator<<(param_1,s_Base_10035344);
  pbVar1 = std::operator<<(pbVar1,'{');
  fVar9 = *(float *)((int)this + 0x44);
  fVar6 = *(float *)((int)this + 0x40);
  cVar10 = '}';
  fVar2 = *(float *)((int)this + 0x3c);
  cVar7 = ',';
  cVar3 = ',';
  pbVar1 = std::operator<<(pbVar1,'{');
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar2);
  pbVar1 = std::operator<<(pbVar1,cVar3);
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar6);
  pbVar1 = std::operator<<(pbVar1,cVar7);
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar9);
  pbVar1 = std::operator<<(pbVar1,cVar10);
  pbVar1 = std::operator<<(pbVar1,',');
  fVar9 = *(float *)((int)this + 0x50);
  fVar6 = *(float *)((int)this + 0x4c);
  cVar10 = '}';
  fVar2 = *(float *)((int)this + 0x48);
  cVar7 = ',';
  cVar3 = ',';
  pbVar1 = std::operator<<(pbVar1,'{');
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar2);
  pbVar1 = std::operator<<(pbVar1,cVar3);
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar6);
  pbVar1 = std::operator<<(pbVar1,cVar7);
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar9);
  pbVar1 = std::operator<<(pbVar1,cVar10);
  pbVar1 = std::operator<<(pbVar1,',');
  fVar9 = *(float *)((int)this + 0x5c);
  fVar6 = *(float *)((int)this + 0x58);
  cVar10 = '}';
  fVar2 = *(float *)((int)this + 0x54);
  cVar7 = ',';
  cVar3 = ',';
  pbVar1 = std::operator<<(pbVar1,'{');
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar2);
  pbVar1 = std::operator<<(pbVar1,cVar3);
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar6);
  pbVar1 = std::operator<<(pbVar1,cVar7);
  pbVar1 = std::basic_ostream<>::operator<<(pbVar1,fVar9);
  pbVar1 = std::operator<<(pbVar1,cVar10);
  pcVar8 = &DAT_10035340;
  pbVar1 = std::operator<<(pbVar1,'}');
  std::operator<<(pbVar1,pcVar8);
  return;
}



void __fastcall FUN_100040c0(undefined4 *param_1)

{
  undefined4 *puVar1;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021c9b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  puVar1 = (undefined4 *)operator_new(0x40);
  local_4 = 0;
  if (puVar1 == (undefined4 *)0x0) {
    puVar1 = (undefined4 *)0x0;
  }
  else {
    puVar1 = FUN_100014a0(puVar1);
  }
  local_4 = 0xffffffff;
  *param_1 = puVar1;
  FUN_10001dc0(puVar1,(int *)param_1[1]);
  param_1[2] = 0;
  ExceptionList = local_c;
  return;
}



void * __thiscall FUN_10004130(void *this,undefined4 param_1)

{
  void *pvVar1;
  int iVar2;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021cbb;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(undefined4 *)((int)this + 0x10) = 0x100;
  pvVar1 = operator_new(0x400);
  *(void **)((int)this + 0xc) = pvVar1;
  iVar2 = 0;
  if (0 < *(int *)((int)this + 0x10)) {
    do {
      *(undefined4 *)(*(int *)((int)this + 0xc) + iVar2 * 4) = 0;
      iVar2 = iVar2 + 1;
    } while (iVar2 < *(int *)((int)this + 0x10));
  }
  *(undefined4 *)((int)this + 0x14) = 0;
  local_4 = 0;
  *(undefined4 *)((int)this + 0x1c) = 0x100;
  pvVar1 = operator_new(0x400);
  *(void **)((int)this + 0x18) = pvVar1;
  iVar2 = 0;
  if (0 < *(int *)((int)this + 0x1c)) {
    do {
      *(undefined4 *)(*(int *)((int)this + 0x18) + iVar2 * 4) = 0;
      iVar2 = iVar2 + 1;
    } while (iVar2 < *(int *)((int)this + 0x1c));
  }
  *(undefined4 *)((int)this + 0x20) = 0;
  local_4 = CONCAT31(local_4._1_3_,1);
  *(undefined4 *)((int)this + 4) = param_1;
  FUN_100040c0((undefined4 *)this);
  ExceptionList = local_c;
  return this;
}



// public: __thiscall srGFInterface::srGFInterface(char const *)

srGFInterface * __thiscall srGFInterface::srGFInterface(srGFInterface *this,char *param_1)

{
  void *pvVar1;
  int iVar2;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0x41f0  26  ??0srGFInterface@@QAE@PBD@Z
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021ce6;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(undefined4 *)(this + 0x10) = 0x100;
  pvVar1 = operator_new(0x400);
  *(void **)(this + 0xc) = pvVar1;
  iVar2 = 0;
  if (0 < *(int *)(this + 0x10)) {
    do {
      *(undefined4 *)(*(int *)(this + 0xc) + iVar2 * 4) = 0;
      iVar2 = iVar2 + 1;
    } while (iVar2 < *(int *)(this + 0x10));
  }
  *(undefined4 *)(this + 0x14) = 0;
  local_4 = 0;
  *(undefined4 *)(this + 0x1c) = 0x100;
  pvVar1 = operator_new(0x400);
  *(void **)(this + 0x18) = pvVar1;
  iVar2 = 0;
  if (0 < *(int *)(this + 0x1c)) {
    do {
      *(undefined4 *)(*(int *)(this + 0x18) + iVar2 * 4) = 0;
      iVar2 = iVar2 + 1;
    } while (iVar2 < *(int *)(this + 0x1c));
  }
  *(undefined4 *)(this + 0x20) = 0;
  local_4._0_1_ = 1;
  pvVar1 = operator_new(0xc);
  local_4._0_1_ = 2;
  if (pvVar1 == (void *)0x0) {
    pvVar1 = (void *)0x0;
  }
  else {
    pvVar1 = FUN_10007b40(pvVar1,param_1);
  }
  local_4 = CONCAT31(local_4._1_3_,1);
  *(void **)(this + 4) = pvVar1;
  FUN_100040c0((undefined4 *)this);
  ExceptionList = local_c;
  return this;
}



// public: __thiscall srGFInterface::~srGFInterface(void)

void __thiscall srGFInterface::~srGFInterface(srGFInterface *this)

{
  char *pcVar1;
  char cVar2;
  undefined4 *puVar3;
  void *pvVar4;
  undefined4 *puVar5;
  int iVar6;
  int local_10;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0x42e0  44  ??1srGFInterface@@QAE@XZ
  puStack_8 = &LAB_10021d06;
  local_c = ExceptionList;
  puVar3 = *(undefined4 **)(this + 4);
  local_4 = 1;
  ExceptionList = &local_c;
  if (puVar3 != (undefined4 *)0x0) {
    ExceptionList = &local_c;
    FUN_10007750(puVar3);
    FUN_100212a4(puVar3);
  }
  pvVar4 = *(void **)this;
  if (pvVar4 != (void *)0x0) {
    FUN_10001570((int)pvVar4);
    FUN_100212a4(pvVar4);
  }
  local_10 = 0;
  if (0 < *(int *)(this + 0x1c)) {
    do {
      puVar3 = *(undefined4 **)(*(int *)(this + 0x18) + local_10 * 4);
      while (puVar5 = puVar3, puVar5 != (undefined4 *)0x0) {
        puVar3 = (undefined4 *)puVar5[1];
        if (puVar5 != (undefined4 *)0x0) {
          pvVar4 = (void *)*puVar5;
          if (pvVar4 != (void *)0x0) {
            if (*(int *)((int)pvVar4 + 8) != 0) {
              pcVar1 = (char *)(*(int *)((int)pvVar4 + 8) + -1);
              cVar2 = *pcVar1;
              if ((cVar2 == '\0') || (cVar2 == -1)) {
                FUN_100212a4(pcVar1);
              }
              else {
                *pcVar1 = cVar2 + -1;
              }
            }
            *(undefined4 *)((int)pvVar4 + 8) = 0;
            *(undefined4 *)((int)pvVar4 + 0xc) = 0;
            *(undefined4 *)((int)pvVar4 + 0x10) = 0;
            FUN_100212a4(pvVar4);
          }
          FUN_100212a4(puVar5);
        }
      }
      local_10 = local_10 + 1;
    } while (local_10 < *(int *)(this + 0x1c));
  }
  FUN_100212a4(*(void **)(this + 0x18));
  *(undefined4 *)(this + 0x18) = 0;
  *(undefined4 *)(this + 0x20) = 0;
  iVar6 = 0;
  if (0 < *(int *)(this + 0x10)) {
    do {
      puVar3 = *(undefined4 **)(*(int *)(this + 0xc) + iVar6 * 4);
      while (puVar5 = puVar3, puVar5 != (undefined4 *)0x0) {
        puVar3 = (undefined4 *)puVar5[1];
        if (puVar5 != (undefined4 *)0x0) {
          FUN_100212a4((void *)*puVar5);
          FUN_100212a4(puVar5);
        }
      }
      iVar6 = iVar6 + 1;
    } while (iVar6 < *(int *)(this + 0x10));
  }
  FUN_100212a4(*(void **)(this + 0xc));
  *(undefined4 *)(this + 0xc) = 0;
  *(undefined4 *)(this + 0x14) = 0;
  ExceptionList = local_c;
  return;
}



int __cdecl FUN_10004480(int param_1,int param_2,char *param_3)

{
  char *_Str1;
  int iVar1;
  
  if (param_1 != 0) {
    do {
      _Str1 = (char *)FUN_10005e10(param_1);
      if (((_Str1 != (char *)0x0) && (iVar1 = _stricmp(_Str1,param_3), iVar1 == 0)) &&
         (*(int *)(param_1 + 4) == param_2)) {
        return param_1;
      }
      if (*(int *)(param_1 + 0x1c) != 0) {
        FUN_10004480(*(int *)(param_1 + 0x1c),param_2,param_3);
      }
      param_1 = *(int *)(param_1 + 0x18);
    } while (param_1 != 0);
  }
  return 0;
}



// WARNING: Removing unreachable block (ram,0x100052c4)
// WARNING: Removing unreachable block (ram,0x100052df)
// WARNING: Removing unreachable block (ram,0x100052ee)
// WARNING: Removing unreachable block (ram,0x100052f5)
// WARNING: Removing unreachable block (ram,0x1000541a)
// WARNING: Removing unreachable block (ram,0x10005435)
// WARNING: Removing unreachable block (ram,0x10005444)
// WARNING: Removing unreachable block (ram,0x1000544d)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address
// public: class srMeshModel * __fastcall srGFInterface::getModelByName(char const *,class
// std::list<class Housemarque::Threedee_Engine::Material_Node *,class std::allocator<class
// Housemarque::Threedee_Engine::Material_Node *> > const &,class reOrderData *,class
// Import_Statistics *,bool)

srMeshModel * __fastcall
srGFInterface::getModelByName
          (char *param_1,list<> *param_2,reOrderData *param_3,Import_Statistics *param_4,
          bool param_5)

{
  int *piVar1;
  Import_Statistics *pIVar2;
  Emissive_ARGB_Model *pEVar3;
  float *pfVar4;
  uint *puVar5;
  Emissive_ARGB_Model *pEVar6;
  list<> *plVar7;
  bool bVar8;
  char cVar9;
  ushort uVar10;
  void *pvVar11;
  undefined4 *puVar12;
  basic_ostream<> *pbVar13;
  char *pcVar14;
  code *pcVar15;
  int iVar16;
  undefined4 uVar17;
  int *piVar18;
  int **ppiVar19;
  undefined4 *puVar20;
  int *piVar21;
  float *pfVar22;
  code *pcVar23;
  float fVar24;
  srRuntimeClass *psVar25;
  int iVar26;
  uint uVar27;
  uint uVar28;
  uint *puVar29;
  undefined3 in_stack_0000000d;
  char in_stack_00000010;
  uint uVar30;
  char *pcVar31;
  int iVar32;
  uint uStack_430;
  srRuntimeClass *psStack_42c;
  undefined local_425;
  Emissive_ARGB_Model *pEStack_424;
  undefined local_420 [4];
  int *local_41c;
  int local_418;
  int *local_414 [2];
  uint uStack_40c;
  uint local_408;
  uint uStack_404;
  float *pfStack_400;
  char *local_3fc;
  uint uStack_3f8;
  list<> *local_3f4;
  int iStack_3f0;
  undefined uStack_3ec;
  undefined uStack_3eb;
  int *piStack_3e8;
  undefined uStack_3e4;
  undefined4 uStack_3e0;
  int *piStack_3dc;
  int iStack_3d8;
  float fStack_3d4;
  int iStack_3d0;
  int iStack_3cc;
  int iStack_3c8;
  int iStack_3c4;
  uint uStack_3c0;
  int iStack_3bc;
  float fStack_3b8;
  int iStack_3b4;
  int iStack_3b0;
  float fStack_3ac;
  int iStack_3a8;
  int iStack_3a4;
  undefined4 uStack_3a0;
  int *piStack_398;
  int iStack_394;
  srModeler local_390 [20];
  undefined4 auStack_37c [2];
  int aiStack_374 [2];
  int iStack_36c;
  int iStack_354;
  int iStack_350;
  undefined4 uStack_344;
  undefined4 uStack_340;
  undefined4 uStack_33c;
  undefined4 uStack_334;
  int aiStack_32c [42];
  undefined4 uStack_284;
  float fStack_280;
  undefined4 uStack_274;
  float fStack_270;
  undefined4 uStack_234;
  undefined4 uStack_230;
  undefined4 uStack_22c;
  undefined4 uStack_224;
  undefined4 uStack_174;
  float fStack_170;
  undefined4 uStack_164;
  float fStack_160;
  undefined4 uStack_124;
  undefined4 uStack_120;
  undefined4 uStack_11c;
  undefined4 uStack_114;
  undefined4 uStack_64;
  float fStack_60;
  undefined4 uStack_54;
  float fStack_50;
  undefined4 uStack_14;
  void *local_c;
  undefined *puStack_8;
  int local_4;
  
                    // 0x44e0  156
                    // ?getModelByName@srGFInterface@@QAIPAVsrMeshModel@@PBDABV?$list@PAVMaterial_Node@Threedee_Engine@Housemarque@@V?$allocator@PAVMaterial_Node@Threedee_Engine@Housemarque@@@std@@@std@@PAVreOrderData@@PAVImport_Statistics@@_N@Z
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021dce;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  local_3fc = param_1;
  local_3f4 = param_2;
  local_41c = (int *)FUN_10006460((undefined4 *)0x0,0);
  local_418 = 0;
  local_4 = 0;
  pvVar11 = (void *)FUN_10004480(*(int *)(*(int *)param_1 + 0xc),0x11,(char *)param_2);
  if (pvVar11 == (void *)0x0) {
    puVar12 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&local_408);
    local_4 = CONCAT31(local_4._1_3_,1);
    pbVar13 = std::operator<<((basic_ostream<> *)*puVar12,s_SR__cant_find_model_1003537c);
    cVar9 = '\0';
    pbVar13 = std::operator<<(pbVar13,(char *)param_2);
    std::operator<<(pbVar13,cVar9);
    iVar32 = 0xf6;
    pcVar31 = s_E__work_ht_3de_file_import_srGFI_1003534c;
    pcVar14 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(&DAT_10035394,pcVar14,pcVar31,iVar32);
  }
  bVar8 = false;
  local_4 = 0;
  if (pvVar11 == (void *)0x0) {
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&local_408);
    piVar18 = (int *)*local_41c;
LAB_10005677:
    local_4 = 0xffffffff;
    FUN_10006410(local_420,local_414,piVar18,local_41c);
LAB_10005686:
    FUN_100212a4(local_41c);
    ExceptionList = local_c;
    return (srMeshModel *)0x0;
  }
  pvVar11 = (void *)FUN_10003c50(pvVar11,0);
  if (pvVar11 == (void *)0x0) {
    local_4 = 0xffffffff;
    FUN_10006410(local_420,local_414,(int *)*local_41c,local_41c);
    goto LAB_10005686;
  }
  srModeler::srModeler(local_390);
  local_4._0_1_ = 2;
  FUN_10002e10(pvVar11,&uStack_40c,&iStack_3d8);
  FUN_10002dc0(pvVar11,&uStack_3f8,&iStack_3c4);
  if ((iStack_3d8 == 0) || (uStack_40c == 0)) {
    local_4 = (uint)local_4._1_3_ << 8;
    srModeler::~srModeler(local_390);
    piVar18 = (int *)*local_41c;
    goto LAB_10005677;
  }
  if ((iStack_3c4 == 0) || (uStack_3f8 == 0)) {
    local_4 = (uint)local_4._1_3_ << 8;
    srModeler::~srModeler(local_390);
    local_4 = 0xffffffff;
    FUN_10006410(local_420,local_414,(int *)*local_41c,local_41c);
    goto LAB_10005686;
  }
  FUN_10002e60(pvVar11,&uStack_3c0,&iStack_3c8);
  FUN_10002ff0(pvVar11,&uStack_3c0,&iStack_3bc);
  FUN_10002f10(pvVar11,&uStack_3c0,&pfStack_400);
  iVar32 = 0;
  iStack_3f0 = 0;
  if ((pfStack_400 != (float *)0x0) && (fVar24 = *pfStack_400, 1 < (int)uStack_3f8)) {
    iVar26 = uStack_3f8 - 1;
    pfVar22 = pfStack_400;
    do {
      pfVar22 = pfVar22 + 1;
      if (*pfVar22 != fVar24) {
        iVar32 = iVar32 + 1;
        fVar24 = *pfVar22;
      }
      iVar26 = iVar26 + -1;
      iStack_3f0 = iVar32;
    } while (iVar26 != 0);
  }
  srModeler::Triangle::Triangle((Triangle *)aiStack_374);
  uStack_430 = 0;
  if (0 < (int)uStack_3f8) {
    iVar32 = 0;
    uStack_404 = 0;
LAB_10004719:
    uVar27 = uStack_404;
    uStack_33c = *(undefined4 *)(iStack_3d8 + 8 + *(int *)(iStack_3c4 + uStack_404) * 0xc);
    puVar12 = (undefined4 *)(iStack_3d8 + *(int *)(iStack_3c4 + uStack_404) * 0xc);
    uStack_340 = puVar12[1];
    uStack_344 = *puVar12;
    puVar12 = (undefined4 *)(iStack_3d8 + *(int *)(iStack_3c4 + 4 + uStack_404) * 0xc);
    uStack_22c = puVar12[2];
    uStack_230 = puVar12[1];
    uStack_234 = *puVar12;
    puVar12 = (undefined4 *)(iStack_3d8 + *(int *)(iStack_3c4 + 8 + uStack_404) * 0xc);
    uStack_11c = puVar12[2];
    uStack_120 = puVar12[1];
    uStack_124 = *puVar12;
    uStack_334 = 0;
    uStack_224 = 0;
    uStack_114 = 0;
    if (((pfStack_400 != (float *)0x0) && (iStack_3f0 != 0)) && (1 < *(uint *)(param_3 + 8))) {
      iStack_3a8 = CONCAT31(iStack_3a8._1_3_,local_425);
      std::basic_string<>::_Tidy((basic_string<> *)&iStack_3a8,false);
      fStack_3ac = pfStack_400[uStack_430];
      piVar18 = *(int **)(*(int *)(local_3fc + 0x18) +
                         (*(int *)(local_3fc + 0x1c) - 1U & (int)fStack_3ac >> 2) * 4);
      local_4._0_1_ = 3;
      for (; piVar18 != (int *)0x0; piVar18 = (int *)piVar18[1]) {
        if (*(float *)*piVar18 == fStack_3ac) {
          iVar26 = *piVar18;
          if (iVar26 != 0) {
            psStack_42c = (srRuntimeClass *)0x0;
            piVar18 = (int *)**(int **)(param_3 + 4);
            if (piVar18 != *(int **)(param_3 + 4)) goto LAB_1000486f;
            goto LAB_100048d3;
          }
          break;
        }
      }
      uStack_334 = *(undefined4 *)(local_3fc + 8);
      uStack_224 = uStack_334;
      uStack_114 = uStack_334;
      goto LAB_10004943;
    }
    goto LAB_1000495c;
  }
LAB_10004a61:
  pEStack_424 = (Emissive_ARGB_Model *)srHeap::allocate((srHeap *)srHeap_exref,0x3c0);
  local_4._0_1_ = 4;
  if (pEStack_424 == (Emissive_ARGB_Model *)0x0) {
    psStack_42c = (srRuntimeClass *)0x0;
  }
  else {
    psStack_42c = (srRuntimeClass *)Emissive_ARGB_Model::Emissive_ARGB_Model(pEStack_424,0,0);
  }
  psVar25 = psStack_42c;
  local_4._0_1_ = 2;
  srRuntimeClass::setName(psStack_42c,(char *)local_3f4);
  if (local_418 != 0) {
    piVar18 = (int *)*local_41c;
    uVar27 = 0;
    uVar28 = 2000;
    if (piVar18 == local_41c) {
LAB_10004ae9:
      bVar8 = true;
      puVar12 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&local_408);
      local_4 = CONCAT31(local_4._1_3_,5);
      pbVar13 = std::operator<<((basic_ostream<> *)*puVar12,
                                s_At_a_moment_SR_doesnt_support_ob_100353dc);
      cVar9 = '\0';
      pcVar31 = s_min_100353cc;
      pcVar14 = s_max_100353d4;
      uVar30 = uVar27;
      pbVar13 = std::operator<<(pbVar13,(char *)local_3f4);
      pbVar13 = std::operator<<(pbVar13,pcVar14);
      pbVar13 = std::basic_ostream<>::operator<<(pbVar13,uVar30);
      pbVar13 = std::operator<<(pbVar13,pcVar31);
      pbVar13 = std::basic_ostream<>::operator<<(pbVar13,uVar28);
      std::operator<<(pbVar13,cVar9);
      iVar32 = 0x17f;
      pcVar31 = s_E__work_ht_3de_file_import_srGFI_1003539c;
      pcVar14 = Housemarque::Kernel::Error::Tmp_Str();
      Housemarque::Kernel::Throw_Error
                (s_max_pass_count____min_pass_count_10035458,pcVar14,pcVar31,iVar32);
    }
    else {
      do {
        uVar30 = *(uint *)(piVar18[2] + 0x30);
        if (uVar27 < uVar30) {
          uVar27 = uVar30;
        }
        if (uVar30 < uVar28) {
          uVar28 = uVar30;
        }
        piVar18 = (int *)*piVar18;
      } while (piVar18 != local_41c);
      if (uVar27 != uVar28) goto LAB_10004ae9;
    }
    local_4._0_1_ = 2;
    local_4._1_3_ = 0;
    if (bVar8) {
      Housemarque::Kernel::Inserter::~Inserter((Inserter *)&local_408);
    }
    srModeler::setPassCount(local_390,uVar27);
  }
  srModeler::convert(local_390,(srMeshModel *)psVar25,1);
  if (((*(int *)(param_3 + 8) == 0) || (pfStack_400 == (float *)0x0)) || (iStack_3f0 != 0)) {
    srMeshModel::setMaterial((srMeshModel *)psVar25,*(srMaterialIFace **)(local_3fc + 8),0,0);
    goto LAB_10004db2;
  }
  iVar32 = 0;
  iStack_3a8 = CONCAT31(iStack_3a8._1_3_,local_425);
  std::basic_string<>::_Tidy((basic_string<> *)&iStack_3a8,false);
  fStack_3ac = *pfStack_400;
  piVar18 = *(int **)(*(int *)(local_3fc + 0x18) +
                     (*(int *)(local_3fc + 0x1c) - 1U & (int)fStack_3ac >> 2) * 4);
  local_4._0_1_ = 6;
  for (; piVar18 != (int *)0x0; piVar18 = (int *)piVar18[1]) {
    if (*(float *)*piVar18 == fStack_3ac) {
      iVar26 = *piVar18;
      psVar25 = psStack_42c;
      if (iVar26 != 0) {
        piVar18 = (int *)**(int **)(param_3 + 4);
        if (piVar18 != *(int **)(param_3 + 4)) goto LAB_10004c59;
        goto LAB_10004c96;
      }
      break;
    }
  }
  goto LAB_10004d6f;
LAB_1000486f:
  do {
    pcVar15 = *(code **)(iVar26 + 8);
    pcVar23 = _C_exref;
    if (pcVar15 != (code *)0x0) {
      pcVar23 = pcVar15;
    }
    pcVar15 = *(code **)(piVar18[2] + 0xc);
    if (*(code **)(piVar18[2] + 0xc) == (code *)0x0) {
      pcVar15 = _C_exref;
    }
    iVar16 = _stricmp((char *)pcVar15,(char *)pcVar23);
    if (iVar16 == 0) {
      psStack_42c = (srRuntimeClass *)piVar18[2];
      FUN_100063b0(local_420,&pEStack_424,local_41c,&psStack_42c);
    }
    piVar18 = (int *)*piVar18;
  } while (piVar18 != (int *)*(int *)(param_3 + 4));
  if (psStack_42c == (srRuntimeClass *)0x0) {
LAB_100048d3:
    psStack_42c = *(srRuntimeClass **)(**(int **)(param_3 + 4) + 8);
    if (psStack_42c == (srRuntimeClass *)0x0) goto LAB_10004943;
  }
  piVar18 = aiStack_32c;
  iVar26 = 3;
  do {
    if (*(int *)(psStack_42c + 0x18) != 0) {
      piVar18[-2] = *(int *)(psStack_42c + 0x18);
    }
    if (*(int *)(psStack_42c + 0x1c) != 0) {
      *piVar18 = *(int *)(psStack_42c + 0x1c);
    }
    if (*(int *)(psStack_42c + 0x28) != 0) {
      iStack_354 = *(int *)(psStack_42c + 0x28);
    }
    if (*(int *)(psStack_42c + 0x2c) != 0) {
      iStack_350 = *(int *)(psStack_42c + 0x2c);
    }
    if (*(int *)(psStack_42c + 0x20) != 0) {
      aiStack_374[0] = *(int *)(psStack_42c + 0x20);
    }
    if (*(int *)(psStack_42c + 0x24) != 0) {
      iStack_36c = *(int *)(psStack_42c + 0x24);
    }
    piVar18 = piVar18 + 0x44;
    iVar26 = iVar26 + -1;
  } while (iVar26 != 0);
LAB_10004943:
  local_4._0_1_ = 2;
  std::basic_string<>::_Tidy((basic_string<> *)&iStack_3a8,true);
LAB_1000495c:
  if (iStack_3c8 != 0) {
    puVar12 = (undefined4 *)(iVar32 + 8 + iStack_3c8);
    fStack_280 = -*(float *)(iVar32 + 4 + iStack_3c8);
    pfVar22 = (float *)(iVar32 + 0xc + iStack_3c8);
    uStack_284 = *(undefined4 *)(iVar32 + iStack_3c8);
    puVar20 = (undefined4 *)(iVar32 + 0x10 + iStack_3c8);
    uStack_174 = *puVar12;
    fStack_170 = -*pfVar22;
    pfVar4 = (float *)(iVar32 + 0x14 + iStack_3c8);
    uStack_64 = *puVar20;
    fStack_60 = -*pfVar4;
    fStack_270 = -*(float *)(iVar32 + 4 + iStack_3c8);
    uStack_274 = *(undefined4 *)(iVar32 + iStack_3c8);
    uStack_164 = *puVar12;
    fStack_160 = -*pfVar22;
    uStack_54 = *puVar20;
    fStack_50 = -*pfVar4;
    uVar27 = uStack_404;
  }
  if ((iStack_3bc == 0) || (in_stack_00000010 != '\0')) {
    uStack_14 = 0xffffffff;
  }
  else {
    uStack_14 = *(undefined4 *)(iStack_3bc + uStack_430 * 4);
  }
  srModeler::addTriangle(local_390,(Triangle *)aiStack_374);
  uStack_430 = uStack_430 + 1;
  uStack_404 = uVar27 + 0xc;
  iVar32 = iVar32 + 0x18;
  if ((int)uStack_3f8 <= (int)uStack_430) goto LAB_10004a61;
  goto LAB_10004719;
LAB_10004c59:
  do {
    pcVar15 = *(code **)(iVar26 + 8);
    pcVar23 = _C_exref;
    if (pcVar15 != (code *)0x0) {
      pcVar23 = pcVar15;
    }
    pcVar15 = *(code **)(piVar18[2] + 0xc);
    if (*(code **)(piVar18[2] + 0xc) == (code *)0x0) {
      pcVar15 = _C_exref;
    }
    iVar16 = _stricmp((char *)pcVar15,(char *)pcVar23);
    if (iVar16 == 0) {
      iVar32 = piVar18[2];
    }
    piVar18 = (int *)*piVar18;
  } while (piVar18 != (int *)*(int *)(param_3 + 4));
  if (iVar32 == 0) {
LAB_10004c96:
    iVar32 = *(int *)(**(int **)(param_3 + 4) + 8);
    psVar25 = psStack_42c;
    if (iVar32 == 0) goto LAB_10004d6f;
  }
  if (*(srMaterialIFace **)(iVar32 + 0x18) != (srMaterialIFace *)0x0) {
    srMeshModel::setMaterial((srMeshModel *)psStack_42c,*(srMaterialIFace **)(iVar32 + 0x18),0,0);
  }
  if (*(srMaterialIFace **)(iVar32 + 0x1c) != (srMaterialIFace *)0x0) {
    srMeshModel::setMaterial((srMeshModel *)psStack_42c,*(srMaterialIFace **)(iVar32 + 0x1c),1,0);
  }
  if (*(int *)(iVar32 + 0x28) != 0) {
    pEStack_424 = (Emissive_ARGB_Model *)&stack0xfffffbb4;
    FUN_10005ed0(&stack0xfffffbb4,(undefined4 *)(iVar32 + 0x28));
    srMeshModel::setShader((srMeshModel *)psStack_42c);
  }
  if (*(int *)(iVar32 + 0x2c) != 0) {
    pEStack_424 = (Emissive_ARGB_Model *)&stack0xfffffbb4;
    FUN_10005ed0(&stack0xfffffbb4,(undefined4 *)(iVar32 + 0x2c));
    srMeshModel::setShader((srMeshModel *)psStack_42c);
  }
  if (*(srTextureIFace **)(iVar32 + 0x20) != (srTextureIFace *)0x0) {
    srMeshModel::setTexture((srMeshModel *)psStack_42c,*(srTextureIFace **)(iVar32 + 0x20),0,0);
  }
  if (*(srTextureIFace **)(iVar32 + 0x24) != (srTextureIFace *)0x0) {
    srMeshModel::setTexture((srMeshModel *)psStack_42c,*(srTextureIFace **)(iVar32 + 0x24),1,0);
  }
  iVar32 = *(int *)(iVar32 + 0x30);
  *(int *)(psStack_42c + 0x228) = iVar32;
  psVar25 = psStack_42c;
  if (iVar32 < 1) {
    *(undefined4 *)(psStack_42c + 0x228) = 1;
  }
  else if (4 < iVar32) {
    *(undefined4 *)(psStack_42c + 0x228) = 4;
  }
LAB_10004d6f:
  local_4._0_1_ = 2;
  if (iStack_3a4 != 0) {
    pcVar14 = (char *)(iStack_3a4 + -1);
    cVar9 = *pcVar14;
    if ((cVar9 == '\0') || (cVar9 == -1)) {
      FUN_100212a4(pcVar14);
    }
    else {
      *pcVar14 = cVar9 + -1;
    }
  }
LAB_10004db2:
  if (param_4 != (Import_Statistics *)0x0) {
    uVar27 = *(uint *)((srMeshModel *)psVar25 + 0x22c);
    local_408 = uVar27;
    pEStack_424 = (Emissive_ARGB_Model *)srMeshModel::getVertexLoc((srMeshModel *)psVar25);
    uStack_3ec = local_425;
    uStack_3eb = local_425;
    uStack_3e4 = 0;
    FUN_10006cb0((int)&uStack_3ec);
    local_4 = CONCAT31(local_4._1_3_,7);
    uStack_404 = 0xbeffa;
    if (0xbeff9 < uStack_40c) {
      Housemarque::Kernel::Throw_Error
                ((char *)0x0,s_nVx_<_empty_slot_100354ac,s_E__work_ht_3de_file_import_srGFI_1003547c
                 ,0x1c0);
    }
    if (uStack_404 <= uVar27) {
      Housemarque::Kernel::Throw_Error
                ((char *)0x0,s_new_vcount_<_empty_slot_100354f0,
                 s_E__work_ht_3de_file_import_srGFI_100354c0,0x1c1);
    }
    bVar8 = Housemarque::Kernel::Logging_Enabled();
    pcVar15 = operator<<_exref;
    if (bVar8) {
      iVar32 = Housemarque::Kernel::Error_Log(local_414);
      pcVar15 = operator<<_exref;
      local_4 = CONCAT31(local_4._1_3_,8);
      pbVar13 = std::operator<<(*(basic_ostream<> **)(iVar32 + 4),s_orig_v_count_1003551c);
      pcVar31 = &DAT_10035508;
      pcVar14 = s_new_v_count_1003550c;
      pbVar13 = std::basic_ostream<>::operator<<(pbVar13,uStack_40c);
      pbVar13 = std::operator<<(pbVar13,pcVar14);
      pbVar13 = std::basic_ostream<>::operator<<(pbVar13,uVar27);
      std::operator<<(pbVar13,pcVar31);
    }
    local_4 = 7;
    if (bVar8) {
      Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter((Error_Log_Inserter *)local_414);
    }
    bVar8 = false;
    uStack_430 = 0;
    if (uStack_40c != 0) {
      do {
        iStack_3cc = *(int *)(iStack_3d8 + 8 + uStack_430 * 0xc);
        fStack_3d4 = *(float *)(iStack_3d8 + uStack_430 * 0xc);
        iStack_3d0 = *(int *)(iStack_3d8 + 4 + uStack_430 * 0xc);
        fStack_3b8 = fStack_3d4;
        iStack_3b4 = iStack_3d0;
        iStack_3b0 = iStack_3cc;
        piStack_398 = FUN_10007070(&uStack_3ec,&fStack_3b8);
        if ((piStack_398 == piStack_3e8) ||
           (uVar10 = FUN_10005ee0((int)&fStack_3b8,(float *)(piStack_398 + 3)), (char)uVar10 != '\0'
           )) {
          piStack_3dc = piStack_3e8;
          ppiVar19 = &piStack_3dc;
        }
        else {
          ppiVar19 = &piStack_398;
        }
        if (*ppiVar19 != piStack_3e8) {
          bVar8 = true;
          Housemarque::Kernel::Error::Tmp_Str_Inserter(local_414);
          local_4 = CONCAT31(local_4._1_3_,9);
          uVar17 = (*pcVar15)();
          cVar9 = '\0';
          pbVar13 = (basic_ostream<> *)(*pcVar15)(uVar17);
          std::operator<<(pbVar13,cVar9);
          iVar32 = 0x1cb;
          pcVar31 = s_E__work_ht_3de_file_import_srGFI_1003552c;
          pcVar14 = Housemarque::Kernel::Error::Tmp_Str();
          Housemarque::Kernel::Throw_Error(s_i__vm_end___100355d8,pcVar14,pcVar31,iVar32);
        }
        local_4 = 7;
        if (bVar8) {
          bVar8 = false;
          Housemarque::Kernel::Inserter::~Inserter((Inserter *)local_414);
        }
        fStack_3ac = fStack_3b8;
        iStack_3a8 = iStack_3b4;
        iStack_3a4 = iStack_3b0;
        uStack_3a0 = 0;
        piVar18 = (int *)FUN_100065b0(&uStack_3ec,auStack_37c,&fStack_3ac);
        iStack_394 = piVar18[1];
        *(uint *)(*piVar18 + 0x18) = uStack_430;
        uStack_430 = uStack_430 + 1;
      } while (uStack_430 < uStack_40c);
    }
    uStack_430 = 0;
    if (uStack_40c != 0) {
      do {
        FUN_10006130(param_4,*(undefined4 **)(param_4 + 8),1,&uStack_404);
        uStack_430 = uStack_430 + 1;
      } while (uStack_430 < uStack_40c);
    }
    uStack_430 = 0;
    if (local_408 != 0) {
      do {
        pEVar6 = pEStack_424;
        pEVar3 = pEStack_424 + uStack_430 * 0xc;
        fStack_3d4 = *(float *)pEVar3;
        iStack_3d0 = *(int *)(pEVar3 + 4);
        iStack_3cc = *(int *)(pEVar3 + 8);
        local_414[0] = FUN_10007070(&uStack_3ec,&fStack_3d4);
        if ((local_414[0] == piStack_3e8) ||
           (uVar10 = FUN_10005ee0((int)&fStack_3d4,(float *)(local_414[0] + 3)),
           (char)uVar10 != '\0')) {
          piStack_3dc = piStack_3e8;
          ppiVar19 = &piStack_3dc;
        }
        else {
          ppiVar19 = local_414;
        }
        piVar18 = *ppiVar19;
        if (piVar18 == piStack_3e8) {
          Housemarque::Kernel::Throw_Error
                    ((char *)0x0,s_i____vm_end___10035614,
                     s_E__work_ht_3de_file_import_srGFI_100355e4,0x1d5);
        }
        if ((((float)piVar18[3] != *(float *)(pEVar6 + uStack_430 * 0xc)) ||
            ((float)piVar18[4] != *(float *)(pEVar6 + uStack_430 * 0xc + 4))) ||
           ((float)piVar18[5] != *(float *)(pEVar6 + uStack_430 * 0xc + 8))) {
          Housemarque::Kernel::Throw_Error
                    ((char *)0x0,s___i__first_loc____new_vlist_v__10035654,
                     s_E__work_ht_3de_file_import_srGFI_10035624,0x1d7);
        }
        piVar1 = piVar18 + 6;
        if (*(uint *)(*(int *)(param_4 + 4) + piVar18[6] * 4) == uStack_404) {
          *(uint *)(*(int *)(param_4 + 4) + piVar18[6] * 4) = uStack_430;
        }
        else {
          puVar29 = *(uint **)(param_4 + 0x28);
          pIVar2 = param_4 + 0x20;
          if (*(int *)(param_4 + 0x2c) - (int)puVar29 >> 2 == 0) {
            uVar27 = FUN_10005fe0((int)pIVar2);
            if (uVar27 < 2) {
              iVar32 = 1;
            }
            else {
              iVar32 = FUN_10005fe0((int)pIVar2);
            }
            iVar26 = FUN_10005fe0((int)pIVar2);
            local_414[0] = (int *)(iVar26 + iVar32);
            piVar18 = local_414[0];
            if ((int)local_414[0] < 0) {
              piVar18 = (int *)0x0;
            }
            puVar12 = (undefined4 *)operator_new((int)piVar18 * 4);
            puVar20 = FUN_10006350(*(undefined4 **)(param_4 + 0x24),puVar29,puVar12);
            FUN_10006380(puVar20,1,&uStack_430);
            FUN_10006350(puVar29,*(undefined4 **)(param_4 + 0x28),puVar20 + 1);
            FUN_10006340();
            FUN_100212a4(*(void **)(param_4 + 0x24));
            *(undefined4 **)(param_4 + 0x2c) = puVar12 + (int)local_414[0];
            iVar32 = FUN_10005fe0((int)pIVar2);
            *(undefined4 **)(param_4 + 0x24) = puVar12;
            *(undefined4 **)(param_4 + 0x28) = puVar12 + iVar32 + 1;
          }
          else {
            FUN_10006350(puVar29,puVar29,puVar29 + 1);
            FUN_10006380(*(undefined4 **)(param_4 + 0x28),
                         1 - ((int)*(undefined4 **)(param_4 + 0x28) - (int)puVar29 >> 2),&uStack_430
                        );
            puVar5 = *(uint **)(param_4 + 0x28);
            for (; puVar29 != puVar5; puVar29 = puVar29 + 1) {
              *puVar29 = uStack_430;
            }
            *(int *)(param_4 + 0x28) = *(int *)(param_4 + 0x28) + 4;
          }
          piVar18 = *(int **)(param_4 + 0x18);
          pIVar2 = param_4 + 0x10;
          if (*(int *)(param_4 + 0x1c) - (int)piVar18 >> 2 == 0) {
            if ((*(int *)(param_4 + 0x14) == 0) ||
               ((uint)((int)piVar18 - *(int *)(param_4 + 0x14) >> 2) < 2)) {
              iVar32 = 1;
            }
            else {
              iVar32 = FUN_10005fe0((int)pIVar2);
            }
            iVar26 = FUN_10005fe0((int)pIVar2);
            local_414[0] = (int *)(iVar26 + iVar32);
            piVar21 = local_414[0];
            if ((int)local_414[0] < 0) {
              piVar21 = (int *)0x0;
            }
            puVar12 = (undefined4 *)operator_new((int)piVar21 << 2);
            puVar20 = FUN_10006350(*(undefined4 **)(param_4 + 0x14),piVar18,puVar12);
            FUN_10006380(puVar20,1,piVar1);
            FUN_10006350(piVar18,*(undefined4 **)(param_4 + 0x18),puVar20 + 1);
            FUN_10006340();
            FUN_100212a4(*(void **)(param_4 + 0x14));
            *(undefined4 **)(param_4 + 0x1c) = puVar12 + (int)local_414[0];
            iVar32 = FUN_10005fe0((int)pIVar2);
            *(undefined4 **)(param_4 + 0x14) = puVar12;
            *(undefined4 **)(param_4 + 0x18) = puVar12 + iVar32 + 1;
          }
          else {
            FUN_10006350(piVar18,piVar18,piVar18 + 1);
            FUN_10006380(*(undefined4 **)(param_4 + 0x18),
                         1 - ((int)*(undefined4 **)(param_4 + 0x18) - (int)piVar18 >> 2),piVar1);
            piVar21 = *(int **)(param_4 + 0x18);
            for (; piVar18 != piVar21; piVar18 = piVar18 + 1) {
              *piVar18 = *piVar1;
            }
            *(int *)(param_4 + 0x18) = *(int *)(param_4 + 0x18) + 4;
          }
        }
        uStack_430 = uStack_430 + 1;
      } while (uStack_430 < local_408);
    }
    local_4._0_1_ = 2;
    FUN_10006490(&uStack_3ec,&pEStack_424,(int *)*piStack_3e8,piStack_3e8);
    FUN_100212a4(piStack_3e8);
    piStack_3e8 = (int *)0x0;
    uStack_3e0 = 0;
    std::_Lockit::_Lockit((_Lockit *)&pEStack_424);
    _DAT_100384d8 = _DAT_100384d8 + -1;
    if (_DAT_100384d8 == 0) {
      FUN_100212a4(DAT_100384d4);
      DAT_100384d4 = (void *)0x0;
    }
    std::_Lockit::~_Lockit((_Lockit *)&pEStack_424);
    psVar25 = psStack_42c;
  }
  if (_param_5 != (uint *)0x0) {
    if (iStack_3f0 == 0) {
      *(undefined *)((int)_param_5 + 0x12) = 0;
    }
    else {
      *(undefined *)((int)_param_5 + 0x12) = 1;
    }
    if (pfStack_400 == (float *)0x0) {
      *(undefined *)((int)_param_5 + 0x13) = 0;
    }
    else {
      *(undefined *)((int)_param_5 + 0x13) = 1;
    }
    if (iStack_3c8 == 0) {
      *(undefined *)((int)_param_5 + 0x11) = 0;
    }
    else {
      *(undefined *)((int)_param_5 + 0x11) = 1;
    }
    if (iStack_3bc == 0) {
      *(undefined *)(_param_5 + 4) = 0;
    }
    else {
      *(undefined *)(_param_5 + 4) = 1;
    }
    cVar9 = FUN_1001ac40((srMeshModel *)psVar25);
    if (cVar9 == '\0') {
      *(undefined *)(_param_5 + 5) = 0;
    }
    else {
      *(undefined *)(_param_5 + 5) = 1;
    }
    _param_5[1] = uStack_40c;
    *_param_5 = uStack_3f8;
    _param_5[2] = *(uint *)((srMeshModel *)psVar25 + 0x230);
    _param_5[3] = *(uint *)((srMeshModel *)psVar25 + 0x22c);
    (**(code **)(*(int *)psVar25 + 0x2c))();
    plVar7 = local_3f4;
    FUN_10005770(local_3fc,(char *)local_3f4,_param_5 + 6);
    FUN_100058f0(local_3fc,(char *)plVar7,_param_5 + 0xf);
    iVar32 = FUN_100056b0((float *)(_param_5 + 0xf));
    if ((char)iVar32 == '\0') {
      *(undefined *)((int)_param_5 + 0x15) = 0;
    }
    else {
      *(undefined *)((int)_param_5 + 0x15) = 1;
    }
  }
  local_4 = (uint)local_4._1_3_ << 8;
  srModeler::~srModeler(local_390);
  piVar1 = local_41c;
  piVar18 = (int *)*local_41c;
  while (piVar18 != piVar1) {
    piVar21 = (int *)*piVar18;
    *(int *)piVar18[1] = *piVar18;
    *(int *)(*piVar18 + 4) = piVar18[1];
    FUN_100212a4(piVar18);
    local_418 = local_418 + -1;
    piVar18 = piVar21;
  }
  FUN_100212a4(local_41c);
  ExceptionList = local_c;
  return (srMeshModel *)psVar25;
}



int __cdecl FUN_100056b0(float *param_1)

{
  float *pfVar1;
  uint3 uVar2;
  float fVar3;
  float local_24;
  float local_20;
  float local_1c;
  float local_18;
  float local_14;
  float local_10;
  float local_c;
  float local_8;
  float local_4;
  
  local_c = *param_1;
  local_8 = param_1[3];
  local_4 = param_1[6];
  local_18 = param_1[1];
  local_14 = param_1[4];
  local_10 = param_1[7];
  local_24 = param_1[2];
  local_20 = param_1[5];
  local_1c = param_1[8];
  pfVar1 = &local_24;
  fVar3 = Housemarque::Template_Library::V3_V3_Cross_V3_Dot(&local_c,&local_18,pfVar1);
  uVar2 = (uint3)(CONCAT22((short)((uint)pfVar1 >> 0x10),
                           (ushort)(fVar3 < 0.0) << 8 | (ushort)NAN(fVar3) << 10 |
                           (ushort)(fVar3 == 0.0) << 0xe) >> 8);
  if (fVar3 >= 0.0 && (fVar3 == 0.0) == 0) {
    return CONCAT31(uVar2,1);
  }
  return (uint)uVar2 << 8;
}



void __thiscall FUN_10005770(void *this,char *param_1,undefined4 *param_2)

{
  undefined4 *puVar1;
  basic_ostream<> *pbVar2;
  char *pcVar3;
  char *pcVar4;
  bool bVar5;
  char *pcVar6;
  char cVar7;
  int iVar8;
  void *local_14;
  int iStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  pcVar4 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021e08;
  pvStack_c = ExceptionList;
                    // WARNING: Load size is inaccurate
  ExceptionList = &pvStack_c;
  local_14 = (void *)FUN_10004480(*(int *)(*this + 0xc),0x11,param_1);
  bVar5 = local_14 == (void *)0x0;
  if (bVar5) {
    puVar1 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&param_1);
    local_4 = 0;
    pbVar2 = std::operator<<((basic_ostream<> *)*puVar1,s_SR__cant_find_model_100356a4);
    cVar7 = '\0';
    pbVar2 = std::operator<<(pbVar2,pcVar4);
    std::operator<<(pbVar2,cVar7);
    iVar8 = 0x219;
    pcVar6 = s_E__work_ht_3de_file_import_srGFI_10035674;
    pcVar3 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(&DAT_100356bc,pcVar3,pcVar6,iVar8);
  }
  local_4 = 0xffffffff;
  if (bVar5) {
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&param_1);
  }
  param_1 = (char *)FUN_10003c50(local_14,0);
  bVar5 = param_1 == (char *)0x0;
  if (bVar5) {
    puVar1 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&local_14);
    local_4 = 1;
    pbVar2 = std::operator<<((basic_ostream<> *)*puVar1,s_SR__cant_find_model_100356f4);
    cVar7 = '\0';
    pbVar2 = std::operator<<(pbVar2,pcVar4);
    std::operator<<(pbVar2,cVar7);
    iVar8 = 0x21b;
    pcVar3 = s_E__work_ht_3de_file_import_srGFI_100356c4;
    pcVar4 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(&DAT_1003570c,pcVar4,pcVar3,iVar8);
  }
  local_4 = 0xffffffff;
  if (bVar5) {
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&local_14);
  }
  FUN_10002ed0(param_1,&iStack_10);
  puVar1 = (undefined4 *)(iStack_10 + 0xc);
  iVar8 = 3;
  do {
    *param_2 = *puVar1;
    puVar1 = puVar1 + 4;
    param_2 = param_2 + 1;
    iVar8 = iVar8 + -1;
  } while (iVar8 != 0);
  ExceptionList = pvStack_c;
  return;
}



void __thiscall FUN_100058f0(void *this,char *param_1,undefined4 *param_2)

{
  undefined4 *puVar1;
  basic_ostream<> *pbVar2;
  char *pcVar3;
  char *pcVar4;
  int iVar5;
  bool bVar6;
  char *pcVar7;
  char cVar8;
  int iVar9;
  void *local_14;
  undefined4 *puStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  pcVar4 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021e42;
  pvStack_c = ExceptionList;
                    // WARNING: Load size is inaccurate
  ExceptionList = &pvStack_c;
  local_14 = (void *)FUN_10004480(*(int *)(*this + 0xc),0x11,param_1);
  bVar6 = local_14 == (void *)0x0;
  if (bVar6) {
    puVar1 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&param_1);
    local_4 = 0;
    pbVar2 = std::operator<<((basic_ostream<> *)*puVar1,s_SR__cant_find_model_10035740);
    cVar8 = '\0';
    pbVar2 = std::operator<<(pbVar2,pcVar4);
    std::operator<<(pbVar2,cVar8);
    iVar9 = 0x229;
    pcVar7 = s_E__work_ht_3de_file_import_srGFI_10035710;
    pcVar3 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(&DAT_10035758,pcVar3,pcVar7,iVar9);
  }
  local_4 = 0xffffffff;
  if (bVar6) {
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&param_1);
  }
  param_1 = (char *)FUN_10003c50(local_14,0);
  bVar6 = param_1 == (char *)0x0;
  if (bVar6) {
    puVar1 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&local_14);
    local_4 = 1;
    pbVar2 = std::operator<<((basic_ostream<> *)*puVar1,s_SR__cant_find_model_10035790);
    cVar8 = '\0';
    pbVar2 = std::operator<<(pbVar2,pcVar4);
    std::operator<<(pbVar2,cVar8);
    iVar9 = 0x22b;
    pcVar3 = s_E__work_ht_3de_file_import_srGFI_10035760;
    pcVar4 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(&DAT_100357a8,pcVar4,pcVar3,iVar9);
  }
  local_4 = 0xffffffff;
  if (bVar6) {
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&local_14);
  }
  FUN_10002ed0(param_1,&puStack_10);
  iVar9 = 3;
  do {
    iVar5 = 3;
    puVar1 = puStack_10;
    do {
      *param_2 = *puVar1;
      puVar1 = puVar1 + 1;
      param_2 = param_2 + 1;
      iVar5 = iVar5 + -1;
    } while (iVar5 != 0);
    puStack_10 = puStack_10 + 4;
    iVar9 = iVar9 + -1;
  } while (iVar9 != 0);
  ExceptionList = pvStack_c;
  return;
}



// public: void __fastcall srGFInterface::loadMaterials(void)

void __fastcall srGFInterface::loadMaterials(void)

{
  char cVar1;
  void *pvVar2;
  void *this;
  int *piVar3;
  void **ppvVar4;
  bool bVar5;
  int *piVar6;
  undefined4 *puVar7;
  int *in_ECX;
  uint uVar8;
  uint uVar9;
  int iVar10;
  char *pcVar11;
  char *pcVar12;
  basic_string<> local_5ad;
  int iStack_5ac;
  void *local_5a8;
  basic_string<> local_5a4 [4];
  char *pcStack_5a0;
  undefined4 uStack_59c;
  undefined4 uStack_598;
  int *piStack_594;
  int *local_590;
  char *pcStack_58c;
  int *piStack_588;
  char acStack_584 [4];
  int *piStack_580;
  int aiStack_57c [2];
  char acStack_574 [616];
  char acStack_30c [256];
  char acStack_20c [256];
  char acStack_10c [256];
  void *local_c;
  undefined *puStack_8;
  int iStack_4;
  
                    // 0x5a70  158  ?loadMaterials@srGFInterface@@QAIXXZ
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_10021e71;
  pvVar2 = *(void **)(*in_ECX + 0xc);
  ppvVar4 = &local_c;
  local_590 = in_ECX;
  local_c = ExceptionList;
  do {
    ExceptionList = ppvVar4;
    if (pvVar2 == (void *)0x0) {
      ExceptionList = local_c;
      return;
    }
    local_5a8 = pvVar2;
    if (*(int *)((int)pvVar2 + 4) == 0xb) {
      std::basic_string<>::_Tidy(local_5a4,false);
      uVar8 = 0xffffffff;
      pcVar11 = s_not_defined_100357ac;
      do {
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        cVar1 = *pcVar11;
        pcVar11 = pcVar11 + 1;
      } while (cVar1 != '\0');
      uVar8 = ~uVar8 - 1;
      bVar5 = std::basic_string<>::_Grow(local_5a4,uVar8,true);
      if (bVar5) {
        pcVar11 = s_not_defined_100357ac;
        pcVar12 = pcStack_5a0;
        for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
          *(undefined4 *)pcVar12 = *(undefined4 *)pcVar11;
          pcVar11 = pcVar11 + 4;
          pcVar12 = pcVar12 + 4;
        }
        for (uVar9 = uVar8 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
          *pcVar12 = *pcVar11;
          pcVar11 = pcVar11 + 1;
          pcVar12 = pcVar12 + 1;
        }
        std::basic_string<>::_Eos(local_5a4,uVar8);
      }
      this = *(void **)((int)pvVar2 + 0x1c);
      iStack_4 = 0;
      for (; this != (void *)0x0; this = *(void **)((int)this + 0x18)) {
        if (*(int *)((int)this + 4) == 3) {
          FUN_10002180(this,&iStack_5ac);
          iStack_4._0_1_ = 1;
          if (iStack_5ac != -1) {
            FUN_100010a0(&iStack_5ac);
            iStack_4 = (uint)iStack_4._1_3_ << 8;
            FUN_10001090(&iStack_5ac);
            break;
          }
          iStack_4 = (uint)iStack_4._1_3_ << 8;
          FUN_10001090(&iStack_5ac);
        }
      }
      FUN_100026d0(pvVar2,&piStack_588);
      for (pvVar2 = *(void **)((int)pvVar2 + 0x1c); pvVar2 != (void *)0x0;
          pvVar2 = *(void **)((int)pvVar2 + 0x18)) {
        if (*(int *)((int)pvVar2 + 4) == 0xd) {
          piStack_594 = aiStack_57c;
          FUN_100028c0(pvVar2,&piStack_594);
          iVar10 = 0;
          if (piStack_594 != (int *)0x0) {
            iVar10 = *piStack_594;
          }
          FUN_10002900(pvVar2,&pcStack_58c);
          srSystem::splitPath(pcStack_58c,acStack_584,acStack_10c,acStack_20c,acStack_30c);
          sprintf(acStack_574,&DAT_100357b8,acStack_20c,acStack_30c);
          if (iVar10 == 0) {
            uVar8 = 0xffffffff;
            pcVar11 = acStack_574;
            do {
              if (uVar8 == 0) break;
              uVar8 = uVar8 - 1;
              cVar1 = *pcVar11;
              pcVar11 = pcVar11 + 1;
            } while (cVar1 != '\0');
            uVar8 = ~uVar8 - 1;
            bVar5 = std::basic_string<>::_Grow(local_5a4,uVar8,true);
            if (bVar5) {
              pcVar11 = acStack_574;
              pcVar12 = pcStack_5a0;
              for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                *(undefined4 *)pcVar12 = *(undefined4 *)pcVar11;
                pcVar11 = pcVar11 + 4;
                pcVar12 = pcVar12 + 4;
              }
LAB_10005c77:
              for (uVar9 = uVar8 & 3; uVar9 != 0; uVar9 = uVar9 - 1) {
                *pcVar12 = *pcVar11;
                pcVar11 = pcVar11 + 1;
                pcVar12 = pcVar12 + 1;
              }
              std::basic_string<>::_Eos(local_5a4,uVar8);
            }
          }
          else {
            uVar8 = 0xffffffff;
            pcVar11 = acStack_574;
            do {
              if (uVar8 == 0) break;
              uVar8 = uVar8 - 1;
              cVar1 = *pcVar11;
              pcVar11 = pcVar11 + 1;
            } while (cVar1 != '\0');
            uVar8 = ~uVar8 - 1;
            bVar5 = std::basic_string<>::_Grow(local_5a4,uVar8,true);
            if (bVar5) {
              pcVar11 = acStack_574;
              pcVar12 = pcStack_5a0;
              for (uVar9 = uVar8 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
                *(undefined4 *)pcVar12 = *(undefined4 *)pcVar11;
                pcVar11 = pcVar11 + 4;
                pcVar12 = pcVar12 + 4;
              }
              goto LAB_10005c77;
            }
          }
        }
      }
      piVar6 = (int *)operator_new(0x14);
      iStack_4._0_1_ = 2;
      piStack_580 = piVar6;
      if (piVar6 == (int *)0x0) {
        piVar6 = (int *)0x0;
      }
      else {
        *(basic_string<> *)(piVar6 + 1) = local_5ad;
        std::basic_string<>::_Tidy((basic_string<> *)(piVar6 + 1),false);
      }
      iStack_4 = (uint)iStack_4._1_3_ << 8;
      std::basic_string<>::assign((basic_string<> *)(piVar6 + 1),local_5a4,0,*(uint *)npos_exref);
      piVar3 = local_590;
      iVar10 = *piStack_588;
      *piVar6 = iVar10;
      uVar8 = local_590[7] - 1U & iVar10 >> 2;
      puVar7 = (undefined4 *)operator_new(8);
      if (puVar7 == (undefined4 *)0x0) {
        puVar7 = (undefined4 *)0x0;
      }
      else {
        *puVar7 = piVar6;
        puVar7[1] = 0;
      }
      puVar7[1] = *(undefined4 *)(piVar3[6] + uVar8 * 4);
      *(undefined4 **)(piVar3[6] + uVar8 * 4) = puVar7;
      piVar3[8] = piVar3[8] + 1;
      iStack_4 = 0xffffffff;
      if (pcStack_5a0 != (char *)0x0) {
        cVar1 = pcStack_5a0[-1];
        if ((cVar1 == '\0') || (cVar1 == -1)) {
          FUN_100212a4(pcStack_5a0 + -1);
        }
        else {
          pcStack_5a0[-1] = cVar1 + -1;
        }
      }
      pcStack_5a0 = (char *)0x0;
      uStack_59c = 0;
      uStack_598 = 0;
    }
    pvVar2 = *(void **)((int)local_5a8 + 0x18);
    ppvVar4 = (void **)ExceptionList;
  } while( true );
}



void FUN_10005d90(undefined4 param_1,undefined4 param_2,int param_3,undefined *param_4)

{
  if (-1 < param_3 + -1) {
    do {
      (*(code *)param_4)();
      param_3 = param_3 + -1;
    } while (param_3 != 0);
  }
  return;
}



// public: class srGERD * __fastcall Housemarque::Threedee_Engine::Engine::Get_GERD(void)

srGERD * __fastcall Housemarque::Threedee_Engine::Engine::Get_GERD(void)

{
  int in_ECX;
  
                    // 0x5dc0  77  ?Get_GERD@Engine@Threedee_Engine@Housemarque@@QAIPAVsrGERD@@XZ
  return *(srGERD **)(in_ECX + 0x44);
}



// public: enum Housemarque::Threedee_Engine::Scene::Polygon_Mode __fastcall
// Housemarque::Threedee_Engine::Scene::Get_Polygon_Mode(void)

Polygon_Mode __fastcall Housemarque::Threedee_Engine::Scene::Get_Polygon_Mode(void)

{
  int in_ECX;
  
                    // 0x5dd0  88
                    // ?Get_Polygon_Mode@Scene@Threedee_Engine@Housemarque@@QAI?AW4Polygon_Mode@123@XZ
  return *(Polygon_Mode *)(in_ECX + 0x20);
}



// public: float __fastcall
// Housemarque::Threedee_Engine::Trimesh::Smallest_Visible_Size_In_Pixels(void)const 

float __fastcall Housemarque::Threedee_Engine::Trimesh::Smallest_Visible_Size_In_Pixels(void)

{
  int in_ECX;
  
                    // 0x5de0  149
                    // ?Smallest_Visible_Size_In_Pixels@Trimesh@Threedee_Engine@Housemarque@@QBIMXZ
  return *(float *)(in_ECX + 0x17c);
}



// public: void __fastcall Housemarque::Threedee_Engine::Resource_Handler::Enable_Texturing(bool)

void __fastcall Housemarque::Threedee_Engine::Resource_Handler::Enable_Texturing(bool param_1)

{
  undefined3 in_register_00000005;
  char in_DL;
  
                    // 0x5df0  68
                    // ?Enable_Texturing@Resource_Handler@Threedee_Engine@Housemarque@@QAIX_N@Z
  *(bool *)(CONCAT31(in_register_00000005,param_1) + 0x2a) = in_DL == '\0';
  return;
}



// public: void __thiscall Emissive_ARGB_Model::`default constructor closure'(void)

void __thiscall Emissive_ARGB_Model::_default_constructor_closure_(Emissive_ARGB_Model *this)

{
                    // 0x5e00  47  ??_FEmissive_ARGB_Model@@QAEXXZ
  Emissive_ARGB_Model(this,0,0);
  return;
}



undefined4 __fastcall FUN_10005e10(int param_1)

{
  void *this;
  void **ppvVar1;
  undefined4 uVar2;
  int local_10;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021e88;
  this = *(void **)(param_1 + 0x1c);
  ppvVar1 = &local_c;
  local_10 = param_1;
  local_c = ExceptionList;
  do {
    ExceptionList = ppvVar1;
    if (this == (void *)0x0) {
      ExceptionList = local_c;
      return 0;
    }
    if (*(int *)((int)this + 4) == 3) {
      FUN_10002180(this,&local_10);
      local_4 = 0;
      if (local_10 != -1) {
        uVar2 = FUN_100010a0(&local_10);
        local_4 = 0xffffffff;
        FUN_10001090(&local_10);
        ExceptionList = local_c;
        return uVar2;
      }
      local_4 = 0xffffffff;
      FUN_10001090(&local_10);
    }
    this = *(void **)((int)this + 0x18);
    ppvVar1 = (void **)ExceptionList;
  } while( true );
}



void __cdecl FUN_10005eb0(void *param_1)

{
  srHeap::free(param_1);
  return;
}



void __thiscall FUN_10005ed0(void *this,undefined4 *param_1)

{
  *(undefined4 *)this = *param_1;
  return;
}



ushort __cdecl FUN_10005ee0(int param_1,float *param_2)

{
  float fVar1;
  float fVar2;
  uint uVar3;
  float *pfVar4;
  
  uVar3 = 0;
  pfVar4 = param_2;
  do {
    fVar1 = *(float *)((param_1 - (int)param_2) + (int)pfVar4);
    fVar2 = *pfVar4;
    if ((fVar1 == fVar2) == 0) {
      if (*(float *)(param_1 + uVar3 * 4) < param_2[uVar3]) {
        return 1;
      }
      return 0;
    }
    uVar3 = uVar3 + 1;
    pfVar4 = pfVar4 + 1;
  } while (uVar3 < 3);
  return (ushort)(fVar1 < fVar2) << 8 | (ushort)(NAN(fVar1) || NAN(fVar2)) << 10 |
         (ushort)(fVar1 == fVar2) << 0xe;
}



void __fastcall FUN_10005f30(int param_1)

{
  char *pcVar1;
  char cVar2;
  
  if (*(int *)(param_1 + 8) != 0) {
    pcVar1 = (char *)(*(int *)(param_1 + 8) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(param_1 + 8) = 0;
  *(undefined4 *)(param_1 + 0xc) = 0;
  *(undefined4 *)(param_1 + 0x10) = 0;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __fastcall FUN_10005f70(void *param_1)

{
  void *local_4;
  
  local_4 = param_1;
  FUN_10006490(param_1,&local_4,(int *)**(int **)((int)param_1 + 4),*(int **)((int)param_1 + 4));
  FUN_100212a4(*(void **)((int)param_1 + 4));
  *(undefined4 *)((int)param_1 + 4) = 0;
  *(undefined4 *)((int)param_1 + 0xc) = 0;
  std::_Lockit::_Lockit((_Lockit *)&local_4);
  _DAT_100384d8 = _DAT_100384d8 + -1;
  if (_DAT_100384d8 == 0) {
    FUN_100212a4(DAT_100384d4);
    DAT_100384d4 = (void *)0x0;
  }
  std::_Lockit::~_Lockit((_Lockit *)&local_4);
  return;
}



int __fastcall FUN_10005fe0(int param_1)

{
  if (*(int *)(param_1 + 4) == 0) {
    return 0;
  }
  return *(int *)(param_1 + 8) - *(int *)(param_1 + 4) >> 2;
}



void __fastcall FUN_10006000(int param_1)

{
  int *piVar1;
  int *piVar2;
  int *piVar3;
  
  piVar1 = *(int **)(param_1 + 4);
  piVar3 = (int *)*piVar1;
  while (piVar3 != piVar1) {
    piVar2 = (int *)*piVar3;
    *(int *)piVar3[1] = *piVar3;
    *(int *)(*piVar3 + 4) = piVar3[1];
    FUN_100212a4(piVar3);
    *(int *)(param_1 + 8) = *(int *)(param_1 + 8) + -1;
    piVar3 = piVar2;
  }
  FUN_100212a4(*(void **)(param_1 + 4));
  *(undefined4 *)(param_1 + 4) = 0;
  *(undefined4 *)(param_1 + 8) = 0;
  return;
}



void __fastcall FUN_10006050(int *param_1)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  
  iVar3 = 0;
  if (0 < param_1[1]) {
    do {
      puVar1 = *(undefined4 **)(*param_1 + iVar3 * 4);
      while (puVar2 = puVar1, puVar2 != (undefined4 *)0x0) {
        puVar1 = (undefined4 *)puVar2[1];
        if (puVar2 != (undefined4 *)0x0) {
          FUN_100212a4((void *)*puVar2);
          FUN_100212a4(puVar2);
        }
      }
      iVar3 = iVar3 + 1;
    } while (iVar3 < param_1[1]);
  }
  FUN_100212a4((void *)*param_1);
  *param_1 = 0;
  param_1[2] = 0;
  return;
}



void __fastcall FUN_100060b0(int *param_1)

{
  undefined4 *puVar1;
  void *pvVar2;
  undefined4 *puVar3;
  int local_4;
  
  local_4 = 0;
  if (0 < param_1[1]) {
    do {
      puVar1 = *(undefined4 **)(*param_1 + local_4 * 4);
      while (puVar3 = puVar1, puVar3 != (undefined4 *)0x0) {
        puVar1 = (undefined4 *)puVar3[1];
        if (puVar3 != (undefined4 *)0x0) {
          pvVar2 = (void *)*puVar3;
          if (pvVar2 != (void *)0x0) {
            FUN_10005f30((int)pvVar2);
            FUN_100212a4(pvVar2);
          }
          FUN_100212a4(puVar3);
        }
      }
      local_4 = local_4 + 1;
    } while (local_4 < param_1[1]);
  }
  FUN_100212a4((void *)*param_1);
  *param_1 = 0;
  param_1[2] = 0;
  return;
}



void __thiscall FUN_10006130(void *this,undefined4 *param_1,uint param_2,undefined4 *param_3)

{
  undefined4 *puVar1;
  int iVar2;
  int iVar3;
  undefined4 *puVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  uint uVar7;
  
  puVar5 = *(undefined4 **)((int)this + 8);
  if (param_2 <= (uint)(*(int *)((int)this + 0xc) - (int)puVar5 >> 2)) {
    if ((uint)((int)puVar5 - (int)param_1 >> 2) < param_2) {
      puVar6 = param_1 + param_2;
      if (param_1 != puVar5) {
        puVar4 = puVar6 + -param_2;
        do {
          if (puVar6 != (undefined4 *)0x0) {
            *puVar6 = *puVar4;
          }
          puVar4 = puVar4 + 1;
          puVar6 = puVar6 + 1;
        } while (puVar4 != puVar5);
      }
      puVar5 = *(undefined4 **)((int)this + 8);
      for (iVar2 = param_2 - ((int)puVar5 - (int)param_1 >> 2); iVar2 != 0; iVar2 = iVar2 + -1) {
        if (puVar5 != (undefined4 *)0x0) {
          *puVar5 = *param_3;
        }
        puVar5 = puVar5 + 1;
      }
      puVar5 = *(undefined4 **)((int)this + 8);
      if (param_1 != puVar5) {
        do {
          *param_1 = *param_3;
          param_1 = param_1 + 1;
        } while (param_1 != puVar5);
      }
      *(int *)((int)this + 8) = *(int *)((int)this + 8) + param_2 * 4;
      return;
    }
    if (param_2 != 0) {
      puVar6 = puVar5;
      for (puVar4 = puVar5 + -param_2; puVar4 != puVar5; puVar4 = puVar4 + 1) {
        if (puVar6 != (undefined4 *)0x0) {
          *puVar6 = *puVar4;
        }
        puVar6 = puVar6 + 1;
      }
      puVar5 = *(undefined4 **)((int)this + 8);
      for (puVar6 = puVar5 + -param_2; param_1 != puVar6; puVar6 = puVar6 + -1) {
        puVar5 = puVar5 + -1;
        *puVar5 = puVar6[-1];
      }
      puVar5 = param_1 + param_2;
      if (param_1 != puVar5) {
        do {
          *param_1 = *param_3;
          param_1 = param_1 + 1;
        } while (param_1 != puVar5);
      }
      *(int *)((int)this + 8) = *(int *)((int)this + 8) + param_2 * 4;
    }
    return;
  }
  iVar2 = *(int *)((int)this + 4);
  if ((iVar2 == 0) || (uVar7 = (int)puVar5 - iVar2 >> 2, uVar7 <= param_2)) {
    uVar7 = param_2;
  }
  if (iVar2 == 0) {
    iVar2 = 0;
  }
  else {
    iVar2 = (int)puVar5 - iVar2 >> 2;
  }
  iVar2 = iVar2 + uVar7;
  iVar3 = iVar2;
  if (iVar2 < 0) {
    iVar3 = 0;
  }
  puVar4 = (undefined4 *)operator_new(iVar3 * 4);
  puVar6 = puVar4;
  for (puVar5 = *(undefined4 **)((int)this + 4); puVar5 != param_1; puVar5 = puVar5 + 1) {
    if (puVar6 != (undefined4 *)0x0) {
      *puVar6 = *puVar5;
    }
    puVar6 = puVar6 + 1;
  }
  puVar5 = puVar6;
  uVar7 = param_2;
  if (param_2 != 0) {
    do {
      if (puVar5 != (undefined4 *)0x0) {
        *puVar5 = *param_3;
      }
      uVar7 = uVar7 - 1;
      puVar5 = puVar5 + 1;
    } while (uVar7 != 0);
  }
  puVar1 = *(undefined4 **)((int)this + 8);
  puVar5 = puVar6 + param_2;
  if (param_1 != puVar1) {
    puVar6 = (undefined4 *)((int)puVar5 + (param_2 * -4 - (int)puVar6) + (int)param_1);
    do {
      if (puVar5 != (undefined4 *)0x0) {
        *puVar5 = *puVar6;
      }
      puVar6 = puVar6 + 1;
      puVar5 = puVar5 + 1;
    } while (puVar6 != puVar1);
  }
  FUN_100212a4(*(void **)((int)this + 4));
  *(undefined4 **)((int)this + 0xc) = puVar4 + iVar2;
  if (*(int *)((int)this + 4) == 0) {
    *(undefined4 **)((int)this + 8) = puVar4 + param_2;
    *(undefined4 **)((int)this + 4) = puVar4;
    return;
  }
  *(undefined4 **)((int)this + 8) =
       puVar4 + (*(int *)((int)this + 8) - *(int *)((int)this + 4) >> 2) + param_2;
  *(undefined4 **)((int)this + 4) = puVar4;
  return;
}



void FUN_10006340(void)

{
  return;
}



undefined4 * FUN_10006350(undefined4 *param_1,undefined4 *param_2,undefined4 *param_3)

{
  if (param_1 == param_2) {
    return param_3;
  }
  do {
    if (param_3 != (undefined4 *)0x0) {
      *param_3 = *param_1;
    }
    param_1 = param_1 + 1;
    param_3 = param_3 + 1;
  } while (param_1 != param_2);
  return param_3;
}



void FUN_10006380(undefined4 *param_1,int param_2,undefined4 *param_3)

{
  if (param_2 != 0) {
    do {
      if (param_1 != (undefined4 *)0x0) {
        *param_1 = *param_3;
      }
      param_1 = param_1 + 1;
      param_2 = param_2 + -1;
    } while (param_2 != 0);
  }
  return;
}



undefined4 * __thiscall
FUN_100063b0(void *this,undefined4 *param_1,void *param_2,undefined4 *param_3)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  
  puVar3 = *(undefined4 **)((int)param_2 + 4);
  puVar1 = (undefined4 *)operator_new(0xc);
  puVar2 = (undefined4 *)param_2;
  if (param_2 == (void *)0x0) {
    puVar2 = puVar1;
  }
  *puVar1 = puVar2;
  if (puVar3 == (undefined4 *)0x0) {
    puVar3 = puVar1;
  }
  puVar1[1] = puVar3;
  *(undefined4 **)((int)param_2 + 4) = puVar1;
  *(undefined4 **)puVar1[1] = puVar1;
  if (puVar1 + 2 != (undefined4 *)0x0) {
    puVar1[2] = *param_3;
  }
  *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
  *param_1 = puVar1;
  return param_1;
}



void __thiscall FUN_10006410(void *this,undefined4 *param_1,int *param_2,int *param_3)

{
  int *piVar1;
  
  piVar1 = param_2;
  if (param_2 != param_3) {
    do {
      param_2 = (int *)*piVar1;
      *(int *)piVar1[1] = *piVar1;
      *(int *)(*piVar1 + 4) = piVar1[1];
      FUN_100212a4(piVar1);
      *(int *)((int)this + 8) = *(int *)((int)this + 8) + -1;
      piVar1 = param_2;
    } while (param_2 != param_3);
  }
  *param_1 = param_2;
  return;
}



void FUN_10006460(undefined4 *param_1,int param_2)

{
  undefined4 *puVar1;
  
  puVar1 = (undefined4 *)operator_new(0xc);
  if (param_1 == (undefined4 *)0x0) {
    param_1 = puVar1;
  }
  *puVar1 = param_1;
  if (param_2 != 0) {
    puVar1[1] = param_2;
    return;
  }
  puVar1[1] = puVar1;
  return;
}



undefined4 * __thiscall FUN_10006490(void *this,undefined4 *param_1,int *param_2,int *param_3)

{
  int *piVar1;
  int *piVar2;
  _Lockit local_14 [4];
  undefined4 local_10;
  void *local_c;
  undefined *puStack_8;
  int local_4;
  
  piVar2 = param_3;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021eb2;
  local_c = ExceptionList;
  if (((*(int *)((int)this + 0xc) == 0) || (param_2 != (int *)**(int **)((int)this + 4))) ||
     (param_3 != *(int **)((int)this + 4))) {
    ExceptionList = &local_c;
    if (param_2 != param_3) {
      do {
        piVar1 = param_2;
        FUN_100071c0((int *)&param_2);
        FUN_100066f0(this,&local_10,piVar1);
      } while (param_2 != piVar2);
    }
    *param_1 = param_2;
  }
  else {
    ExceptionList = &local_c;
    std::_Lockit::_Lockit(local_14);
    piVar2 = *(int **)(*(int *)((int)this + 4) + 4);
    local_4 = 0;
    std::_Lockit::_Lockit((_Lockit *)&param_3);
    local_4._0_1_ = 1;
    if (piVar2 != DAT_100384d4) {
      do {
        FUN_10006c20((int *)piVar2[2]);
        piVar1 = (int *)*piVar2;
        FUN_100212a4(piVar2);
        piVar2 = piVar1;
      } while (piVar1 != DAT_100384d4);
    }
    local_4 = (uint)local_4._1_3_ << 8;
    std::_Lockit::~_Lockit((_Lockit *)&param_3);
    *(int **)(*(int *)((int)this + 4) + 4) = DAT_100384d4;
    *(undefined4 *)((int)this + 0xc) = 0;
    *(undefined4 *)*(undefined4 *)((int)this + 4) = *(undefined4 *)((int)this + 4);
    *(int *)(*(int *)((int)this + 4) + 8) = *(int *)((int)this + 4);
    *param_1 = **(undefined4 **)((int)this + 4);
    local_4 = 0xffffffff;
    std::_Lockit::~_Lockit(local_14);
  }
  ExceptionList = local_c;
  return param_1;
}



void __thiscall FUN_100065b0(void *this,undefined4 *param_1,float *param_2)

{
  float *pfVar1;
  ushort uVar2;
  undefined4 *puVar3;
  int *piVar4;
  int *piVar5;
  char local_d;
  int *piStack_c;
  _Lockit local_8 [4];
  uint uStack_4;
  
  piVar5 = *(int **)((int)this + 4);
  piVar4 = (int *)piVar5[1];
  local_d = '\x01';
  std::_Lockit::_Lockit(local_8);
  pfVar1 = param_2;
  if (piVar4 != DAT_100384d4) {
    do {
      piVar5 = piVar4;
      uVar2 = FUN_10005ee0((int)pfVar1,(float *)(piVar5 + 3));
      local_d = (char)uVar2;
      if (local_d == '\0') {
        piVar4 = (int *)piVar5[2];
      }
      else {
        piVar4 = (int *)*piVar5;
      }
    } while (piVar4 != DAT_100384d4);
  }
  std::_Lockit::~_Lockit(local_8);
  if (*(char *)((int)this + 8) != '\0') {
    puVar3 = FUN_10006d60(this,&param_2,(int)piVar4,piVar5,pfVar1);
    uStack_4 = CONCAT31(uStack_4._1_3_,1);
    *param_1 = *puVar3;
    param_1[1] = uStack_4;
    return;
  }
  piStack_c = piVar5;
  if (local_d != '\0') {
    if (piVar5 == (int *)**(int **)((int)this + 4)) {
      puVar3 = FUN_10006d60(this,&param_2,(int)piVar4,piVar5,pfVar1);
      uStack_4 = CONCAT31(uStack_4._1_3_,1);
      *param_1 = *puVar3;
      param_1[1] = uStack_4;
      return;
    }
    FUN_10007100((int *)&piStack_c);
  }
  uVar2 = FUN_10005ee0((int)(piStack_c + 3),pfVar1);
  if ((char)uVar2 != '\0') {
    puVar3 = FUN_10006d60(this,&param_2,(int)piVar4,piVar5,pfVar1);
    uStack_4 = CONCAT31(uStack_4._1_3_,1);
    *param_1 = *puVar3;
    param_1[1] = uStack_4;
    return;
  }
  *param_1 = piStack_c;
  uStack_4 = uStack_4 & 0xffffff00;
  param_1[1] = uStack_4;
  return;
}



undefined4 * __thiscall FUN_100066f0(void *this,undefined4 *param_1,int *param_2)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  int *piVar4;
  int *piVar5;
  int *piVar6;
  int *local_1c;
  _Lockit local_18 [4];
  _Lockit local_14 [4];
  _Lockit local_10 [4];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  piVar6 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021ec9;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_100071c0((int *)&param_2);
  local_1c = piVar6;
  std::_Lockit::_Lockit(local_10);
  piVar5 = (int *)*piVar6;
  local_4 = 0;
  if (piVar5 == DAT_100384d4) {
    piVar5 = (int *)piVar6[2];
  }
  else {
    piVar2 = (int *)piVar6[2];
    if (piVar2 != DAT_100384d4) {
      std::_Lockit::_Lockit(local_18);
      local_1c = piVar2;
      for (piVar5 = (int *)*piVar2; piVar5 != DAT_100384d4; piVar5 = (int *)*piVar5) {
        local_1c = piVar5;
      }
      std::_Lockit::~_Lockit(local_18);
      piVar5 = (int *)local_1c[2];
      if (local_1c != piVar6) {
        *(int **)(*piVar6 + 4) = local_1c;
        *local_1c = *piVar6;
        if (local_1c == (int *)piVar6[2]) {
          piVar5[1] = (int)local_1c;
        }
        else {
          piVar5[1] = local_1c[1];
          *(int **)local_1c[1] = piVar5;
          local_1c[2] = piVar6[2];
          *(int **)(piVar6[2] + 4) = local_1c;
        }
        if (*(int **)(*(int *)((int)this + 4) + 4) == piVar6) {
          *(int **)(*(int *)((int)this + 4) + 4) = local_1c;
        }
        else {
          piVar2 = (int *)piVar6[1];
          if ((int *)*piVar2 == piVar6) {
            *piVar2 = (int)local_1c;
          }
          else {
            piVar2[2] = (int)local_1c;
          }
        }
        local_1c[1] = piVar6[1];
        iVar1 = local_1c[7];
        local_1c[7] = piVar6[7];
        piVar6[7] = iVar1;
        local_1c = piVar6;
        goto LAB_100068a5;
      }
    }
  }
  piVar5[1] = local_1c[1];
  if (*(int **)(*(int *)((int)this + 4) + 4) == piVar6) {
    *(int **)(*(int *)((int)this + 4) + 4) = piVar5;
  }
  else {
    piVar2 = (int *)piVar6[1];
    if ((int *)*piVar2 == piVar6) {
      *piVar2 = (int)piVar5;
    }
    else {
      piVar2[2] = (int)piVar5;
    }
  }
  if ((int *)**(int **)((int)this + 4) == piVar6) {
    if ((int *)piVar6[2] == DAT_100384d4) {
      **(int **)((int)this + 4) = piVar6[1];
    }
    else {
      std::_Lockit::_Lockit(local_14);
      piVar3 = (int *)*piVar5;
      piVar2 = piVar5;
      while (piVar4 = piVar3, piVar4 != DAT_100384d4) {
        piVar2 = piVar4;
        piVar3 = (int *)*piVar4;
      }
      std::_Lockit::~_Lockit(local_14);
      **(undefined4 **)((int)this + 4) = piVar2;
    }
  }
  if (*(int **)(*(int *)((int)this + 4) + 8) == piVar6) {
    if ((int *)*piVar6 == DAT_100384d4) {
      *(int *)(*(int *)((int)this + 4) + 8) = piVar6[1];
    }
    else {
      std::_Lockit::_Lockit(local_14);
      piVar2 = (int *)piVar5[2];
      piVar6 = piVar5;
      while (piVar3 = piVar2, piVar3 != DAT_100384d4) {
        piVar6 = piVar3;
        piVar2 = (int *)piVar3[2];
      }
      std::_Lockit::~_Lockit(local_14);
      *(int **)(*(int *)((int)this + 4) + 8) = piVar6;
    }
  }
LAB_100068a5:
  if (local_1c[7] == 1) {
    if (piVar5 != *(int **)(*(int *)((int)this + 4) + 4)) {
      do {
        if (piVar5[7] != 1) break;
        piVar6 = (int *)piVar5[1];
        if (piVar5 == (int *)*piVar6) {
          piVar6 = (int *)piVar6[2];
          if (piVar6[7] == 0) {
            piVar6[7] = 1;
            *(undefined4 *)(piVar5[1] + 0x1c) = 0;
            iVar1 = piVar5[1];
            std::_Lockit::_Lockit(local_14);
            piVar6 = *(int **)(iVar1 + 8);
            *(int *)(iVar1 + 8) = *piVar6;
            if ((int *)*piVar6 != DAT_100384d4) {
              ((int *)*piVar6)[1] = iVar1;
            }
            piVar6[1] = *(int *)(iVar1 + 4);
            if (iVar1 == *(int *)(*(int *)((int)this + 4) + 4)) {
              *(int **)(*(int *)((int)this + 4) + 4) = piVar6;
            }
            else {
              piVar2 = *(int **)(iVar1 + 4);
              if (iVar1 == *piVar2) {
                *piVar2 = (int)piVar6;
              }
              else {
                piVar2[2] = (int)piVar6;
              }
            }
            *piVar6 = iVar1;
            *(int **)(iVar1 + 4) = piVar6;
            std::_Lockit::~_Lockit(local_14);
            piVar6 = *(int **)(piVar5[1] + 8);
          }
          if ((*(int *)(*piVar6 + 0x1c) != 1) || (*(int *)(piVar6[2] + 0x1c) != 1)) {
            if (*(int *)(piVar6[2] + 0x1c) == 1) {
              *(undefined4 *)(*piVar6 + 0x1c) = 1;
              piVar6[7] = 0;
              std::_Lockit::_Lockit(local_14);
              iVar1 = *piVar6;
              *piVar6 = *(int *)(iVar1 + 8);
              if (*(int **)(iVar1 + 8) != DAT_100384d4) {
                (*(int **)(iVar1 + 8))[1] = (int)piVar6;
              }
              *(int *)(iVar1 + 4) = piVar6[1];
              if (piVar6 == *(int **)(*(int *)((int)this + 4) + 4)) {
                *(int *)(*(int *)((int)this + 4) + 4) = iVar1;
              }
              else {
                piVar2 = (int *)piVar6[1];
                if (piVar6 == (int *)piVar2[2]) {
                  piVar2[2] = iVar1;
                }
                else {
                  *piVar2 = iVar1;
                }
              }
              *(int **)(iVar1 + 8) = piVar6;
              piVar6[1] = iVar1;
              std::_Lockit::~_Lockit(local_14);
              piVar6 = *(int **)(piVar5[1] + 8);
            }
            piVar6[7] = *(int *)(piVar5[1] + 0x1c);
            *(undefined4 *)(piVar5[1] + 0x1c) = 1;
            *(undefined4 *)(piVar6[2] + 0x1c) = 1;
            iVar1 = piVar5[1];
            std::_Lockit::_Lockit(local_14);
            piVar6 = *(int **)(iVar1 + 8);
            *(int *)(iVar1 + 8) = *piVar6;
            if ((int *)*piVar6 != DAT_100384d4) {
              ((int *)*piVar6)[1] = iVar1;
            }
            piVar6[1] = *(int *)(iVar1 + 4);
            if (iVar1 == *(int *)(*(int *)((int)this + 4) + 4)) {
              *(int **)(*(int *)((int)this + 4) + 4) = piVar6;
              *piVar6 = iVar1;
              *(int **)(iVar1 + 4) = piVar6;
            }
            else {
              piVar2 = *(int **)(iVar1 + 4);
              if (iVar1 == *piVar2) {
                *piVar2 = (int)piVar6;
                *piVar6 = iVar1;
                *(int **)(iVar1 + 4) = piVar6;
              }
              else {
                piVar2[2] = (int)piVar6;
                *piVar6 = iVar1;
                *(int **)(iVar1 + 4) = piVar6;
              }
            }
LAB_10006bc0:
            std::_Lockit::~_Lockit(local_14);
            break;
          }
        }
        else {
          piVar6 = (int *)*piVar6;
          if (piVar6[7] == 0) {
            piVar6[7] = 1;
            *(undefined4 *)(piVar5[1] + 0x1c) = 0;
            piVar6 = (int *)piVar5[1];
            std::_Lockit::_Lockit(local_18);
            iVar1 = *piVar6;
            *piVar6 = *(int *)(iVar1 + 8);
            if (*(int **)(iVar1 + 8) != DAT_100384d4) {
              (*(int **)(iVar1 + 8))[1] = (int)piVar6;
            }
            *(int *)(iVar1 + 4) = piVar6[1];
            if (piVar6 == *(int **)(*(int *)((int)this + 4) + 4)) {
              *(int *)(*(int *)((int)this + 4) + 4) = iVar1;
            }
            else {
              piVar2 = (int *)piVar6[1];
              if (piVar6 == (int *)piVar2[2]) {
                piVar2[2] = iVar1;
              }
              else {
                *piVar2 = iVar1;
              }
            }
            *(int **)(iVar1 + 8) = piVar6;
            piVar6[1] = iVar1;
            std::_Lockit::~_Lockit(local_18);
            piVar6 = *(int **)piVar5[1];
          }
          if ((*(int *)(piVar6[2] + 0x1c) != 1) || (*(int *)(*piVar6 + 0x1c) != 1)) {
            if (*(int *)(*piVar6 + 0x1c) == 1) {
              *(undefined4 *)(piVar6[2] + 0x1c) = 1;
              piVar6[7] = 0;
              std::_Lockit::_Lockit(local_14);
              piVar2 = (int *)piVar6[2];
              piVar6[2] = *piVar2;
              if ((int *)*piVar2 != DAT_100384d4) {
                ((int *)*piVar2)[1] = (int)piVar6;
              }
              piVar2[1] = piVar6[1];
              if (piVar6 == *(int **)(*(int *)((int)this + 4) + 4)) {
                *(int **)(*(int *)((int)this + 4) + 4) = piVar2;
              }
              else {
                piVar3 = (int *)piVar6[1];
                if (piVar6 == (int *)*piVar3) {
                  *piVar3 = (int)piVar2;
                }
                else {
                  piVar3[2] = (int)piVar2;
                }
              }
              *piVar2 = (int)piVar6;
              piVar6[1] = (int)piVar2;
              std::_Lockit::~_Lockit(local_14);
              piVar6 = *(int **)piVar5[1];
            }
            piVar6[7] = *(int *)(piVar5[1] + 0x1c);
            *(undefined4 *)(piVar5[1] + 0x1c) = 1;
            *(undefined4 *)(*piVar6 + 0x1c) = 1;
            piVar6 = (int *)piVar5[1];
            std::_Lockit::_Lockit(local_14);
            iVar1 = *piVar6;
            *piVar6 = *(int *)(iVar1 + 8);
            if (*(int **)(iVar1 + 8) != DAT_100384d4) {
              (*(int **)(iVar1 + 8))[1] = (int)piVar6;
            }
            *(int *)(iVar1 + 4) = piVar6[1];
            if (piVar6 == *(int **)(*(int *)((int)this + 4) + 4)) {
              *(int *)(*(int *)((int)this + 4) + 4) = iVar1;
            }
            else {
              piVar2 = (int *)piVar6[1];
              if (piVar6 == (int *)piVar2[2]) {
                piVar2[2] = iVar1;
              }
              else {
                *piVar2 = iVar1;
              }
            }
            *(int **)(iVar1 + 8) = piVar6;
            piVar6[1] = iVar1;
            goto LAB_10006bc0;
          }
        }
        piVar6[7] = 0;
        piVar5 = (int *)piVar5[1];
      } while (piVar5 != *(int **)(*(int *)((int)this + 4) + 4));
    }
    piVar5[7] = 1;
  }
  FUN_100212a4(local_1c);
  *(int *)((int)this + 0xc) = *(int *)((int)this + 0xc) + -1;
  *param_1 = param_2;
  local_4 = 0xffffffff;
  std::_Lockit::~_Lockit(local_10);
  ExceptionList = pvStack_c;
  return param_1;
}



void FUN_10006c20(int *param_1)

{
  int *piVar1;
  _Lockit local_10 [4];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10021ee9;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  std::_Lockit::_Lockit(local_10);
  uStack_4 = 0;
  if (param_1 != DAT_100384d4) {
    do {
      FUN_10006c20((int *)param_1[2]);
      piVar1 = (int *)*param_1;
      FUN_100212a4(param_1);
      param_1 = piVar1;
    } while (piVar1 != DAT_100384d4);
  }
  uStack_4 = 0xffffffff;
  std::_Lockit::~_Lockit(local_10);
  ExceptionList = pvStack_c;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __fastcall FUN_10006cb0(int param_1)

{
  undefined4 *puVar1;
  void *pvVar2;
  int local_10;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10021f09;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_10 = param_1;
  std::_Lockit::_Lockit((_Lockit *)&local_10);
  uStack_4 = 0;
  if (DAT_100384d4 == (undefined4 *)0x0) {
    DAT_100384d4 = (undefined4 *)operator_new(0x20);
    DAT_100384d4[1] = 0;
    DAT_100384d4[7] = 1;
    *DAT_100384d4 = 0;
    DAT_100384d4[2] = 0;
  }
  puVar1 = DAT_100384d4;
  _DAT_100384d8 = _DAT_100384d8 + 1;
  pvVar2 = operator_new(0x20);
  *(undefined4 **)((int)pvVar2 + 4) = puVar1;
  *(undefined4 *)((int)pvVar2 + 0x1c) = 0;
  *(void **)(param_1 + 4) = pvVar2;
  *(undefined4 *)(param_1 + 0xc) = 0;
  *(void **)pvVar2 = pvVar2;
  *(int *)(*(int *)(param_1 + 4) + 8) = *(int *)(param_1 + 4);
  uStack_4 = 0xffffffff;
  std::_Lockit::~_Lockit((_Lockit *)&local_10);
  ExceptionList = pvStack_c;
  return;
}



undefined4 * __thiscall
FUN_10006d60(void *this,undefined4 *param_1,int param_2,int *param_3,undefined4 *param_4)

{
  int *piVar1;
  int *piVar2;
  int iVar3;
  undefined4 *puVar4;
  ushort uVar5;
  int *piVar6;
  _Lockit *this_00;
  int *piVar7;
  _Lockit a_Stack_14 [4];
  _Lockit local_10 [4];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10021f29;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  std::_Lockit::_Lockit(local_10);
  uStack_4 = 0;
  piVar6 = (int *)operator_new(0x20);
  puVar4 = param_4;
  piVar7 = param_3;
  piVar6[7] = 0;
  piVar6[1] = (int)param_3;
  *piVar6 = DAT_100384d4;
  piVar6[2] = DAT_100384d4;
  FUN_10007270(piVar6 + 3,param_4);
  *(int *)((int)this + 0xc) = *(int *)((int)this + 0xc) + 1;
  if ((piVar7 != *(int **)((int)this + 4)) && (param_2 == DAT_100384d4)) {
    uVar5 = FUN_10005ee0((int)puVar4,(float *)(piVar7 + 3));
    if ((char)uVar5 == '\0') {
      piVar7[2] = (int)piVar6;
      if (piVar7 == *(int **)(*(int *)((int)this + 4) + 8)) {
        *(int **)(*(int *)((int)this + 4) + 8) = piVar6;
      }
      goto LAB_10006e18;
    }
  }
  *piVar7 = (int)piVar6;
  piVar1 = *(int **)((int)this + 4);
  if (piVar7 == piVar1) {
    piVar1[1] = (int)piVar6;
    *(int **)(*(int *)((int)this + 4) + 8) = piVar6;
  }
  else if (piVar7 == (int *)*piVar1) {
    *piVar1 = (int)piVar6;
  }
LAB_10006e18:
  piVar7 = piVar6;
  if (piVar6 != *(int **)(*(int *)((int)this + 4) + 4)) {
    do {
      piVar1 = (int *)piVar7[1];
      if (piVar1[7] != 0) break;
      piVar2 = (int *)piVar1[1];
      if (piVar1 == (int *)*piVar2) {
        iVar3 = piVar2[2];
        if (*(int *)(iVar3 + 0x1c) == 0) {
          piVar1[7] = 1;
          *(undefined4 *)(iVar3 + 0x1c) = 1;
          *(undefined4 *)(*(int *)(piVar7[1] + 4) + 0x1c) = 0;
          piVar7 = *(int **)(piVar7[1] + 4);
        }
        else {
          if (piVar7 == (int *)piVar1[2]) {
            std::_Lockit::_Lockit((_Lockit *)&param_3);
            piVar7 = (int *)piVar1[2];
            piVar1[2] = *piVar7;
            if (*piVar7 != DAT_100384d4) {
              *(int **)(*piVar7 + 4) = piVar1;
            }
            piVar7[1] = piVar1[1];
            if (piVar1 == *(int **)(*(int *)((int)this + 4) + 4)) {
              *(int **)(*(int *)((int)this + 4) + 4) = piVar7;
            }
            else {
              piVar2 = (int *)piVar1[1];
              if (piVar1 == (int *)*piVar2) {
                *piVar2 = (int)piVar7;
              }
              else {
                piVar2[2] = (int)piVar7;
              }
            }
            *piVar7 = (int)piVar1;
            piVar1[1] = (int)piVar7;
            std::_Lockit::~_Lockit((_Lockit *)&param_3);
            piVar7 = piVar1;
          }
          *(undefined4 *)(piVar7[1] + 0x1c) = 1;
          *(undefined4 *)(*(int *)(piVar7[1] + 4) + 0x1c) = 0;
          piVar1 = *(int **)(piVar7[1] + 4);
          std::_Lockit::_Lockit((_Lockit *)&param_4);
          iVar3 = *piVar1;
          *piVar1 = *(int *)(iVar3 + 8);
          if (*(int *)(iVar3 + 8) != DAT_100384d4) {
            *(int **)(*(int *)(iVar3 + 8) + 4) = piVar1;
          }
          *(int *)(iVar3 + 4) = piVar1[1];
          if (piVar1 == *(int **)(*(int *)((int)this + 4) + 4)) {
            *(int *)(*(int *)((int)this + 4) + 4) = iVar3;
            *(int **)(iVar3 + 8) = piVar1;
            piVar1[1] = iVar3;
            this_00 = (_Lockit *)&param_4;
          }
          else {
            piVar2 = (int *)piVar1[1];
            if (piVar1 == (int *)piVar2[2]) {
              piVar2[2] = iVar3;
              *(int **)(iVar3 + 8) = piVar1;
              piVar1[1] = iVar3;
              this_00 = (_Lockit *)&param_4;
            }
            else {
              *piVar2 = iVar3;
              *(int **)(iVar3 + 8) = piVar1;
              piVar1[1] = iVar3;
              this_00 = (_Lockit *)&param_4;
            }
          }
LAB_10007020:
          std::_Lockit::~_Lockit(this_00);
        }
      }
      else {
        iVar3 = *piVar2;
        if (*(int *)(iVar3 + 0x1c) != 0) {
          if (piVar7 == (int *)*piVar1) {
            std::_Lockit::_Lockit((_Lockit *)&param_2);
            iVar3 = *piVar1;
            *piVar1 = *(int *)(iVar3 + 8);
            if (*(int *)(iVar3 + 8) != DAT_100384d4) {
              *(int **)(*(int *)(iVar3 + 8) + 4) = piVar1;
            }
            *(int *)(iVar3 + 4) = piVar1[1];
            if (piVar1 == *(int **)(*(int *)((int)this + 4) + 4)) {
              *(int *)(*(int *)((int)this + 4) + 4) = iVar3;
            }
            else {
              piVar7 = (int *)piVar1[1];
              if (piVar1 == (int *)piVar7[2]) {
                piVar7[2] = iVar3;
              }
              else {
                *piVar7 = iVar3;
              }
            }
            *(int **)(iVar3 + 8) = piVar1;
            piVar1[1] = iVar3;
            std::_Lockit::~_Lockit((_Lockit *)&param_2);
            piVar7 = piVar1;
          }
          *(undefined4 *)(piVar7[1] + 0x1c) = 1;
          *(undefined4 *)(*(int *)(piVar7[1] + 4) + 0x1c) = 0;
          iVar3 = *(int *)(piVar7[1] + 4);
          std::_Lockit::_Lockit(a_Stack_14);
          piVar1 = *(int **)(iVar3 + 8);
          *(int *)(iVar3 + 8) = *piVar1;
          if (*piVar1 != DAT_100384d4) {
            *(int *)(*piVar1 + 4) = iVar3;
          }
          piVar1[1] = *(int *)(iVar3 + 4);
          if (iVar3 == *(int *)(*(int *)((int)this + 4) + 4)) {
            *(int **)(*(int *)((int)this + 4) + 4) = piVar1;
          }
          else {
            piVar2 = *(int **)(iVar3 + 4);
            if (iVar3 == *piVar2) {
              *piVar2 = (int)piVar1;
            }
            else {
              piVar2[2] = (int)piVar1;
            }
          }
          *piVar1 = iVar3;
          *(int **)(iVar3 + 4) = piVar1;
          this_00 = a_Stack_14;
          goto LAB_10007020;
        }
        piVar1[7] = 1;
        *(undefined4 *)(iVar3 + 0x1c) = 1;
        *(undefined4 *)(*(int *)(piVar7[1] + 4) + 0x1c) = 0;
        piVar7 = *(int **)(piVar7[1] + 4);
      }
    } while (piVar7 != *(int **)(*(int *)((int)this + 4) + 4));
  }
  *(undefined4 *)(*(int *)(*(int *)((int)this + 4) + 4) + 0x1c) = 1;
  *param_1 = piVar6;
  uStack_4 = 0xffffffff;
  std::_Lockit::~_Lockit(local_10);
  ExceptionList = pvStack_c;
  return param_1;
}



undefined4 * __thiscall FUN_10007070(void *this,float *param_1)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  uint uVar3;
  float *pfVar4;
  _Lockit local_4 [4];
  
  std::_Lockit::_Lockit(local_4);
  puVar2 = *(undefined4 **)((int)this + 4);
  puVar1 = (undefined4 *)(*(undefined4 **)((int)this + 4))[1];
joined_r0x10007091:
  do {
    if (puVar1 == DAT_100384d4) {
      std::_Lockit::~_Lockit(local_4);
      return puVar2;
    }
    uVar3 = 0;
    pfVar4 = param_1;
    do {
      if (*(float *)((int)puVar1 + (0xc - (int)param_1) + (int)pfVar4) != *pfVar4) {
        if ((float)puVar1[uVar3 + 3] < param_1[uVar3]) {
          puVar1 = (undefined4 *)puVar1[2];
          goto joined_r0x10007091;
        }
        break;
      }
      uVar3 = uVar3 + 1;
      pfVar4 = pfVar4 + 1;
    } while (uVar3 < 3);
    puVar2 = puVar1;
    puVar1 = (undefined4 *)*puVar1;
  } while( true );
}



void __fastcall FUN_10007100(int *param_1)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  _Lockit local_14 [4];
  _Lockit local_10 [4];
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021f49;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  std::_Lockit::_Lockit(local_10);
  piVar3 = (int *)*param_1;
  local_4 = 0;
  if ((piVar3[7] == 0) && (*(int **)(piVar3[1] + 4) == piVar3)) {
    *param_1 = piVar3[2];
  }
  else {
    iVar1 = *piVar3;
    if (iVar1 == DAT_100384d4) {
      piVar3 = (int *)piVar3[1];
      if (*param_1 == *piVar3) {
        do {
          *param_1 = (int)piVar3;
          piVar3 = (int *)piVar3[1];
        } while (*param_1 == *piVar3);
      }
      *param_1 = (int)piVar3;
    }
    else {
      std::_Lockit::_Lockit(local_14);
      for (iVar2 = *(int *)(iVar1 + 8); iVar2 != DAT_100384d4; iVar2 = *(int *)(iVar2 + 8)) {
        iVar1 = iVar2;
      }
      std::_Lockit::~_Lockit(local_14);
      *param_1 = iVar1;
    }
  }
  local_4 = 0xffffffff;
  std::_Lockit::~_Lockit(local_10);
  ExceptionList = local_c;
  return;
}



void __fastcall FUN_100071c0(int *param_1)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  _Lockit local_14 [4];
  _Lockit local_10 [4];
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021f69;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  std::_Lockit::_Lockit(local_10);
  puVar1 = *(undefined4 **)(*param_1 + 8);
  local_4 = 0;
  if (puVar1 == DAT_100384d4) {
    iVar3 = *(int *)(*param_1 + 4);
    if (*param_1 == *(int *)(iVar3 + 8)) {
      do {
        *param_1 = iVar3;
        iVar3 = *(int *)(iVar3 + 4);
      } while (*param_1 == *(int *)(iVar3 + 8));
    }
    if (*(int *)(*param_1 + 8) != iVar3) {
      *param_1 = iVar3;
    }
  }
  else {
    std::_Lockit::_Lockit(local_14);
    for (puVar2 = (undefined4 *)*puVar1; puVar2 != DAT_100384d4; puVar2 = (undefined4 *)*puVar2) {
      puVar1 = puVar2;
    }
    std::_Lockit::~_Lockit(local_14);
    *param_1 = (int)puVar1;
  }
  local_4 = 0xffffffff;
  std::_Lockit::~_Lockit(local_10);
  ExceptionList = local_c;
  return;
}



void __cdecl FUN_10007270(undefined4 *param_1,undefined4 *param_2)

{
  if (param_1 != (undefined4 *)0x0) {
    *param_1 = *param_2;
    param_1[1] = param_2[1];
    param_1[2] = param_2[2];
    param_1[3] = param_2[3];
  }
  return;
}



Importer * __fastcall FUN_100072a0(Importer *param_1)

{
  srSurfaceIOManager *psVar1;
  char *pcVar2;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_10021f89;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined ***)param_1 = &PTR_LAB_1002587c;
  pcVar2 = &DAT_10035864;
  local_4 = 0;
  psVar1 = srCore::getSurfaceIOManager((srCore *)srCore_exref);
  srIOManager::Importer::addToImporters(param_1,(srIOManager *)psVar1,pcVar2);
  ExceptionList = pvStack_c;
  return param_1;
}



void __fastcall FUN_10007300(Importer *param_1)

{
  srSurfaceIOManager *psVar1;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_10021f9c;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined ***)param_1 = &PTR_LAB_1002587c;
  local_4 = 0;
  psVar1 = srCore::getSurfaceIOManager((srCore *)srCore_exref);
  srIOManager::Importer::removeFromImporters(param_1,(srIOManager *)psVar1);
  ExceptionList = pvStack_c;
  return;
}



srColorSurface * FUN_10007360(srColorSurface *param_1)

{
  ulong uVar1;
  undefined uVar2;
  undefined3 uVar3;
  srSurfaceIOManager *psVar4;
  srColorSurfaceIFace *psVar5;
  srColorSurface *psVar6;
  int iVar7;
  int iVar8;
  srColorSurface *this;
  code *pcVar9;
  char local_230 [256];
  char local_130 [256];
  ulong uStack_30;
  undefined4 uStack_2c;
  srColorSurfaceIFace *local_28;
  srColorSurface *psStack_24;
  ulong uStack_20;
  srColorSurface *local_1c;
  srColorSurfaceIFace *local_18;
  undefined *local_14;
  void *pvStack_10;
  undefined *puStack_c;
  undefined4 local_8;
  
  psVar6 = param_1;
  puStack_c = &LAB_10021fbc;
  pvStack_10 = ExceptionList;
  local_14 = &stack0xfffffdc4;
  local_28 = (srColorSurfaceIFace *)0x0;
  local_18 = (srColorSurfaceIFace *)0x0;
  local_8 = 0;
  ExceptionList = &pvStack_10;
  FUN_10007570(local_230,(srBinIStream *)param_1);
  FUN_10007570(local_130,(srBinIStream *)psVar6);
  param_1 = (srColorSurface *)0x0;
  psVar4 = srCore::getSurfaceIOManager((srCore *)srCore_exref);
  psVar5 = srSurfaceIOManager::importSurface(psVar4,local_230,(ImportInfo *)&param_1);
  local_28 = psVar5;
  if (local_130[0] == '\0') {
    local_8 = 0xffffffff;
    local_1c = (srColorSurface *)0x0;
    pcVar9 = release_exref;
    if (psVar5 != (srColorSurfaceIFace *)0x0) {
      uStack_20 = *(ulong *)(psVar5 + 0x1c);
      uVar1 = *(ulong *)(psVar5 + 0x20);
      uStack_30 = uVar1;
      psVar6 = (srColorSurface *)srHeap::allocate((srHeap *)srHeap_exref,0x5c);
      local_8 = 2;
      param_1 = psVar6;
      if (psVar6 == (srColorSurface *)0x0) {
        psVar6 = (srColorSurface *)0x0;
      }
      else {
        srColorSurface::srColorSurface(psVar6,0xe,uStack_20,uVar1);
        *(undefined ***)psVar6 = &PTR_FUN_10025890;
      }
      local_8 = 0xffffffff;
      local_1c = psVar6;
      (**(code **)(*(int *)psVar6 + 0x88))(psVar5);
      pcVar9 = release_exref;
      if (local_18 != (srColorSurfaceIFace *)0x0) {
        psStack_24 = (srColorSurface *)srHeap::allocate((srHeap *)srHeap_exref,0x5c);
        local_8 = 3;
        param_1 = psStack_24;
        if (psStack_24 == (srColorSurface *)0x0) {
          psStack_24 = (srColorSurface *)0x0;
        }
        else {
          srColorSurface::srColorSurface(psStack_24,2,uStack_20,uVar1);
          *(undefined ***)psStack_24 = &PTR_FUN_10025890;
        }
        this = psStack_24;
        local_8 = 0xffffffff;
        (**(code **)(*(int *)psStack_24 + 0x88))(local_18);
        iVar7 = 0;
        if (0 < (int)uStack_30) {
          do {
            iVar8 = 0;
            if (0 < (int)uStack_20) {
              do {
                uVar2 = (**(code **)(*(int *)psStack_24 + 0x30))(iVar8,iVar7);
                param_1 = (srColorSurface *)CONCAT13(uVar2,param_1._0_3_);
                uVar3 = (**(code **)(*(int *)psVar6 + 0x28))(iVar8,iVar7);
                uStack_2c = CONCAT13(param_1._3_1_,uVar3);
                (**(code **)(*(int *)psVar6 + 0x2c))(iVar8,iVar7,uStack_2c);
                iVar8 = iVar8 + 1;
              } while (iVar8 < (int)uStack_20);
            }
            iVar7 = iVar7 + 1;
            this = psStack_24;
          } while (iVar7 < (int)uStack_30);
        }
        pcVar9 = release_exref;
        srClass::release((srClass *)this);
      }
    }
    (*pcVar9)();
    (*pcVar9)();
    ExceptionList = pvStack_10;
    return local_1c;
  }
  local_1c = (srColorSurface *)0x0;
  psVar4 = srCore::getSurfaceIOManager((srCore *)srCore_exref);
  local_18 = srSurfaceIOManager::importSurface(psVar4,local_130,(ImportInfo *)&local_1c);
  psVar6 = (srColorSurface *)FUN_10007411();
  return psVar6;
}



undefined * Catch_10007408(void)

{
  return &DAT_1000740e;
}



undefined4 FUN_10007411(void)

{
  ulong uVar1;
  int iVar2;
  code *this;
  undefined uVar3;
  srColorSurface *this_00;
  srClass *this_01;
  undefined4 uVar4;
  int unaff_EBX;
  int iVar5;
  int unaff_EBP;
  int iVar6;
  code *pcVar7;
  
  *(undefined4 *)(unaff_EBP + -4) = 0xffffffff;
  *(undefined4 *)(unaff_EBP + -0x18) = 0;
  this = srHeap_exref;
  pcVar7 = release_exref;
  if (unaff_EBX != 0) {
    uVar1 = *(ulong *)(unaff_EBX + 0x20);
    *(undefined4 *)(unaff_EBP + -0x1c) = *(undefined4 *)(unaff_EBX + 0x1c);
    *(ulong *)(unaff_EBP + -0x2c) = uVar1;
    this_00 = (srColorSurface *)srHeap::allocate((srHeap *)this,0x5c);
    *(srColorSurface **)(unaff_EBP + 8) = this_00;
    *(undefined4 *)(unaff_EBP + -4) = 2;
    if (this_00 == (srColorSurface *)0x0) {
      this_00 = (srColorSurface *)0x0;
    }
    else {
      srColorSurface::srColorSurface(this_00,0xe,*(ulong *)(unaff_EBP + -0x1c),uVar1);
      *(undefined ***)this_00 = &PTR_FUN_10025890;
    }
    iVar5 = *(int *)this_00;
    *(undefined4 *)(unaff_EBP + -4) = 0xffffffff;
    *(srColorSurface **)(unaff_EBP + -0x18) = this_00;
    (**(code **)(iVar5 + 0x88))();
    pcVar7 = release_exref;
    if (*(int *)(unaff_EBP + -0x14) != 0) {
      this_01 = (srClass *)srHeap::allocate((srHeap *)srHeap_exref,0x5c);
      *(srClass **)(unaff_EBP + 8) = this_01;
      *(undefined4 *)(unaff_EBP + -4) = 3;
      if (this_01 == (srClass *)0x0) {
        *(undefined4 *)(unaff_EBP + -0x20) = 0;
        this_01 = *(srClass **)(unaff_EBP + -0x20);
      }
      else {
        srColorSurface::srColorSurface
                  ((srColorSurface *)this_01,2,*(ulong *)(unaff_EBP + -0x1c),uVar1);
        *(undefined ***)this_01 = &PTR_FUN_10025890;
        *(srClass **)(unaff_EBP + -0x20) = this_01;
      }
      iVar5 = *(int *)this_01;
      *(undefined4 *)(unaff_EBP + -4) = 0xffffffff;
      (**(code **)(iVar5 + 0x88))(*(undefined4 *)(unaff_EBP + -0x14));
      iVar5 = 0;
      if (0 < *(int *)(unaff_EBP + -0x2c)) {
        do {
          iVar6 = 0;
          if (0 < *(int *)(unaff_EBP + -0x1c)) {
            do {
              uVar3 = (**(code **)(**(int **)(unaff_EBP + -0x20) + 0x30))(iVar6,iVar5);
              *(undefined *)(unaff_EBP + 0xb) = uVar3;
              uVar4 = (**(code **)(*(int *)this_00 + 0x28))(iVar6,iVar5);
              iVar2 = *(int *)this_00;
              *(undefined4 *)(unaff_EBP + -0x28) = uVar4;
              *(undefined *)(unaff_EBP + -0x25) = *(undefined *)(unaff_EBP + 0xb);
              (**(code **)(iVar2 + 0x2c))(iVar6,iVar5,*(undefined4 *)(unaff_EBP + -0x28));
              iVar6 = iVar6 + 1;
            } while (iVar6 < *(int *)(unaff_EBP + -0x1c));
          }
          iVar5 = iVar5 + 1;
        } while (iVar5 < *(int *)(unaff_EBP + -0x2c));
        this_01 = *(srClass **)(unaff_EBP + -0x20);
      }
      pcVar7 = release_exref;
      srClass::release(this_01);
    }
  }
  (*pcVar7)();
  (*pcVar7)();
  ExceptionList = *(void **)(unaff_EBP + -0xc);
  return *(undefined4 *)(unaff_EBP + -0x18);
}



void FUN_10007570(char *param_1,srBinIStream *param_2)

{
  char cVar1;
  ushort uVar2;
  
  uVar2 = srBinIStream::getChar(param_2);
  if (uVar2 == 0xffff) {
    *param_1 = '\0';
    return;
  }
  while (cVar1 = (char)uVar2, cVar1 != '\r') {
    if ((cVar1 == '\n') || (cVar1 == ' ')) goto LAB_100075aa;
    *param_1 = cVar1;
    param_1 = param_1 + 1;
    uVar2 = srBinIStream::getChar(param_2);
    if (uVar2 == 0xffff) goto LAB_100075aa;
  }
  srBinIStream::getChar(param_2);
LAB_100075aa:
  *param_1 = '\0';
  return;
}



void * __thiscall FUN_100075c0(void *this,byte param_1)

{
  FUN_10007300((Importer *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void * __thiscall FUN_100075e0(void *this,byte param_1)

{
  srColorSurface::~srColorSurface((srColorSurface *)this);
  if ((param_1 & 1) != 0) {
    srHeap::free(this);
  }
  return this;
}



srColorSurface * __fastcall FUN_100076a0(srColorSurface *param_1)

{
  srColorSurface *this;
  
  this = (srColorSurface *)(**(code **)(*(int *)param_1 + 0x20))();
  srColorSurface::operator=(this,param_1);
  return this;
}



void FUN_100076d0(void)

{
                    // WARNING: Could not recover jumptable at 0x100076d5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::Init((Init *)&DAT_100384dd);
  return;
}



void FUN_100076e0(void)

{
  FUN_100214c2(FUN_100076f0);
  return;
}



void FUN_100076f0(void)

{
                    // WARNING: Could not recover jumptable at 0x100076f5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::~Init((Init *)&DAT_100384dd);
  return;
}



void FUN_10007710(void)

{
                    // WARNING: Could not recover jumptable at 0x10007715. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::_Winit((_Winit *)&DAT_100384dc);
  return;
}



void FUN_10007720(void)

{
  FUN_100214c2(FUN_10007730);
  return;
}



void FUN_10007730(void)

{
                    // WARNING: Could not recover jumptable at 0x10007735. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::~_Winit((_Winit *)&DAT_100384dc);
  return;
}



void __fastcall FUN_10007740(undefined4 *param_1)

{
  *param_1 = &PTR_LAB_1002596c;
  return;
}



void __fastcall FUN_10007750(undefined4 *param_1)

{
  *param_1 = &PTR_LAB_1002596c;
  return;
}



undefined4 * __fastcall FUN_10007760(undefined4 *param_1)

{
  FUN_10007740(param_1);
  param_1[1] = 0;
  param_1[2] = 0;
  *param_1 = &PTR_LAB_100259cc;
  return param_1;
}



void * __thiscall FUN_10007780(void *this,char *param_1)

{
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021fd8;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_10007740((undefined4 *)this);
  local_4 = 0;
  *(undefined4 *)((int)this + 4) = 0;
  *(undefined4 *)((int)this + 8) = 0;
  *(undefined ***)this = &PTR_LAB_100259cc;
  FUN_10007850(this,param_1);
  ExceptionList = local_c;
  return this;
}



void __fastcall FUN_100077e0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_10021fea;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_LAB_100259cc;
  local_4 = 0;
  if (param_1[2] != 0) {
    std::operator<<((basic_ostream<> *)cout_exref,s_closed_srFIOStream_10035a2c);
    FUN_10007880((int)param_1);
  }
  FUN_10007750(param_1);
  ExceptionList = pvStack_c;
  return;
}



void __thiscall FUN_10007850(void *this,char *param_1)

{
  FILE *pFVar1;
  
  pFVar1 = fopen(param_1,&DAT_10035a40);
  *(FILE **)((int)this + 8) = pFVar1;
  if (pFVar1 == (FILE *)0x0) {
    *(undefined4 *)((int)this + 4) = 2;
    *(undefined4 *)((int)this + 8) = 0;
  }
  return;
}



void __fastcall FUN_10007880(int param_1)

{
  if (*(FILE **)(param_1 + 8) != (FILE *)0x0) {
    fclose(*(FILE **)(param_1 + 8));
  }
  *(undefined4 *)(param_1 + 4) = 0;
  *(undefined4 *)(param_1 + 8) = 0;
  return;
}



undefined4 __fastcall FUN_10007a10(int *param_1)

{
  undefined4 uVar1;
  undefined4 uVar2;
  
  uVar1 = (**(code **)(*param_1 + 0x54))();
  (**(code **)(*param_1 + 0x50))(0);
  uVar2 = (**(code **)(*param_1 + 0x54))();
  (**(code **)(*param_1 + 0x48))(uVar1);
  return uVar2;
}



void * __thiscall FUN_10007a40(void *this,undefined4 param_1)

{
                    // WARNING: Load size is inaccurate
  (**(code **)(*this + 0x14))(param_1);
  return this;
}



void * __thiscall FUN_10007a60(void *this,undefined4 param_1)

{
                    // WARNING: Load size is inaccurate
  (**(code **)(*this + 0x10))(param_1);
  return this;
}



void * __thiscall FUN_10007a80(void *this,undefined4 param_1)

{
                    // WARNING: Load size is inaccurate
  (**(code **)(*this + 0xc))(param_1);
  return this;
}



void * __thiscall FUN_10007aa0(void *this,undefined4 param_1)

{
                    // WARNING: Load size is inaccurate
  (**(code **)(*this + 0xc))(param_1);
  return this;
}



void * __thiscall FUN_10007ac0(void *this,undefined *param_1)

{
  undefined uVar1;
  
                    // WARNING: Load size is inaccurate
  uVar1 = (**(code **)(*this + 8))();
  *param_1 = uVar1;
  return this;
}



void * __thiscall FUN_10007ae0(void *this,undefined2 *param_1)

{
  undefined2 uVar1;
  
                    // WARNING: Load size is inaccurate
  uVar1 = (**(code **)(*this + 4))();
  *param_1 = uVar1;
  return this;
}



void * __thiscall FUN_10007b00(void *this,undefined4 *param_1)

{
  undefined4 uVar1;
  
                    // WARNING: Load size is inaccurate
  uVar1 = (*(code *)**this)();
  *param_1 = uVar1;
  return this;
}



void * __thiscall FUN_10007b40(void *this,char *param_1)

{
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10021ffc;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_10007760((undefined4 *)this);
  local_4 = 0;
  *(undefined ***)this = &PTR_LAB_10025a34;
  FUN_10007b90(this,param_1);
  ExceptionList = local_c;
  return this;
}



void __thiscall FUN_10007b90(void *this,char *param_1)

{
  FILE *pFVar1;
  undefined4 *puVar2;
  basic_ostream<> *pbVar3;
  char *pcVar4;
  char *pcVar5;
  char cVar6;
  int iVar7;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  pcVar4 = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_1002201e;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  pFVar1 = fopen(param_1,&DAT_10035a44);
  *(FILE **)((int)this + 8) = pFVar1;
  if (pFVar1 == (FILE *)0x0) {
    *(undefined4 *)((int)this + 4) = 2;
    *(undefined4 *)((int)this + 8) = 0;
    puVar2 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&param_1);
    uStack_4 = 0;
    pbVar3 = std::operator<<((basic_ostream<> *)*puVar2,s_SR__Cant_open_file_10035a74);
    cVar6 = '\0';
    pbVar3 = std::operator<<(pbVar3,pcVar4);
    std::operator<<(pbVar3,cVar6);
    iVar7 = 0x103;
    pcVar5 = s_E__work_ht_3de_file_import_sdk_s_10035a48;
    pcVar4 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(&DAT_10035a88,pcVar4,pcVar5,iVar7);
    uStack_4 = 0xffffffff;
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&param_1);
  }
  ExceptionList = pvStack_c;
  return;
}



void * __thiscall FUN_10007c60(void *this,char *param_1)

{
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10022030;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_10007760((undefined4 *)this);
  local_4 = 0;
  *(undefined ***)this = &PTR_LAB_10025a9c;
  FUN_10007cb0(this,param_1);
  ExceptionList = local_c;
  return this;
}



void __thiscall FUN_10007cb0(void *this,char *param_1)

{
  FILE *pFVar1;
  
  pFVar1 = fopen(param_1,&DAT_10035a8c);
  *(FILE **)((int)this + 8) = pFVar1;
  if (pFVar1 == (FILE *)0x0) {
    *(undefined4 *)((int)this + 4) = 2;
    *(undefined4 *)((int)this + 8) = 0;
  }
  return;
}



int * __cdecl FUN_10007d40(int *param_1,undefined4 param_2)

{
  int *piVar1;
  
  piVar1 = (int *)(**(code **)(*param_1 + 0x20))(param_2);
  piVar1 = (int *)(**(code **)(*piVar1 + 0x20))(param_2);
  (**(code **)(*piVar1 + 0x20))(param_2);
  return param_1;
}



void FUN_10007e50(void)

{
                    // WARNING: Could not recover jumptable at 0x10007e55. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::Init((Init *)&DAT_100384e1);
  return;
}



void FUN_10007e60(void)

{
  FUN_100214c2(FUN_10007e70);
  return;
}



void FUN_10007e70(void)

{
                    // WARNING: Could not recover jumptable at 0x10007e75. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::~Init((Init *)&DAT_100384e1);
  return;
}



void FUN_10007e90(void)

{
                    // WARNING: Could not recover jumptable at 0x10007e95. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::_Winit((_Winit *)&DAT_100384e0);
  return;
}



void FUN_10007ea0(void)

{
  FUN_100214c2(FUN_10007eb0);
  return;
}



void FUN_10007eb0(void)

{
                    // WARNING: Could not recover jumptable at 0x10007eb5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::~_Winit((_Winit *)&DAT_100384e0);
  return;
}



// public: __thiscall Housemarque::Threedee_Engine::Camera_Path::Camera_Path(void)

void __thiscall Housemarque::Threedee_Engine::Camera_Path::Camera_Path(Camera_Path *this)

{
  Camera_Path local_1;
  
                    // 0x7ec0  3  ??0Camera_Path@Threedee_Engine@Housemarque@@QAE@XZ
  local_1 = SUB41((uint)this >> 0x18,0);
  this[8] = local_1;
  *(undefined4 *)(this + 0xc) = 0;
  *(undefined4 *)(this + 0x10) = 0;
  *(undefined4 *)(this + 0x14) = 0;
  *(undefined4 *)(this + 0x18) = 0;
  *(undefined4 *)(this + 0x1c) = 0;
  *(undefined ***)(this + 4) = &PTR_FUN_10025b14;
  this[0x24] = local_1;
  *(undefined4 *)(this + 0x28) = 0;
  *(undefined4 *)(this + 0x2c) = 0;
  *(undefined4 *)(this + 0x30) = 0;
  *(undefined4 *)(this + 0x34) = 0;
  *(undefined4 *)(this + 0x38) = 0;
  *(undefined ***)(this + 0x20) = &PTR_FUN_10025b14;
  this[0x40] = local_1;
  *(undefined4 *)(this + 0x44) = 0;
  *(undefined4 *)(this + 0x48) = 0;
  *(undefined4 *)(this + 0x4c) = 0;
  *(undefined4 *)(this + 0x50) = 0;
  *(undefined4 *)(this + 0x54) = 0;
  *(undefined ***)(this + 0x3c) = &PTR_FUN_10025b14;
  *(undefined4 *)(this + 0x58) = 0;
  *(undefined ***)this = &PTR_FUN_10025b0c;
  return;
}



// public: virtual __thiscall Housemarque::Threedee_Engine::Camera_Path::~Camera_Path(void)

void __thiscall Housemarque::Threedee_Engine::Camera_Path::~Camera_Path(Camera_Path *this)

{
  int *piVar1;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0x7f30  28  ??1Camera_Path@Threedee_Engine@Housemarque@@UAE@XZ
  puStack_8 = &LAB_10022097;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(undefined ***)this = &PTR_FUN_10025b0c;
  *(undefined ***)(this + 0x3c) = &PTR_FUN_10025b14;
  piVar1 = *(int **)(this + 0x44);
  local_4._0_1_ = 2;
  local_4._1_3_ = 0;
  if (piVar1 != *(int **)(this + 0x48)) {
    do {
      if ((undefined4 *)*piVar1 != (undefined4 *)0x0) {
        (*(code *)**(undefined4 **)*piVar1)(1);
      }
      piVar1 = piVar1 + 1;
    } while (piVar1 != *(int **)(this + 0x48));
  }
  FUN_100212a4(*(void **)(this + 0x44));
  *(undefined4 *)(this + 0x44) = 0;
  *(undefined4 *)(this + 0x48) = 0;
  *(undefined4 *)(this + 0x4c) = 0;
  *(undefined ***)(this + 0x20) = &PTR_FUN_10025b14;
  piVar1 = *(int **)(this + 0x28);
  local_4 = CONCAT31(local_4._1_3_,3);
  if (piVar1 != *(int **)(this + 0x2c)) {
    do {
      if ((undefined4 *)*piVar1 != (undefined4 *)0x0) {
        (*(code *)**(undefined4 **)*piVar1)(1);
      }
      piVar1 = piVar1 + 1;
    } while (piVar1 != *(int **)(this + 0x2c));
  }
  FUN_100212a4(*(void **)(this + 0x28));
  *(undefined4 *)(this + 0x28) = 0;
  *(undefined4 *)(this + 0x2c) = 0;
  *(undefined4 *)(this + 0x30) = 0;
  *(undefined ***)(this + 4) = &PTR_FUN_10025b14;
  piVar1 = *(int **)(this + 0xc);
  local_4 = 4;
  if (piVar1 != *(int **)(this + 0x10)) {
    do {
      if ((undefined4 *)*piVar1 != (undefined4 *)0x0) {
        (*(code *)**(undefined4 **)*piVar1)(1);
      }
      piVar1 = piVar1 + 1;
    } while (piVar1 != *(int **)(this + 0x10));
  }
  FUN_100212a4(*(void **)(this + 0xc));
  *(undefined4 *)(this + 0xc) = 0;
  *(undefined4 *)(this + 0x10) = 0;
  *(undefined4 *)(this + 0x14) = 0;
  ExceptionList = local_c;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Camera_Path::Init(class
// Housemarque::Threedee_Engine::Camera_Path_Config const &)

void __fastcall Housemarque::Threedee_Engine::Camera_Path::Init(Camera_Path_Config *param_1)

{
  Camera_Path_Config *pCVar1;
  int iVar2;
  Camera_Path_Config *pCVar3;
  int iVar4;
  undefined4 uVar5;
  int *piVar6;
  int *piVar7;
  int *piVar8;
  int in_EDX;
  int *piVar9;
  Camera_Path_Config *pCVar10;
  float10 fVar11;
  int *local_3c;
  Camera_Path_Config *local_38;
  Camera_Path_Config *local_34;
  Camera_Path_Config *pCStack_30;
  int local_2c;
  Camera_Path_Config *local_28;
  int local_24;
  int local_20;
  int local_1c;
  int local_18;
  int local_14;
  int local_10;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0x8060  107
                    // ?Init@Camera_Path@Threedee_Engine@Housemarque@@QAIXABVCamera_Path_Config@23@@Z
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10022147;
  pvStack_c = ExceptionList;
  piVar6 = *(int **)(in_EDX + 8);
  piVar7 = (int *)*piVar6;
  local_38 = param_1;
  local_34 = (Camera_Path_Config *)piVar7;
  if (piVar7 == piVar6) {
    ExceptionList = &pvStack_c;
    local_3c = (int *)operator_new(0x1c);
    local_28 = (Camera_Path_Config *)local_3c;
    if (local_3c == (int *)0x0) {
      local_3c = (int *)0x0;
    }
    else {
      local_18 = 0;
      local_14 = 0;
      local_10 = 0;
      local_24 = 0;
      local_20 = 0;
      local_1c = 0;
      *local_3c = (int)&PTR_LAB_10025b3c;
      local_4 = 1;
      FUN_10005d90(local_3c + 1,0xc,2,&LAB_100098c0);
      local_3c[1] = local_24;
      local_3c[2] = local_20;
      local_3c[4] = local_18;
      local_3c[3] = local_1c;
      local_3c[5] = local_14;
      *local_3c = (int)&PTR_FUN_10025b1c;
      local_3c[6] = local_10;
    }
    local_4 = 0xffffffff;
    iVar4 = FUN_10009f30((int)(param_1 + 8));
    if (iVar4 != 0) {
      iVar4 = *local_3c;
      uVar5 = FUN_10009ab0(param_1 + 4,&local_18,*(int *)(param_1 + 0x18) + -1);
      (**(code **)(iVar4 + 0x14))(uVar5);
    }
    iVar4 = (**(code **)(*local_3c + 8))();
    *(int *)(param_1 + 0x18) = *(int *)(param_1 + 0x18) + iVar4;
    fVar11 = (float10)(**(code **)(*local_3c + 0x18))();
    *(float *)(param_1 + 0x1c) = (float)(fVar11 + (float10)*(float *)(param_1 + 0x1c));
    FUN_10009f60(param_1 + 8,*(undefined4 **)(param_1 + 0x10),&local_3c);
    ExceptionList = pvStack_c;
    return;
  }
  piVar9 = (int *)*piVar7;
  if (piVar9 == piVar6) {
    ExceptionList = &pvStack_c;
    local_3c = (int *)operator_new(0x1c);
    local_28 = (Camera_Path_Config *)local_3c;
    if (local_3c == (int *)0x0) {
      local_3c = (int *)0x0;
    }
    else {
      *local_3c = (int)&PTR_LAB_10025b3c;
      local_4 = 3;
      FUN_10005d90(local_3c + 1,0xc,2,&LAB_100098c0);
      *local_3c = (int)&PTR_FUN_10025b1c;
      iVar4 = piVar7[3];
      iVar2 = piVar7[4];
      local_3c[1] = piVar7[2];
      local_3c[2] = iVar4;
      local_3c[3] = iVar2;
      iVar4 = piVar7[3];
      iVar2 = piVar7[4];
      local_3c[4] = piVar7[2];
      local_3c[5] = iVar4;
      local_3c[6] = iVar2;
      param_1 = local_38;
    }
    local_4 = 0xffffffff;
    iVar4 = FUN_10009f30((int)(param_1 + 8));
    if (iVar4 != 0) {
      iVar4 = *local_3c;
      uVar5 = FUN_10009ab0(param_1 + 4,&local_18,*(int *)(param_1 + 0x18) + -1);
      (**(code **)(iVar4 + 0x14))(uVar5);
    }
    iVar4 = (**(code **)(*local_3c + 8))();
    *(int *)(param_1 + 0x18) = *(int *)(param_1 + 0x18) + iVar4;
    fVar11 = (float10)(**(code **)(*local_3c + 0x18))();
    *(float *)(param_1 + 0x1c) = (float)(fVar11 + (float10)*(float *)(param_1 + 0x1c));
    FUN_10009f60(param_1 + 8,*(undefined4 **)(param_1 + 0x10),&local_3c);
    piVar6 = (int *)operator_new(0x1c);
    pCVar10 = local_34;
    local_28 = (Camera_Path_Config *)piVar6;
    if (piVar6 == (int *)0x0) {
      local_3c = (int *)0x0;
    }
    else {
      piVar7 = (int *)((int)local_34 + 0x14);
      *piVar6 = (int)&PTR_LAB_10025b3c;
      local_4 = 5;
      FUN_10005d90(piVar6 + 1,0xc,2,&LAB_100098c0);
      *piVar6 = (int)&PTR_FUN_10025b1c;
      iVar4 = *(int *)((int)pCVar10 + 0x18);
      iVar2 = *(int *)((int)pCVar10 + 0x1c);
      piVar6[1] = *piVar7;
      piVar6[2] = iVar4;
      piVar6[3] = iVar2;
      iVar4 = *(int *)((int)pCVar10 + 0x18);
      iVar2 = *(int *)((int)pCVar10 + 0x1c);
      piVar6[4] = *piVar7;
      piVar6[5] = iVar4;
      piVar6[6] = iVar2;
      local_3c = piVar6;
    }
    local_4 = 0xffffffff;
    iVar4 = FUN_10009f30((int)(param_1 + 8));
    if (iVar4 != 0) {
      iVar4 = *local_3c;
      uVar5 = FUN_10009ab0(param_1 + 4,&local_18,*(int *)(param_1 + 0x18) + -1);
      (**(code **)(iVar4 + 0x14))(uVar5);
    }
    iVar4 = (**(code **)(*local_3c + 8))();
    *(int *)(param_1 + 0x18) = *(int *)(param_1 + 0x18) + iVar4;
    fVar11 = (float10)(**(code **)(*local_3c + 0x18))();
    *(float *)(param_1 + 0x1c) = (float)(fVar11 + (float10)*(float *)(param_1 + 0x1c));
    FUN_10009f60(param_1 + 8,*(undefined4 **)(param_1 + 0x10),&local_3c);
    piVar6 = (int *)operator_new(0x1c);
    pCVar10 = local_34;
    local_28 = (Camera_Path_Config *)piVar6;
    if (piVar6 == (int *)0x0) {
      local_3c = (int *)0x0;
    }
    else {
      piVar7 = (int *)((int)local_34 + 0x20);
      *piVar6 = (int)&PTR_LAB_10025b3c;
      local_4 = 7;
      FUN_10005d90(piVar6 + 1,0xc,2,&LAB_100098c0);
      *piVar6 = (int)&PTR_FUN_10025b1c;
      iVar4 = *(int *)((int)pCVar10 + 0x24);
      iVar2 = *(int *)((int)pCVar10 + 0x28);
      piVar6[1] = *piVar7;
      piVar6[2] = iVar4;
      piVar6[3] = iVar2;
      iVar4 = *(int *)((int)pCVar10 + 0x24);
      iVar2 = *(int *)((int)pCVar10 + 0x28);
      piVar6[4] = *piVar7;
      piVar6[5] = iVar4;
      piVar6[6] = iVar2;
      local_3c = piVar6;
    }
    pCVar3 = local_38;
    pCVar10 = local_38 + 0x3c;
    pCVar1 = local_38 + 0x40;
    local_4 = 0xffffffff;
    iVar4 = FUN_10009f30((int)pCVar1);
    if (iVar4 != 0) {
      iVar4 = *local_3c;
      uVar5 = FUN_10009ab0(pCVar10,&local_18,*(int *)(pCVar3 + 0x50) + -1);
      (**(code **)(iVar4 + 0x14))(uVar5);
    }
    iVar4 = (**(code **)(*local_3c + 8))();
    *(int *)(pCVar3 + 0x50) = *(int *)(pCVar3 + 0x50) + iVar4;
    fVar11 = (float10)(**(code **)(*local_3c + 0x18))();
    *(float *)(pCVar3 + 0x54) = (float)(fVar11 + (float10)*(float *)(pCVar3 + 0x54));
    FUN_10009f60(pCVar1,*(undefined4 **)(pCVar3 + 0x48),&local_3c);
    ExceptionList = pvStack_c;
    return;
  }
  ExceptionList = &pvStack_c;
  local_2c = in_EDX;
  local_3c = (int *)operator_new(0x1c);
  if (local_3c == (int *)0x0) {
    local_3c = (int *)0x0;
  }
  else {
    *local_3c = (int)&PTR_LAB_10025b3c;
    local_4 = 9;
    local_28 = (Camera_Path_Config *)local_3c;
    FUN_10005d90(local_3c + 1,0xc,2,&LAB_100098c0);
    *local_3c = (int)&PTR_FUN_10025b1c;
    iVar4 = piVar7[3];
    iVar2 = piVar7[4];
    local_3c[1] = piVar7[2];
    local_3c[2] = iVar4;
    local_3c[3] = iVar2;
    iVar4 = piVar9[3];
    iVar2 = piVar9[4];
    local_3c[4] = piVar9[2];
    local_3c[5] = iVar4;
    local_3c[6] = iVar2;
    param_1 = local_38;
  }
  pCVar10 = param_1 + 8;
  local_4 = 0xffffffff;
  local_34 = pCVar10;
  local_28 = param_1 + 4;
  iVar4 = FUN_10009f30((int)pCVar10);
  if (iVar4 != 0) {
    iVar4 = *local_3c;
    uVar5 = FUN_10009ab0(param_1 + 4,&local_18,*(int *)(param_1 + 0x18) + -1);
    (**(code **)(iVar4 + 0x14))(uVar5);
    pCVar10 = local_34;
  }
  iVar4 = (**(code **)(*local_3c + 8))();
  *(int *)(param_1 + 0x18) = *(int *)(param_1 + 0x18) + iVar4;
  fVar11 = (float10)(**(code **)(*local_3c + 0x18))();
  *(float *)(param_1 + 0x1c) = (float)(fVar11 + (float10)*(float *)(param_1 + 0x1c));
  FUN_10009f60(pCVar10,*(undefined4 **)(pCVar10 + 8),&local_3c);
  piVar6 = (int *)operator_new(0x1c);
  pCStack_30 = (Camera_Path_Config *)piVar6;
  if (piVar6 == (int *)0x0) {
    local_3c = (int *)0x0;
  }
  else {
    *piVar6 = (int)&PTR_LAB_10025b3c;
    local_4 = 0xb;
    FUN_10005d90(piVar6 + 1,0xc,2,&LAB_100098c0);
    *piVar6 = (int)&PTR_FUN_10025b1c;
    iVar4 = piVar7[6];
    iVar2 = piVar7[7];
    piVar6[1] = piVar7[5];
    piVar6[2] = iVar4;
    piVar6[3] = iVar2;
    iVar4 = piVar9[6];
    iVar2 = piVar9[7];
    piVar6[4] = piVar9[5];
    piVar6[5] = iVar4;
    piVar6[6] = iVar2;
    local_3c = piVar6;
  }
  pCVar3 = local_38;
  pCVar10 = local_38 + 0x20;
  pCVar1 = local_38 + 0x24;
  local_4 = 0xffffffff;
  local_34 = pCVar10;
  iVar4 = FUN_10009f30((int)pCVar1);
  if (iVar4 != 0) {
    pCStack_30 = (Camera_Path_Config *)*local_3c;
    uVar5 = FUN_10009ab0(pCVar10,&local_18,*(int *)(pCVar3 + 0x34) + -1);
    (**(code **)((int)pCStack_30 + 0x14))(uVar5);
  }
  iVar4 = (**(code **)(*local_3c + 8))();
  *(int *)(pCVar3 + 0x34) = *(int *)(pCVar3 + 0x34) + iVar4;
  fVar11 = (float10)(**(code **)(*local_3c + 0x18))();
  *(float *)(pCVar3 + 0x38) = (float)(fVar11 + (float10)*(float *)(pCVar3 + 0x38));
  FUN_10009f60(pCVar1,*(undefined4 **)(pCVar3 + 0x2c),&local_3c);
  piVar6 = (int *)operator_new(0x1c);
  if (piVar6 == (int *)0x0) {
    piVar6 = (int *)0x0;
  }
  else {
    *piVar6 = (int)&PTR_LAB_10025b3c;
    local_4 = 0xd;
    pCStack_30 = (Camera_Path_Config *)piVar6;
    FUN_10005d90(piVar6 + 1,0xc,2,&LAB_100098c0);
    *piVar6 = (int)&PTR_FUN_10025b1c;
    iVar4 = piVar7[9];
    iVar2 = piVar7[10];
    piVar6[1] = piVar7[8];
    piVar6[2] = iVar4;
    piVar6[3] = iVar2;
    iVar4 = piVar9[9];
    iVar2 = piVar9[10];
    piVar6[4] = piVar9[8];
    piVar6[5] = iVar4;
    piVar6[6] = iVar2;
    pCVar10 = local_34;
  }
  pCStack_30 = local_38 + 0x3c;
  local_4 = 0xffffffff;
  FUN_10009b50(pCStack_30,piVar6,1);
  piVar6 = (int *)*piVar9;
  local_34 = (Camera_Path_Config *)(local_2c + 4);
  piVar7 = (int *)FUN_10009f50(local_34,&local_2c);
  if (piVar6 != (int *)*piVar7) {
    do {
      local_38 = (Camera_Path_Config *)operator_new(0x1c);
      local_4 = 0xe;
      if (local_38 == (Camera_Path_Config *)0x0) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = (int *)FUN_10009900(local_38,piVar9 + 2,piVar6 + 2);
      }
      local_4 = 0xffffffff;
      FUN_10009b50(local_28,piVar7,1);
      local_38 = (Camera_Path_Config *)operator_new(0x1c);
      local_4 = 0xf;
      if (local_38 == (Camera_Path_Config *)0x0) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = (int *)FUN_10009900(local_38,piVar9 + 5,piVar6 + 5);
      }
      local_4 = 0xffffffff;
      FUN_10009b50(pCVar10,piVar7,1);
      local_38 = (Camera_Path_Config *)operator_new(0x1c);
      local_4 = 0x10;
      if (local_38 == (Camera_Path_Config *)0x0) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = (int *)FUN_10009900(local_38,piVar9 + 8,piVar6 + 8);
      }
      local_4 = 0xffffffff;
      FUN_10009b50(pCStack_30,piVar7,1);
      piVar7 = (int *)*piVar6;
      piVar8 = (int *)FUN_10009f50(local_34,&local_2c);
      piVar9 = piVar6;
      piVar6 = piVar7;
    } while (piVar7 != (int *)*piVar8);
  }
  ExceptionList = pvStack_c;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Camera_Path::Position(float)

void __fastcall Housemarque::Threedee_Engine::Camera_Path::Position(float param_1)

{
  undefined4 in_stack_00000004;
  
                    // 0x87c0  116  ?Position@Camera_Path@Threedee_Engine@Housemarque@@QAIXM@Z
  *(undefined4 *)((int)param_1 + 0x58) = in_stack_00000004;
  return;
}



// public: float __fastcall Housemarque::Threedee_Engine::Camera_Path::Position(void)const 

float __fastcall Housemarque::Threedee_Engine::Camera_Path::Position(void)

{
  int in_ECX;
  
                    // 0x87d0  117  ?Position@Camera_Path@Threedee_Engine@Housemarque@@QBIMXZ
  return *(float *)(in_ECX + 0x58);
}



// public: class Housemarque::Template_Library::Vector_3<float> __fastcall
// Housemarque::Threedee_Engine::Camera_Path::Camera_Location(void)const 

undefined4 * __fastcall
Housemarque::Threedee_Engine::Camera_Path::Camera_Location(int param_1,undefined4 *param_2)

{
  float fVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 *puVar4;
  int iVar5;
  float10 extraout_ST0;
  uint local_18 [3];
  undefined local_c [12];
  
                    // 0x87e0  59
                    // ?Camera_Location@Camera_Path@Threedee_Engine@Housemarque@@QBI?AV?$Vector_3@M@Template_Library@3@XZ
  fVar1 = *(float *)(param_1 + 0x58);
  if ((*(int *)(param_1 + 0xc) != 0) &&
     ((*(int *)(param_1 + 0x10) - *(int *)(param_1 + 0xc) & 0xfffffffcU) != 0)) {
    if (0.0 <= fVar1) {
      if (*(int *)(param_1 + 0xc) == 0) {
        local_18[0] = 0;
      }
      else {
        local_18[0] = *(int *)(param_1 + 0x10) - *(int *)(param_1 + 0xc) >> 2;
      }
      local_18[1] = 0;
      local_18[0] = _ftol();
      if ((*(int *)(param_1 + 0xc) == 0) ||
         ((uint)(*(int *)(param_1 + 0x10) - *(int *)(param_1 + 0xc) >> 2) <= local_18[0])) {
        if (*(int *)(param_1 + 0xc) == 0) {
          iVar5 = 0;
        }
        else {
          iVar5 = *(int *)(param_1 + 0x10) - *(int *)(param_1 + 0xc) >> 2;
        }
        local_18[0] = iVar5 - 1;
      }
      puVar4 = (undefined4 *)
               (**(code **)(**(int **)(*(int *)(param_1 + 0xc) + local_18[0] * 4) + 4))
                         (local_c,(float)(((float10)fVar1 - (float10)(int)local_18[0] * extraout_ST0
                                          ) / extraout_ST0));
    }
    else {
      puVar4 = (undefined4 *)
               (**(code **)(*(int *)**(undefined4 **)(param_1 + 0xc) + 4))(local_18,fVar1);
    }
    uVar2 = puVar4[1];
    uVar3 = puVar4[2];
    *param_2 = *puVar4;
    param_2[1] = uVar2;
    param_2[2] = uVar3;
    return param_2;
  }
  *param_2 = 0x40c00000;
  param_2[1] = 0x40c00000;
  param_2[2] = 0x40c00000;
  return param_2;
}



// public: void __fastcall Housemarque::Threedee_Engine::Camera_Path::Set_Camera(class
// Housemarque::Threedee_Engine::Scene *,class Housemarque::Template_Library::Vector_3<float> const
// &,class Housemarque::Template_Library::Vector_3<float> const &,float)

void __fastcall
Housemarque::Threedee_Engine::Camera_Path::Set_Camera
          (Scene *param_1,Vector_3<float> *param_2,Vector_3<float> *param_3,float param_4)

{
  float fVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  float *pfVar5;
  int iVar6;
  float *pfVar7;
  float in_stack_0000000c;
  float local_88;
  float local_84;
  float local_80;
  float local_78;
  float local_74;
  float local_70;
  float local_6c;
  float local_68;
  float local_64;
  undefined local_60 [12];
  undefined local_54 [12];
  float local_48 [9];
  float local_24 [9];
  
                    // 0x8910  131
                    // ?Set_Camera@Camera_Path@Threedee_Engine@Housemarque@@QAIXPAVScene@23@ABV?$Vector_3@M@Template_Library@3@1M@Z
  fVar1 = *(float *)(param_1 + 0x58);
  if ((*(int *)(param_1 + 0xc) == 0) ||
     ((*(int *)(param_1 + 0x10) - *(int *)(param_1 + 0xc) & 0xfffffffcU) == 0)) {
    local_88 = 6.0;
    local_84 = 6.0;
    local_80 = 6.0;
  }
  else {
    if (0.0 <= fVar1) {
      if (*(int *)(param_1 + 0xc) == 0) {
        uVar2 = 0;
      }
      else {
        uVar2 = *(int *)(param_1 + 0x10) - *(int *)(param_1 + 0xc) >> 2;
      }
      uVar3 = _ftol();
      uVar4 = FUN_10009f30((int)(param_1 + 8));
      if (uVar4 <= uVar3) {
        iVar6 = FUN_10009f30((int)(param_1 + 8));
        uVar3 = iVar6 - 1;
      }
      pfVar5 = (float *)(**(code **)(**(int **)(*(int *)(param_1 + 0xc) + uVar3 * 4) + 4))
                                  (local_60,(fVar1 - (float)(int)uVar3 * (1.0 / (float)uVar2)) /
                                            (1.0 / (float)uVar2));
    }
    else {
      pfVar5 = (float *)(**(code **)(*(int *)**(undefined4 **)(param_1 + 0xc) + 4))(local_54,fVar1);
    }
    local_88 = *pfVar5;
    local_84 = pfVar5[1];
    local_80 = pfVar5[2];
  }
  local_74 = local_84 + *(float *)(param_3 + 4);
  fVar1 = *(float *)(param_1 + 0x58);
  local_70 = local_80 + *(float *)(param_3 + 8);
  local_78 = local_88 + *(float *)param_3;
  if ((*(int *)(param_1 + 0x28) == 0) ||
     ((*(int *)(param_1 + 0x2c) - *(int *)(param_1 + 0x28) & 0xfffffffcU) == 0)) {
    local_88 = 6.0;
    local_84 = 6.0;
    local_80 = 6.0;
  }
  else {
    if (0.0 <= fVar1) {
      if (*(int *)(param_1 + 0x28) == 0) {
        uVar2 = 0;
      }
      else {
        uVar2 = *(int *)(param_1 + 0x2c) - *(int *)(param_1 + 0x28) >> 2;
      }
      uVar3 = _ftol();
      uVar4 = FUN_10009f30((int)(param_1 + 0x24));
      if (uVar4 <= uVar3) {
        iVar6 = FUN_10009f30((int)(param_1 + 0x24));
        uVar3 = iVar6 - 1;
      }
      pfVar5 = (float *)(**(code **)(**(int **)(*(int *)(param_1 + 0x28) + uVar3 * 4) + 4))
                                  (local_54,(fVar1 - (float)(int)uVar3 * (1.0 / (float)uVar2)) /
                                            (1.0 / (float)uVar2));
    }
    else {
      pfVar5 = (float *)(**(code **)(*(int *)**(undefined4 **)(param_1 + 0x28) + 4))(local_60,fVar1)
      ;
    }
    local_88 = *pfVar5;
    local_84 = pfVar5[1];
    local_80 = pfVar5[2];
  }
  local_68 = local_84 + *(float *)((int)param_4 + 4);
  fVar1 = *(float *)(param_1 + 0x58);
  local_64 = local_80 + *(float *)((int)param_4 + 8);
  local_6c = local_88 + *(float *)param_4;
  if ((*(int *)(param_1 + 0x44) == 0) ||
     ((*(int *)(param_1 + 0x48) - *(int *)(param_1 + 0x44) & 0xfffffffcU) == 0)) {
    local_88 = 6.0;
  }
  else {
    if (0.0 <= fVar1) {
      if (*(int *)(param_1 + 0x44) == 0) {
        uVar2 = 0;
      }
      else {
        uVar2 = *(int *)(param_1 + 0x48) - *(int *)(param_1 + 0x44) >> 2;
      }
      uVar3 = _ftol();
      uVar4 = FUN_10009f30((int)(param_1 + 0x40));
      if (uVar4 <= uVar3) {
        iVar6 = FUN_10009f30((int)(param_1 + 0x40));
        uVar3 = iVar6 - 1;
      }
      pfVar5 = (float *)(**(code **)(**(int **)(*(int *)(param_1 + 0x44) + uVar3 * 4) + 4))
                                  (local_54,(fVar1 - (float)(int)uVar3 * (1.0 / (float)uVar2)) /
                                            (1.0 / (float)uVar2));
    }
    else {
      pfVar5 = (float *)(**(code **)(*(int *)**(undefined4 **)(param_1 + 0x44) + 4))(local_60,fVar1)
      ;
    }
    local_88 = *pfVar5;
  }
  pfVar5 = Point_Camera_To(local_24,&local_78,&local_6c,local_88 * 0.008726646 + in_stack_0000000c);
  pfVar7 = local_48;
  for (iVar6 = 9; iVar6 != 0; iVar6 = iVar6 + -1) {
    *pfVar7 = *pfVar5;
    pfVar5 = pfVar5 + 1;
    pfVar7 = pfVar7 + 1;
  }
  Scene::Set_Camera(param_2,(Matrix_3x3<float> *)&local_78);
  return;
}



// public: __thiscall
// Housemarque::Threedee_Engine::Camera_Path_Config::Cam_Values::Cam_Values(float,float,float,float,float,float,float,float)

void __thiscall
Housemarque::Threedee_Engine::Camera_Path_Config::Cam_Values::Cam_Values
          (Cam_Values *this,float param_1,float param_2,float param_3,float param_4,float param_5,
          float param_6,float param_7,float param_8)

{
                    // 0x8d00  2
                    // ??0Cam_Values@Camera_Path_Config@Threedee_Engine@Housemarque@@QAE@MMMMMMMM@Z
  *(float *)this = param_1;
  *(float *)(this + 4) = param_2;
  *(float *)(this + 8) = param_3;
  *(float *)(this + 0xc) = param_4;
  *(float *)(this + 0x10) = param_5;
  *(float *)(this + 0x14) = param_6;
  *(float *)(this + 0x18) = param_7;
  *(float *)(this + 0x1c) = param_8;
  *(undefined4 *)(this + 0x20) = 0;
  return;
}



// public: __thiscall Housemarque::Threedee_Engine::Camera_Path_Config::Camera_Path_Config(void)

Camera_Path_Config * __thiscall
Housemarque::Threedee_Engine::Camera_Path_Config::Camera_Path_Config(Camera_Path_Config *this)

{
  void *pvVar1;
  Camera_Path_Config local_1;
  
                    // 0x8d50  4  ??0Camera_Path_Config@Threedee_Engine@Housemarque@@QAE@XZ
  local_1 = SUB41((uint)this >> 0x18,0);
  this[4] = local_1;
  pvVar1 = operator_new(0x2c);
  *(void **)pvVar1 = pvVar1;
  *(void **)((int)pvVar1 + 4) = pvVar1;
  *(void **)(this + 8) = pvVar1;
  *(undefined4 *)(this + 0xc) = 0;
  *(undefined ***)this = &PTR_FUN_10025b6c;
  return this;
}



// public: virtual __thiscall
// Housemarque::Threedee_Engine::Camera_Path_Config::~Camera_Path_Config(void)

void __thiscall
Housemarque::Threedee_Engine::Camera_Path_Config::~Camera_Path_Config(Camera_Path_Config *this)

{
  int *piVar1;
  int *piVar2;
  int *piVar3;
  
                    // 0x8d80  29  ??1Camera_Path_Config@Threedee_Engine@Housemarque@@UAE@XZ
  *(undefined ***)this = &PTR_FUN_10025b6c;
  piVar1 = *(int **)(this + 8);
  piVar3 = (int *)*piVar1;
  while (piVar3 != piVar1) {
    piVar2 = (int *)*piVar3;
    *(int *)piVar3[1] = *piVar3;
    *(int *)(*piVar3 + 4) = piVar3[1];
    FUN_100212a4(piVar3);
    *(int *)(this + 0xc) = *(int *)(this + 0xc) + -1;
    piVar3 = piVar2;
  }
  FUN_100212a4(*(void **)(this + 8));
  *(undefined4 *)(this + 8) = 0;
  *(undefined4 *)(this + 0xc) = 0;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Camera_Path_Config::Load(class
// std::basic_istream<char,struct std::char_traits<char> > &)

void __fastcall Housemarque::Threedee_Engine::Camera_Path_Config::Load(basic_istream<> *param_1)

{
  char cVar1;
  byte bVar2;
  undefined4 *puVar3;
  basic_istream<> *pbVar4;
  void **ppvVar5;
  int *piVar6;
  locale *plVar7;
  int iVar8;
  undefined4 *puVar9;
  uint uVar10;
  uint uVar11;
  basic_istream<> *in_EDX;
  byte *pbVar12;
  char *pcVar13;
  byte *pbVar14;
  bool bVar15;
  bool bVar16;
  basic_string<> abStack_1e8 [4];
  byte *pbStack_1e4;
  uint uStack_1e0;
  undefined4 uStack_1dc;
  float fStack_1d8;
  float fStack_1d4;
  float fStack_1d0;
  float fStack_1cc;
  basic_istream<> *local_1c8;
  float fStack_1c4;
  float fStack_1c0;
  float fStack_1bc;
  locale local_1b8 [4];
  float fStack_1b4;
  Cam_Values aCStack_1b0 [36];
  char acStack_18c [128];
  char acStack_10c [256];
  void *local_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0x8de0  110
                    // ?Load@Camera_Path_Config@Threedee_Engine@Housemarque@@QAIXAAV?$basic_istream@DU?$char_traits@D@std@@@std@@@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10022169;
  piVar6 = (int *)(*(int *)in_EDX + 4);
  iVar8 = *(int *)(in_EDX + *piVar6 + 4);
  ppvVar5 = &local_c;
  local_1c8 = param_1;
  local_c = ExceptionList;
  do {
    ExceptionList = ppvVar5;
    if (iVar8 != 0) {
      ExceptionList = local_c;
      return;
    }
    plVar7 = (locale *)std::ios_base::getloc((ios_base *)(in_EDX + *piVar6),local_1b8);
    uStack_4 = 0;
    FUN_1000a1a0(plVar7,0,(char *)0x1);
    uStack_4 = 0xffffffff;
    std::locale::~locale(local_1b8);
    std::basic_istream<>::getline(in_EDX,acStack_10c,0x100,'\n');
    iVar8 = sscanf(acStack_10c,s__s__f__f__f__f__f__f__f__f_10035c00,acStack_18c,&fStack_1d0,
                   &fStack_1d8,&fStack_1d4,&fStack_1c4,&fStack_1cc,&fStack_1b4,&fStack_1bc,
                   &fStack_1c0);
    if (iVar8 == 9) {
      uVar10 = 0xffffffff;
      pcVar13 = acStack_18c;
      do {
        if (uVar10 == 0) break;
        uVar10 = uVar10 - 1;
        cVar1 = *pcVar13;
        pcVar13 = pcVar13 + 1;
      } while (cVar1 != '\0');
      pbStack_1e4 = (byte *)0x0;
      uStack_1e0 = 0;
      uStack_1dc = 0;
      std::basic_string<>::assign(abStack_1e8,acStack_18c,~uVar10 - 1);
      uVar10 = 0xffffffff;
      pcVar13 = s_camera_10035c1c;
      do {
        if (uVar10 == 0) break;
        uVar10 = uVar10 - 1;
        cVar1 = *pcVar13;
        pcVar13 = pcVar13 + 1;
      } while (cVar1 != '\0');
      uVar11 = ~uVar10 - 1;
      uStack_4 = 1;
      uVar10 = uStack_1e0;
      if (uVar11 <= uStack_1e0) {
        uVar10 = uVar11;
      }
      bVar15 = false;
      iVar8 = 0;
      bVar16 = true;
      pbVar12 = pbStack_1e4;
      pbVar14 = (byte *)s_camera_10035c1c;
      do {
        if (uVar10 == 0) break;
        uVar10 = uVar10 - 1;
        bVar15 = *pbVar12 < *pbVar14;
        bVar16 = *pbVar12 == *pbVar14;
        pbVar12 = pbVar12 + 1;
        pbVar14 = pbVar14 + 1;
      } while (bVar16);
      if (!bVar16) {
        iVar8 = (1 - (uint)bVar15) - (uint)(bVar15 != 0);
      }
      if (((iVar8 == 0) && (uVar11 <= uStack_1e0)) && (uStack_1e0 == uVar11)) {
        puVar9 = (undefined4 *)
                 Cam_Values::Cam_Values
                           (aCStack_1b0,fStack_1d0,-fStack_1d4,fStack_1d8,fStack_1c4,-fStack_1b4,
                            fStack_1cc,fStack_1bc,fStack_1c0);
        pbVar4 = local_1c8;
        puVar3 = *(undefined4 **)(local_1c8 + 8);
        iVar8 = FUN_1000a110(puVar3,puVar3[1]);
        puVar3[1] = iVar8;
        **(int **)(iVar8 + 4) = iVar8;
        FUN_1000a410((undefined4 *)(iVar8 + 8),puVar9);
        *(int *)(pbVar4 + 0xc) = *(int *)(pbVar4 + 0xc) + 1;
        uStack_4 = 0xffffffff;
        if (pbStack_1e4 != (byte *)0x0) {
          bVar2 = pbStack_1e4[-1];
          if ((bVar2 == 0) || (bVar2 == 0xff)) {
            FUN_100212a4(pbStack_1e4 + -1);
          }
          else {
            pbStack_1e4[-1] = bVar2 - 1;
          }
        }
        pbStack_1e4 = (byte *)0x0;
        uStack_1e0 = 0;
        uStack_1dc = 0;
      }
      else {
        uStack_4 = 0xffffffff;
        std::basic_string<>::_Tidy(abStack_1e8,true);
      }
    }
    iVar8 = *(int *)(in_EDX + *(int *)(*(int *)in_EDX + 4) + 4);
    piVar6 = (int *)(*(int *)in_EDX + 4);
    ppvVar5 = (void **)ExceptionList;
  } while( true );
}



// class std::basic_ostream<char,struct std::char_traits<char> > & __fastcall
// Housemarque::Threedee_Engine::operator<<(class std::basic_ostream<char,struct
// std::char_traits<char> > &,class Housemarque::Threedee_Engine::Camera_Path const &)

basic_ostream<> * __fastcall
Housemarque::Threedee_Engine::operator<<(basic_ostream<> *param_1,Camera_Path *param_2)

{
  Camera_Path *pCVar1;
  ulonglong uVar2;
  basic_ostream<> *pbVar3;
  int iVar4;
  uint *puVar5;
  int iVar6;
  uint uVar7;
  uint uVar8;
  undefined *puVar9;
  undefined4 *puVar10;
  Vector_3<float> *pVVar11;
  char *pcVar12;
  char *pcVar13;
  char *pcVar14;
  char *pcVar15;
  float fVar16;
  float fStack_98;
  int iStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  basic_ostream<> *local_58;
  uint uStack_54;
  uint uStack_50;
  uint uStack_4c;
  uint uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined auStack_3c [12];
  undefined auStack_30 [12];
  undefined auStack_24 [12];
  undefined auStack_18 [12];
  undefined4 auStack_c [3];
  
  local_58 = param_1;
                    // 0x9030  46
                    // ??6Threedee_Engine@Housemarque@@YIAAV?$basic_ostream@DU?$char_traits@D@std@@@std@@AAV23@ABVCamera_Path@01@@Z
  std::operator<<(param_1,s_Camera_path__10035c24);
  iVar6 = *(int *)(param_2 + 0x18);
  pcVar14 = &DAT_10035c34;
  pbVar3 = std::operator<<(param_1,s_control_points__10035c38);
  pbVar3 = std::basic_ostream<>::operator<<(pbVar3,iVar6);
  std::operator<<(pbVar3,pcVar14);
  fStack_98 = 0.0;
  iStack_8c = 0;
  if (0 < *(int *)(param_2 + 0x18)) {
    do {
      puVar10 = *(undefined4 **)(param_2 + 0x44);
      iVar6 = 0;
      if (puVar10 != *(undefined4 **)(param_2 + 0x48)) {
        do {
          iVar4 = (**(code **)(*(int *)*puVar10 + 8))();
          if (iStack_8c < iVar4 + iVar6) {
            puVar10 = (undefined4 *)
                      (**(code **)(*(int *)*puVar10 + 0x10))(auStack_3c,iStack_8c - iVar6);
            uStack_7c = *puVar10;
            uStack_78 = puVar10[1];
            uStack_74 = puVar10[2];
            goto LAB_100090df;
          }
          puVar10 = puVar10 + 1;
          iVar6 = iVar4 + iVar6;
        } while (puVar10 != *(undefined4 **)(param_2 + 0x48));
      }
      uStack_48 = 0;
      uStack_44 = 0;
      uStack_40 = 0;
      uStack_74 = 0;
      uStack_7c = 0;
      uStack_78 = 0;
LAB_100090df:
      puVar10 = *(undefined4 **)(param_2 + 0x28);
      iVar6 = 0;
      if (puVar10 != *(undefined4 **)(param_2 + 0x2c)) {
        do {
          iVar4 = (**(code **)(*(int *)*puVar10 + 8))();
          if (iStack_8c < iVar4 + iVar6) {
            puVar10 = (undefined4 *)
                      (**(code **)(*(int *)*puVar10 + 0x10))(auStack_24,iStack_8c - iVar6);
            uStack_88 = *puVar10;
            uStack_84 = puVar10[1];
            uStack_80 = puVar10[2];
            goto LAB_10009181;
          }
          puVar10 = puVar10 + 1;
          iVar6 = iVar4 + iVar6;
        } while (puVar10 != *(undefined4 **)(param_2 + 0x2c));
      }
      uStack_64 = 0;
      uStack_60 = 0;
      uStack_5c = 0;
      uStack_80 = 0;
      uStack_88 = 0;
      uStack_84 = 0;
LAB_10009181:
      puVar10 = *(undefined4 **)(param_2 + 0xc);
      iVar6 = 0;
      if (puVar10 != *(undefined4 **)(param_2 + 0x10)) {
        do {
          iVar4 = (**(code **)(*(int *)*puVar10 + 8))();
          if (iStack_8c < iVar4 + iVar6) {
            puVar5 = (uint *)(**(code **)(*(int *)*puVar10 + 0x10))(auStack_30,iStack_8c - iVar6);
            uStack_54 = *puVar5;
            uStack_50 = puVar5[1];
            uStack_4c = puVar5[2];
            goto LAB_100091da;
          }
          puVar10 = puVar10 + 1;
          iVar6 = iVar4 + iVar6;
        } while (puVar10 != *(undefined4 **)(param_2 + 0x10));
      }
      uStack_70 = 0;
      uStack_6c = 0;
      uStack_68 = 0;
      uStack_4c = 0;
      uStack_54 = 0;
      uStack_50 = 0;
LAB_100091da:
      pcVar15 = &DAT_10035c48;
      pcVar13 = s___Camprop__10035c4c;
      pcVar12 = s___Tarpos__10035c58;
      pcVar14 = s___Campos__10035c64;
      pbVar3 = std::basic_ostream<>::operator<<(local_58,iStack_8c);
      pbVar3 = std::operator<<(pbVar3,pcVar14);
      pbVar3 = Housemarque::Template_Library::operator<<(pbVar3,(Vector_3<float> *)&uStack_54);
      pbVar3 = std::operator<<(pbVar3,pcVar12);
      pbVar3 = Housemarque::Template_Library::operator<<(pbVar3,(Vector_3<float> *)&uStack_88);
      pbVar3 = std::operator<<(pbVar3,pcVar13);
      pbVar3 = Housemarque::Template_Library::operator<<(pbVar3,(Vector_3<float> *)&uStack_7c);
      std::operator<<(pbVar3,pcVar15);
      iStack_8c = iStack_8c + 1;
    } while (iStack_8c < *(int *)(param_2 + 0x18));
  }
  iStack_8c = 0;
  do {
    if ((*(int *)(param_2 + 0x44) == 0) ||
       ((*(int *)(param_2 + 0x48) - *(int *)(param_2 + 0x44) & 0xfffffffcU) == 0)) {
      uStack_70 = 0x40c00000;
      uStack_6c = 0x40c00000;
      uStack_68 = 0x40c00000;
      uStack_88 = 0x40c00000;
      uStack_84 = 0x40c00000;
      uStack_80 = 0x40c00000;
    }
    else {
      if (0.0 <= fStack_98) {
        if (*(int *)(param_2 + 0x44) == 0) {
          uStack_48 = 0;
        }
        else {
          uStack_48 = *(int *)(param_2 + 0x48) - *(int *)(param_2 + 0x44) >> 2;
        }
        uStack_44 = 0;
        uVar2 = (ulonglong)uStack_48;
        uVar7 = _ftol();
        uVar8 = FUN_10009f30((int)(param_2 + 0x40));
        if (uVar8 <= uVar7) {
          iVar6 = FUN_10009f30((int)(param_2 + 0x40));
          uVar7 = iVar6 - 1;
        }
        puVar10 = (undefined4 *)
                  (**(code **)(**(int **)(*(int *)(param_2 + 0x44) + uVar7 * 4) + 4))
                            (auStack_24,
                             (fStack_98 - (float)(int)uVar7 * (1.0 / (float)uVar2)) /
                             (1.0 / (float)uVar2));
      }
      else {
        puVar10 = (undefined4 *)
                  (**(code **)(*(int *)**(undefined4 **)(param_2 + 0x44) + 4))(auStack_30,fStack_98)
        ;
      }
      uStack_88 = *puVar10;
      uStack_84 = puVar10[1];
      uStack_80 = puVar10[2];
    }
    pCVar1 = param_2 + 0x24;
    iVar6 = FUN_10009f30((int)pCVar1);
    if (iVar6 == 0) {
      uStack_64 = 0x40c00000;
      uStack_60 = 0x40c00000;
      uStack_5c = 0x40c00000;
      uStack_74 = 0x40c00000;
      uStack_7c = 0x40c00000;
      uStack_78 = 0x40c00000;
    }
    else {
      if (0.0 <= fStack_98) {
        uStack_54 = FUN_10009f30((int)pCVar1);
        uStack_50 = 0;
        uVar2 = (ulonglong)uStack_54;
        uVar7 = _ftol();
        uVar8 = FUN_10009f30((int)pCVar1);
        if (uVar8 <= uVar7) {
          iVar6 = FUN_10009f30((int)pCVar1);
          uVar7 = iVar6 - 1;
        }
        iVar6 = **(int **)(*(int *)(param_2 + 0x28) + uVar7 * 4);
        puVar9 = auStack_18;
        fVar16 = (fStack_98 - (float)(int)uVar7 * (1.0 / (float)uVar2)) / (1.0 / (float)uVar2);
      }
      else {
        iVar6 = *(int *)**(undefined4 **)(param_2 + 0x28);
        puVar9 = auStack_3c;
        fVar16 = fStack_98;
      }
      puVar10 = (undefined4 *)(**(code **)(iVar6 + 4))(puVar9,fVar16);
      uStack_7c = *puVar10;
      uStack_78 = puVar10[1];
      uStack_74 = puVar10[2];
    }
    pcVar15 = &DAT_10035c70;
    pcVar13 = s___Camprop__10035c74;
    pcVar12 = s___Tarpos__10035c80;
    pVVar11 = (Vector_3<float> *)FUN_10009dc0(param_2 + 4,auStack_c,fStack_98);
    pcVar14 = s___Campos__10035c8c;
    pbVar3 = std::basic_ostream<>::operator<<(local_58,iStack_8c);
    pbVar3 = std::operator<<(pbVar3,pcVar14);
    pbVar3 = Housemarque::Template_Library::operator<<(pbVar3,pVVar11);
    pbVar3 = std::operator<<(pbVar3,pcVar12);
    pbVar3 = Housemarque::Template_Library::operator<<(pbVar3,(Vector_3<float> *)&uStack_7c);
    pbVar3 = std::operator<<(pbVar3,pcVar13);
    pbVar3 = Housemarque::Template_Library::operator<<(pbVar3,(Vector_3<float> *)&uStack_88);
    std::operator<<(pbVar3,pcVar15);
    fStack_98 = fStack_98 + 0.1;
    iStack_8c = iStack_8c + 1;
  } while (iStack_8c < 10);
  return local_58;
}



// class Housemarque::Template_Library::Matrix_3x3<float> __fastcall
// Housemarque::Threedee_Engine::Point_Camera_To(class
// Housemarque::Template_Library::Vector_3<float> const &,class
// Housemarque::Template_Library::Vector_3<float> const &,float)

float * __fastcall
Housemarque::Threedee_Engine::Point_Camera_To
          (float *param_1,float *param_2,float *param_3,float param_4)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float extraout_ECX;
  int iVar4;
  float *pfVar5;
  float *pfVar6;
  float10 fVar7;
  float local_84;
  float local_80;
  float local_7c;
  float local_78;
  float local_74;
  float local_70;
  float local_6c;
  float local_68;
  float local_64;
  float local_60;
  float local_5c;
  float local_58;
  float fStack_54;
  float local_48;
  float local_44;
  float local_40;
  float afStack_3c [2];
  float fStack_34;
  float afStack_30 [12];
  
                    // 0x9540  115
                    // ?Point_Camera_To@Threedee_Engine@Housemarque@@YI?AV?$Matrix_3x3@M@Template_Library@2@ABV?$Vector_3@M@42@0M@Z
  local_40 = 0.0;
  fVar7 = (float10)fsin((float10)param_4 + (float10)3.1415927);
  local_48 = (float)fVar7;
  fVar7 = (float10)fcos((float10)param_4 + (float10)3.1415927);
  local_44 = (float)fVar7;
  local_74 = param_3[1] - param_2[1];
  local_70 = param_3[2] - param_2[2];
  local_78 = *param_3 - *param_2;
  if (SQRT(local_74 * local_74 + local_70 * local_70 + local_78 * local_78) == 0.0) {
    local_70 = 1.0;
  }
  Housemarque::Template_Library::V3_V3_Cross(&local_48,&local_78,&local_84);
  Housemarque::Template_Library::V3_V3_Cross(&local_78,&local_84,&local_6c);
  local_60 = local_78;
  local_5c = local_74;
  local_58 = local_70;
  if (SQRT(local_84 * local_84 + local_80 * local_80 + local_7c * local_7c) == 0.0) {
    local_84 = 1.0;
  }
  fVar1 = local_6c * local_6c + local_68 * local_68 + local_64 * local_64;
  if (SQRT(fVar1) == 0.0) {
    local_68 = 1.0;
  }
  fVar2 = local_78 * local_78 + local_74 * local_74 + local_70 * local_70;
  if (SQRT(fVar2) == 0.0) {
    local_58 = 1.0;
  }
  fStack_34 = Housemarque::Template_Library::Inv_Sqrt(fVar1);
  afStack_30[0] = fStack_34 * local_60;
  fVar1 = fStack_34 * local_5c;
  fStack_34 = fStack_34 * local_64;
  afStack_30[1] = fVar1;
  local_5c = Housemarque::Template_Library::Inv_Sqrt(extraout_ECX);
  local_58 = local_5c * local_70;
  fVar3 = local_5c * local_6c;
  local_5c = local_5c * local_74;
  fStack_54 = fVar3;
  local_48 = Housemarque::Template_Library::Inv_Sqrt(fVar3);
  local_44 = local_48 * fVar1;
  local_40 = local_48 * fVar2;
  local_48 = local_48 * fVar3;
  Housemarque::Template_Library::M3x3_V3_V3_V3_Col(&local_48,&local_60,afStack_3c,afStack_30);
  pfVar5 = afStack_30;
  pfVar6 = param_1;
  for (iVar4 = 9; iVar4 != 0; iVar4 = iVar4 + -1) {
    *pfVar6 = *pfVar5;
    pfVar5 = pfVar5 + 1;
    pfVar6 = pfVar6 + 1;
  }
  return param_1;
}



void * __thiscall FUN_10009800(void *this,byte param_1)

{
  FUN_10009820((undefined4 *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void __fastcall FUN_10009820(undefined4 *param_1)

{
  int *piVar1;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_1002204b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *param_1 = &PTR_FUN_10025b14;
  piVar1 = (int *)param_1[2];
  local_4 = 0;
  if (piVar1 != (int *)param_1[3]) {
    do {
      if ((undefined4 *)*piVar1 != (undefined4 *)0x0) {
        (*(code *)**(undefined4 **)*piVar1)(1);
      }
      piVar1 = piVar1 + 1;
    } while (piVar1 != (int *)param_1[3]);
  }
  FUN_100212a4((void *)param_1[2]);
  param_1[2] = 0;
  param_1[3] = 0;
  param_1[4] = 0;
  ExceptionList = local_c;
  return;
}



void * __thiscall FUN_100098a0(void *this,byte param_1)

{
  Housemarque::Threedee_Engine::Camera_Path::~Camera_Path((Camera_Path *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void __fastcall FUN_100098d0(undefined4 *param_1)

{
  *param_1 = &PTR_LAB_10025b3c;
  return;
}



void __thiscall FUN_10009900(void *this,undefined4 *param_1,undefined4 *param_2)

{
  undefined4 uVar1;
  undefined4 uVar2;
  
  *(undefined ***)this = &PTR_FUN_10025b1c;
  uVar1 = param_1[1];
  uVar2 = param_1[2];
  *(undefined4 *)((int)this + 4) = *param_1;
  *(undefined4 *)((int)this + 8) = uVar1;
  *(undefined4 *)((int)this + 0xc) = uVar2;
  uVar1 = param_2[1];
  uVar2 = param_2[2];
  *(undefined4 *)((int)this + 0x10) = *param_2;
  *(undefined4 *)((int)this + 0x14) = uVar1;
  *(undefined4 *)((int)this + 0x18) = uVar2;
  return;
}



undefined4 FUN_10009990(void)

{
  return 2;
}



void __thiscall FUN_10009a00(void *this,float *param_1)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  
  fVar1 = *(float *)((int)this + 8);
  fVar2 = *(float *)((int)this + 0xc);
  fVar3 = *(float *)((int)this + 4);
  *(float *)((int)this + 4) = *param_1;
  *(float *)((int)this + 8) = param_1[1];
  *(float *)((int)this + 0xc) = param_1[2];
  fVar4 = param_1[1];
  fVar5 = *param_1;
  *(float *)((int)this + 0x18) = (*(float *)((int)this + 0x18) - fVar2) + param_1[2];
  *(float *)((int)this + 0x10) = (*(float *)((int)this + 0x10) - fVar3) + fVar5;
  *(float *)((int)this + 0x14) = (*(float *)((int)this + 0x14) - fVar1) + fVar4;
  return;
}



float10 FUN_10009a70(void)

{
  return (float10)1.0;
}



void * __thiscall FUN_10009a80(void *this,byte param_1)

{
  FUN_10009aa0((undefined4 *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void __fastcall FUN_10009aa0(undefined4 *param_1)

{
  *param_1 = &PTR_LAB_10025b3c;
  return;
}



void __thiscall FUN_10009ab0(void *this,undefined4 *param_1,int param_2)

{
  int iVar1;
  undefined4 *puVar2;
  int iVar3;
  undefined local_c [8];
  undefined4 *local_4;
  
  puVar2 = *(undefined4 **)((int)this + 8);
  iVar3 = 0;
  if (puVar2 != *(undefined4 **)((int)this + 0xc)) {
    do {
      iVar1 = (**(code **)(*(int *)*puVar2 + 8))();
      if (param_2 < iVar1 + iVar3) {
        puVar2 = (undefined4 *)(**(code **)(*(int *)*puVar2 + 0x10))(local_c,param_2 - iVar3);
        *local_4 = *puVar2;
        local_4[1] = puVar2[1];
        local_4[2] = puVar2[2];
        return;
      }
      puVar2 = puVar2 + 1;
      iVar3 = iVar1 + iVar3;
    } while (puVar2 != *(undefined4 **)((int)this + 0xc));
  }
  *param_1 = 0;
  param_1[1] = 0;
  param_1[2] = 0;
  return;
}



// WARNING: Removing unreachable block (ram,0x10009d45)
// WARNING: Removing unreachable block (ram,0x10009d5c)
// WARNING: Removing unreachable block (ram,0x10009d6b)
// WARNING: Removing unreachable block (ram,0x10009d74)

void __thiscall FUN_10009b50(void *this,int *param_1,int param_2)

{
  int iVar1;
  int iVar2;
  undefined4 *puVar3;
  undefined4 *puVar4;
  undefined4 *puVar5;
  uint uVar6;
  int iVar7;
  float10 fVar8;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  undefined4 local_c;
  undefined4 local_8;
  undefined4 local_4;
  
  if (((param_2 == 1) && (puVar3 = *(undefined4 **)((int)this + 8), puVar3 != (undefined4 *)0x0)) &&
     (*(int *)((int)this + 0xc) - (int)puVar3 >> 2 != 0)) {
    iVar7 = *(int *)((int)this + 0x14) + -1;
    iVar2 = 0;
    if (puVar3 != *(undefined4 **)((int)this + 0xc)) {
      do {
        iVar1 = (**(code **)(*(int *)*puVar3 + 8))();
        if (iVar7 < iVar1 + iVar2) {
          puVar3 = (undefined4 *)(**(code **)(*(int *)*puVar3 + 0x10))(&local_c,iVar7 - iVar2);
          local_18 = *puVar3;
          local_14 = puVar3[1];
          local_10 = puVar3[2];
          goto LAB_10009bcb;
        }
        puVar3 = puVar3 + 1;
        iVar2 = iVar1 + iVar2;
      } while (puVar3 != *(undefined4 **)((int)this + 0xc));
    }
    local_c = 0;
    local_8 = 0;
    local_4 = 0;
    local_10 = 0;
    local_18 = 0;
    local_14 = 0;
LAB_10009bcb:
    (**(code **)(*param_1 + 0x14))(&local_18);
  }
  iVar2 = (**(code **)(*param_1 + 8))();
  *(int *)((int)this + 0x14) = *(int *)((int)this + 0x14) + iVar2;
  fVar8 = (float10)(**(code **)(*param_1 + 0x18))();
  *(float *)((int)this + 0x18) = (float)(fVar8 + (float10)*(float *)((int)this + 0x18));
  puVar3 = *(undefined4 **)((int)this + 0xc);
  if (*(int *)((int)this + 0x10) - (int)puVar3 >> 2 == 0) {
    iVar2 = *(int *)((int)this + 8);
    if ((iVar2 == 0) || (uVar6 = (int)puVar3 - iVar2 >> 2, uVar6 < 2)) {
      uVar6 = 1;
    }
    if (iVar2 == 0) {
      iVar2 = 0;
    }
    else {
      iVar2 = (int)puVar3 - iVar2 >> 2;
    }
    param_2 = iVar2 + uVar6;
    iVar2 = param_2;
    if (param_2 < 0) {
      iVar2 = 0;
    }
    puVar4 = (undefined4 *)operator_new(iVar2 * 4);
    puVar5 = FUN_1000a140(*(undefined4 **)((int)this + 8),puVar3,puVar4);
    FUN_1000a170(puVar5,1,&param_1);
    FUN_1000a140(puVar3,*(undefined4 **)((int)this + 0xc),puVar5 + 1);
    FUN_1000a100();
    FUN_100212a4(*(void **)((int)this + 8));
    *(undefined4 **)((int)this + 0x10) = puVar4 + param_2;
    iVar2 = FUN_10009f30((int)this + 4);
    *(undefined4 **)((int)this + 8) = puVar4;
    *(undefined4 **)((int)this + 0xc) = puVar4 + iVar2 + 1;
    return;
  }
  FUN_1000a140(puVar3,puVar3,puVar3 + 1);
  FUN_1000a170(*(undefined4 **)((int)this + 0xc),
               1 - ((int)*(undefined4 **)((int)this + 0xc) - (int)puVar3 >> 2),&param_1);
  puVar4 = *(undefined4 **)((int)this + 0xc);
  if (puVar3 == puVar4) {
    *(int *)((int)this + 0xc) = *(int *)((int)this + 0xc) + 4;
    return;
  }
  do {
    *puVar3 = param_1;
    puVar3 = puVar3 + 1;
  } while (puVar3 != puVar4);
  *(int *)((int)this + 0xc) = *(int *)((int)this + 0xc) + 4;
  return;
}



void * __thiscall FUN_10009da0(void *this,byte param_1)

{
  Housemarque::Threedee_Engine::Camera_Path_Config::~Camera_Path_Config((Camera_Path_Config *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void __thiscall FUN_10009dc0(void *this,undefined4 *param_1,float param_2)

{
  undefined4 *puVar1;
  int iVar2;
  float10 extraout_ST0;
  uint local_c [2];
  undefined4 *local_4;
  
  if ((*(int *)((int)this + 8) != 0) &&
     ((*(int *)((int)this + 0xc) - *(int *)((int)this + 8) & 0xfffffffcU) != 0)) {
    if (param_2 < 0.0) {
      puVar1 = (undefined4 *)
               (**(code **)(*(int *)**(undefined4 **)((int)this + 8) + 4))(local_c,param_2);
      *local_4 = *puVar1;
      local_4[1] = puVar1[1];
      local_4[2] = puVar1[2];
      return;
    }
    if (*(int *)((int)this + 8) == 0) {
      local_c[0] = 0;
    }
    else {
      local_c[0] = *(int *)((int)this + 0xc) - *(int *)((int)this + 8) >> 2;
    }
    local_c[1] = 0;
    local_c[0] = _ftol();
    if ((*(int *)((int)this + 8) == 0) ||
       ((uint)(*(int *)((int)this + 0xc) - *(int *)((int)this + 8) >> 2) <= local_c[0])) {
      if (*(int *)((int)this + 8) == 0) {
        iVar2 = 0;
      }
      else {
        iVar2 = *(int *)((int)this + 0xc) - *(int *)((int)this + 8) >> 2;
      }
      local_c[0] = iVar2 - 1;
    }
    puVar1 = (undefined4 *)
             (**(code **)(**(int **)(*(int *)((int)this + 8) + local_c[0] * 4) + 4))
                       (local_c,(float)(((float10)param_2 - (float10)(int)local_c[0] * extraout_ST0)
                                       / extraout_ST0));
    *local_4 = *puVar1;
    local_4[1] = puVar1[1];
    local_4[2] = puVar1[2];
    return;
  }
  *param_1 = 0x40c00000;
  param_1[1] = 0x40c00000;
  param_1[2] = 0x40c00000;
  return;
}



void __fastcall FUN_10009f00(int param_1)

{
  FUN_100212a4(*(void **)(param_1 + 4));
  *(undefined4 *)(param_1 + 4) = 0;
  *(undefined4 *)(param_1 + 8) = 0;
  *(undefined4 *)(param_1 + 0xc) = 0;
  return;
}



int __fastcall FUN_10009f30(int param_1)

{
  if (*(int *)(param_1 + 4) == 0) {
    return 0;
  }
  return *(int *)(param_1 + 8) - *(int *)(param_1 + 4) >> 2;
}



void __thiscall FUN_10009f50(void *this,undefined4 *param_1)

{
  *param_1 = *(undefined4 *)((int)this + 4);
  return;
}



undefined4 * __thiscall FUN_10009f60(void *this,undefined4 *param_1,undefined4 *param_2)

{
  int iVar1;
  int iVar2;
  undefined4 *puVar3;
  uint uVar4;
  undefined4 *puVar5;
  int iVar6;
  undefined4 *puVar7;
  
  puVar5 = *(undefined4 **)((int)this + 8);
  iVar1 = *(int *)((int)this + 4);
  iVar6 = (int)param_1 - iVar1 >> 2;
  if (*(int *)((int)this + 0xc) - (int)puVar5 >> 2 == 0) {
    if ((iVar1 == 0) || (uVar4 = (int)puVar5 - iVar1 >> 2, uVar4 < 2)) {
      uVar4 = 1;
    }
    if (iVar1 == 0) {
      iVar1 = 0;
    }
    else {
      iVar1 = (int)puVar5 - iVar1 >> 2;
    }
    iVar1 = iVar1 + uVar4;
    iVar2 = iVar1;
    if (iVar1 < 0) {
      iVar2 = 0;
    }
    puVar3 = (undefined4 *)operator_new(iVar2 * 4);
    puVar7 = puVar3;
    for (puVar5 = *(undefined4 **)((int)this + 4); puVar5 != param_1; puVar5 = puVar5 + 1) {
      FUN_1000a430(puVar7,puVar5);
      puVar7 = puVar7 + 1;
    }
    FUN_1000a430(puVar7,param_2);
    FUN_1000a140(param_1,*(undefined4 **)((int)this + 8),puVar7 + 1);
    FUN_1000a100();
    FUN_100212a4(*(void **)((int)this + 4));
    *(undefined4 **)((int)this + 0xc) = puVar3 + iVar1;
    iVar1 = FUN_10009f30((int)this);
    *(undefined4 **)((int)this + 4) = puVar3;
    *(undefined4 **)((int)this + 8) = puVar3 + iVar1 + 1;
    return puVar3 + iVar6;
  }
  if ((int)puVar5 - (int)param_1 >> 2 == 0) {
    FUN_1000a140(param_1,puVar5,param_1 + 1);
    FUN_1000a170(*(undefined4 **)((int)this + 8),
                 1 - ((int)*(undefined4 **)((int)this + 8) - (int)param_1 >> 2),param_2);
    puVar5 = *(undefined4 **)((int)this + 8);
    if (param_1 != puVar5) {
      do {
        *param_1 = *param_2;
        param_1 = param_1 + 1;
      } while (param_1 != puVar5);
    }
  }
  else {
    FUN_1000a140(puVar5 + -1,puVar5,puVar5);
    puVar5 = *(undefined4 **)((int)this + 8);
    puVar7 = puVar5;
    while (param_1 != puVar7 + -1) {
      puVar5 = puVar5 + -1;
      *puVar5 = puVar7[-2];
      puVar7 = puVar7 + -1;
    }
    puVar5 = param_1 + 1;
    if (param_1 != puVar5) {
      do {
        *param_1 = *param_2;
        param_1 = param_1 + 1;
      } while (param_1 != puVar5);
    }
  }
  *(int *)((int)this + 8) = *(int *)((int)this + 8) + 4;
  return (undefined4 *)(*(int *)((int)this + 4) + iVar6 * 4);
}



void FUN_1000a100(void)

{
  return;
}



void FUN_1000a110(undefined4 *param_1,int param_2)

{
  undefined4 *puVar1;
  
  puVar1 = (undefined4 *)operator_new(0x2c);
  if (param_1 == (undefined4 *)0x0) {
    param_1 = puVar1;
  }
  *puVar1 = param_1;
  if (param_2 != 0) {
    puVar1[1] = param_2;
    return;
  }
  puVar1[1] = puVar1;
  return;
}



undefined4 * FUN_1000a140(undefined4 *param_1,undefined4 *param_2,undefined4 *param_3)

{
  if (param_1 == param_2) {
    return param_3;
  }
  do {
    if (param_3 != (undefined4 *)0x0) {
      *param_3 = *param_1;
    }
    param_1 = param_1 + 1;
    param_3 = param_3 + 1;
  } while (param_1 != param_2);
  return param_3;
}



void FUN_1000a170(undefined4 *param_1,int param_2,undefined4 *param_3)

{
  if (param_2 != 0) {
    do {
      if (param_1 != (undefined4 *)0x0) {
        *param_1 = *param_3;
      }
      param_1 = param_1 + 1;
      param_2 = param_2 + -1;
    } while (param_2 != 0);
  }
  return;
}



facet * __cdecl FUN_1000a1a0(locale *param_1,undefined4 param_2,char *param_3)

{
  uint uVar1;
  code *pcVar2;
  locale *this;
  bool bVar3;
  facet *pfVar4;
  _Ctypevec *p_Var5;
  facet *local_64;
  _Lockit local_60 [4];
  _Ctypevec _Stack_5c;
  _Locinfo a_Stack_4c [64];
  void *pvStack_c;
  undefined *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_100221b8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  std::_Lockit::_Lockit(local_60);
  pcVar2 = id_exref;
  local_4 = 0;
  std::_Lockit::_Lockit((_Lockit *)&local_64);
  if (*(int *)pcVar2 == 0) {
    *(int *)_Id_cnt_exref = *(int *)_Id_cnt_exref + 1;
    *(undefined4 *)pcVar2 = *(undefined4 *)_Id_cnt_exref;
  }
  uVar1 = *(uint *)pcVar2;
  std::_Lockit::~_Lockit((_Lockit *)&local_64);
  this = param_1;
  pfVar4 = std::locale::_Getfacet(param_1,uVar1,true);
  if (pfVar4 == (facet *)0x0) {
    if ((char)param_3 != '\0') {
      bVar3 = std::locale::_Iscloc(this);
      if (bVar3) {
        pfVar4 = DAT_100384e8;
        if (DAT_100384e8 == (facet *)0x0) {
          pfVar4 = (facet *)operator_new(0x18);
          local_64 = pfVar4;
          if (pfVar4 == (facet *)0x0) {
            pfVar4 = (facet *)0x0;
          }
          else {
            *(undefined4 *)(pfVar4 + 4) = 0;
            *(code **)pfVar4 = _vftable__exref;
            *(code **)pfVar4 = _vftable__exref;
            *(code **)pfVar4 = _vftable__exref;
            local_4._0_1_ = 2;
            std::_Lockit::_Lockit((_Lockit *)&param_1);
            local_4._0_1_ = 3;
            std::_Locinfo::_Locinfo(a_Stack_4c,&DAT_10035d44);
            local_4 = CONCAT31(local_4._1_3_,4);
            std::_Lockit::_Lockit((_Lockit *)&param_3);
            p_Var5 = _Getctype(&_Stack_5c);
            *(uint *)(pfVar4 + 8) = p_Var5->_Page;
            *(short **)(pfVar4 + 0xc) = p_Var5->_Table;
            *(int *)(pfVar4 + 0x10) = p_Var5->_Delfl;
            *(wchar_t **)(pfVar4 + 0x14) = p_Var5->_LocaleName;
            pcVar2 = _Term_exref;
            if (*(int *)_Cltab_exref == 0) {
              *(undefined4 *)_Cltab_exref = *(undefined4 *)(pfVar4 + 0x10);
              FUN_100214c2(pcVar2);
              *(undefined4 *)(pfVar4 + 0x14) = 0;
            }
            std::_Lockit::~_Lockit((_Lockit *)&param_3);
            local_4 = CONCAT31(local_4._1_3_,3);
            std::_Locinfo::~_Locinfo(a_Stack_4c);
            if (*(int *)(pfVar4 + 0x14) != 0) {
              free(*(void **)(pfVar4 + 0x10));
              *(undefined4 *)(pfVar4 + 0x14) = 0;
            }
            *(undefined4 *)(pfVar4 + 0x10) = *(undefined4 *)_Cltab_exref;
            local_4 = CONCAT31(local_4._1_3_,2);
            std::_Lockit::~_Lockit((_Lockit *)&param_1);
            *(undefined ***)pfVar4 = &PTR_FUN_10025b7c;
          }
          local_4 = local_4 & 0xffffff00;
          std::_Lockit::_Lockit((_Lockit *)&param_1);
          local_4._0_1_ = 5;
          DAT_100384e4 = pfVar4;
          std::_Lockit::_Lockit((_Lockit *)&param_3);
          if (*(int *)(pfVar4 + 4) != -1) {
            *(int *)(pfVar4 + 4) = *(int *)(pfVar4 + 4) + 1;
          }
          std::_Lockit::~_Lockit((_Lockit *)&param_3);
          FUN_100214c2(FUN_1000a450);
          local_4 = (uint)local_4._1_3_ << 8;
          std::_Lockit::~_Lockit((_Lockit *)&param_1);
          DAT_100384e8 = pfVar4;
        }
        goto LAB_1000a38e;
      }
    }
    param_3 = s_missing_locale_facet_10035d2c;
    exception::exception((exception *)&_Stack_5c,&param_3);
    _Stack_5c._Page = (uint)_vftable__exref;
                    // WARNING: Subroutine does not return
    _CxxThrowException(&_Stack_5c,(ThrowInfo *)&DAT_10028070);
  }
LAB_1000a38e:
  local_4 = 0xffffffff;
  std::_Lockit::~_Lockit(local_60);
  ExceptionList = pvStack_c;
  return pfVar4;
}



void * __thiscall FUN_1000a3f0(void *this,byte param_1)

{
  std::ctype<char>::~ctype<char>((ctype<char> *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void __cdecl FUN_1000a410(undefined4 *param_1,undefined4 *param_2)

{
  int iVar1;
  
  if (param_1 != (undefined4 *)0x0) {
    for (iVar1 = 9; iVar1 != 0; iVar1 = iVar1 + -1) {
      *param_1 = *param_2;
      param_2 = param_2 + 1;
      param_1 = param_1 + 1;
    }
  }
  return;
}



void __cdecl FUN_1000a430(undefined4 *param_1,undefined4 *param_2)

{
  if (param_1 != (undefined4 *)0x0) {
    *param_1 = *param_2;
  }
  return;
}



void FUN_1000a450(void)

{
  int iVar1;
  uint uVar2;
  undefined4 *puVar3;
  _Lockit local_14 [4];
  _Lockit local_10 [4];
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_100221d9;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  std::_Lockit::_Lockit(local_10);
  uVar2 = DAT_100384e4;
  local_4 = 0;
  std::_Lockit::_Lockit(local_14);
  iVar1 = *(int *)(uVar2 + 4);
  if ((iVar1 != 0) && (iVar1 != -1)) {
    *(int *)(uVar2 + 4) = iVar1 + -1;
  }
  puVar3 = (undefined4 *)(uVar2 & (*(int *)(uVar2 + 4) != 0) - 1);
  std::_Lockit::~_Lockit(local_14);
  if (puVar3 != (undefined4 *)0x0) {
    (**(code **)*puVar3)(1);
  }
  DAT_100384e4 = 0;
  local_4 = 0xffffffff;
  std::_Lockit::~_Lockit(local_10);
  ExceptionList = local_c;
  return;
}



void FUN_1000a500(void)

{
                    // WARNING: Could not recover jumptable at 0x1000a505. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::Init((Init *)&DAT_100384ed);
  return;
}



void FUN_1000a510(void)

{
  FUN_100214c2(FUN_1000a520);
  return;
}



void FUN_1000a520(void)

{
                    // WARNING: Could not recover jumptable at 0x1000a525. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::~Init((Init *)&DAT_100384ed);
  return;
}



void FUN_1000a540(void)

{
                    // WARNING: Could not recover jumptable at 0x1000a545. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::_Winit((_Winit *)&DAT_100384ec);
  return;
}



void FUN_1000a550(void)

{
  FUN_100214c2(FUN_1000a560);
  return;
}



void FUN_1000a560(void)

{
                    // WARNING: Could not recover jumptable at 0x1000a565. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::~_Winit((_Winit *)&DAT_100384ec);
  return;
}



// int __fastcall Housemarque::Threedee_Engine::Get_Lod_Count(class
// Housemarque::Threedee_Engine::Mesh_Node * const)

int __fastcall Housemarque::Threedee_Engine::Get_Lod_Count(Mesh_Node *param_1)

{
                    // 0xa570  79  ?Get_Lod_Count@Threedee_Engine@Housemarque@@YIHQAVMesh_Node@12@@Z
  return *(int *)(param_1 + 0xc);
}



// class srMeshModel * __fastcall Housemarque::Threedee_Engine::Get_Mesh_Model(class
// Housemarque::Threedee_Engine::Mesh_Node * const,int)

srMeshModel * __fastcall
Housemarque::Threedee_Engine::Get_Mesh_Model(Mesh_Node *param_1,int param_2)

{
                    // 0xa580  81
                    // ?Get_Mesh_Model@Threedee_Engine@Housemarque@@YIPAVsrMeshModel@@QAVMesh_Node@12@H@Z
  return *(srMeshModel **)(param_1 + param_2 * 4 + 0xf8);
}



// public: __thiscall Housemarque::Threedee_Engine::Debug_Kludge::Debug_Kludge(char const *)

Debug_Kludge * __thiscall
Housemarque::Threedee_Engine::Debug_Kludge::Debug_Kludge(Debug_Kludge *this,char *param_1)

{
                    // 0xa590  7  ??0Debug_Kludge@Threedee_Engine@Housemarque@@QAE@PBD@Z
  return this;
}



// public: __thiscall Housemarque::Threedee_Engine::Debug_Kludge::~Debug_Kludge(void)

void __thiscall Housemarque::Threedee_Engine::Debug_Kludge::~Debug_Kludge(Debug_Kludge *this)

{
                    // 0xa5a0  30  ??1Debug_Kludge@Threedee_Engine@Housemarque@@QAE@XZ
  return;
}



// public: __thiscall Housemarque::Threedee_Engine::Engine::Engine(long,bool)

Engine * __thiscall
Housemarque::Threedee_Engine::Engine::Engine(Engine *this,long param_1,bool param_2)

{
  basic_string<> *this_00;
  char cVar1;
  bool bVar2;
  uint uVar3;
  uint uVar4;
  undefined4 *puVar5;
  char *pcVar6;
  undefined4 *puVar7;
  basic_string<> abStack_1c [4];
  undefined4 *puStack_18;
  uint uStack_14;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0xa5b0  9  ??0Engine@Threedee_Engine@Housemarque@@QAE@J_N@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_1002221b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(basic_string<> *)(this + 4) = param_1._0_1_;
  std::basic_string<>::_Tidy((basic_string<> *)(this + 4),false);
  abStack_1c[0] = param_1._0_1_;
  uStack_4 = 0;
  std::basic_string<>::_Tidy(abStack_1c,false);
  uVar3 = 0xffffffff;
  pcVar6 = &DAT_100384f0;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar1 = *pcVar6;
    pcVar6 = pcVar6 + 1;
  } while (cVar1 != '\0');
  uVar3 = ~uVar3 - 1;
  bVar2 = std::basic_string<>::_Grow(abStack_1c,uVar3,true);
  if (bVar2) {
    puVar5 = (undefined4 *)&DAT_100384f0;
    puVar7 = puStack_18;
    for (uVar4 = uVar3 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
      *puVar7 = *puVar5;
      puVar5 = puVar5 + 1;
      puVar7 = puVar7 + 1;
    }
    for (uVar4 = uVar3 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
      *(undefined *)puVar7 = *(undefined *)puVar5;
      puVar5 = (undefined4 *)((int)puVar5 + 1);
      puVar7 = (undefined4 *)((int)puVar7 + 1);
    }
    *(undefined *)((int)puStack_18 + uVar3) = 0;
    uStack_14 = uVar3;
  }
  this_00 = (basic_string<> *)(this + 0x18);
  *(undefined4 *)(this + 0x14) = 0;
  *this_00 = abStack_1c[0];
  uStack_4 = CONCAT31(uStack_4._1_3_,1);
  std::basic_string<>::_Tidy(this_00,false);
  std::basic_string<>::assign(this_00,abStack_1c,0,*(uint *)npos_exref);
  if (puStack_18 != (undefined4 *)0x0) {
    pcVar6 = (char *)((int)puStack_18 + -1);
    cVar1 = *pcVar6;
    if ((cVar1 == '\0') || (cVar1 == -1)) {
      FUN_100212a4(pcVar6);
    }
    else {
      *pcVar6 = cVar1 + -1;
    }
  }
  *(undefined4 *)(this + 0x28) = 0x280;
  *(undefined4 *)(this + 0x2c) = 0x1e0;
  *(undefined4 *)(this + 0x30) = 0x10;
  *(basic_string<> *)(this + 0x34) = param_1._0_1_;
  *(undefined4 *)(this + 0x38) = 0;
  *(undefined4 *)(this + 0x3c) = 0;
  *(undefined4 *)(this + 0x40) = 0;
  uStack_4 = CONCAT31(uStack_4._1_3_,4);
  *(undefined4 *)(this + 0x44) = 0;
  *(undefined4 *)(this + 0x4c) = 0;
  this[0x50] = (Engine)0x0;
  *(undefined ***)this = &PTR_FUN_10025b9c;
  *(long *)(this + 0x48) = param_1;
  srInit();
  srConfig::set((srConfig *)srConfig_exref,s_DD_GLIDE3X_10035d90,s_SINGLETMUMODE___1_10035d7c);
  srConfig::set((srConfig *)srConfig_exref,s_DD_DIRECTX6_10035de4,
                s_DISABLENONDISPLAYDEVICES_0_DISAB_10035d9c);
  srConfig::set((srConfig *)srConfig_exref,s_DD_DIRECTX7_10035e38,
                s_DISABLENONDISPLAYDEVICES_0_DISAB_10035df0);
  *(undefined4 *)(srCore_exref + 0x160) = 1;
  srExtension::load(s_JPEGImporter_10035e44,(char *)0x0);
  ExceptionList = pvStack_c;
  return this;
}



// public: virtual __thiscall Housemarque::Threedee_Engine::Engine::~Engine(void)

void __thiscall Housemarque::Threedee_Engine::Engine::~Engine(Engine *this)

{
  char *pcVar1;
  char cVar2;
  basic_string<> *pbVar3;
  basic_string<> *this_00;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0xa760  32  ??1Engine@Threedee_Engine@Housemarque@@UAE@XZ
  puStack_8 = &LAB_10022247;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined ***)this = &PTR_FUN_10025b9c;
  local_4 = 2;
  if (*(undefined4 **)(this + 0x44) != (undefined4 *)0x0) {
    (**(code **)**(undefined4 **)(this + 0x44))(1);
  }
  srExit();
  pbVar3 = *(basic_string<> **)(this + 0x3c);
  this_00 = *(basic_string<> **)(this + 0x38);
  local_4 = CONCAT31(local_4._1_3_,1);
  for (; this_00 != pbVar3; this_00 = this_00 + 0x10) {
    std::basic_string<>::_Tidy(this_00,true);
  }
  FUN_100212a4(*(void **)(this + 0x38));
  *(undefined4 *)(this + 0x38) = 0;
  *(undefined4 *)(this + 0x3c) = 0;
  *(undefined4 *)(this + 0x40) = 0;
  if (*(int *)(this + 0x1c) != 0) {
    pcVar1 = (char *)(*(int *)(this + 0x1c) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(this + 0x1c) = 0;
  *(undefined4 *)(this + 0x20) = 0;
  *(undefined4 *)(this + 0x24) = 0;
  if (*(int *)(this + 8) != 0) {
    pcVar1 = (char *)(*(int *)(this + 8) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(this + 8) = 0;
  *(undefined4 *)(this + 0xc) = 0;
  *(undefined4 *)(this + 0x10) = 0;
  ExceptionList = pvStack_c;
  return;
}



// public: bool __fastcall Housemarque::Threedee_Engine::Engine::Send_Message_To_SR(void *)

bool __fastcall Housemarque::Threedee_Engine::Engine::Send_Message_To_SR(void *param_1)

{
  int in_EDX;
  undefined4 local_14;
  undefined4 local_10;
  undefined4 local_c;
  undefined4 local_8;
  int local_4;
  
                    // 0xa850  126
                    // ?Send_Message_To_SR@Engine@Threedee_Engine@Housemarque@@QAI_NPAX@Z
  local_c = *(undefined4 *)(in_EDX + 8);
  local_10 = *(undefined4 *)(in_EDX + 4);
  local_14 = *(undefined4 *)((int)param_1 + 0x48);
  local_8 = *(undefined4 *)(in_EDX + 0xc);
  local_4 = 0;
  if (*(srGERD **)((int)param_1 + 0x44) != (srGERD *)0x0) {
    srGERD::extCommand(*(srGERD **)((int)param_1 + 0x44),0xf01,&local_14,0x14);
    if (local_4 != 0) {
      if (*(int *)(in_EDX + 4) == 0x1c) {
        if (*(int *)(in_EDX + 8) == 0) {
          ShowWindow(*(HWND *)((int)param_1 + 0x48),6);
        }
        else {
          ShowWindow(*(HWND *)((int)param_1 + 0x48),1);
          SetFocus(*(HWND *)((int)param_1 + 0x48));
          SetActiveWindow(*(HWND *)((int)param_1 + 0x48));
          SetWindowPos(*(HWND *)((int)param_1 + 0x48),(HWND)0xffffffff,0,0,0,0,0x43);
        }
      }
      if (local_4 != 0) {
        return true;
      }
    }
  }
  return false;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Available_Render_APIs(class
// std::vector<class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char>
// >,class std::allocator<class std::basic_string<char,struct std::char_traits<char>,class
// std::allocator<char> > > > &)

void __fastcall Housemarque::Threedee_Engine::Engine::Available_Render_APIs(vector<> *param_1)

{
  char cVar1;
  bool bVar2;
  uint uVar3;
  char *pcVar4;
  void *_Memory;
  code *pcVar5;
  int iVar6;
  uint uVar7;
  uint uVar8;
  void *in_EDX;
  char *pcVar9;
  uint uStack_38;
  srStringTable local_28 [12];
  basic_string<> abStack_1c [4];
  char *pcStack_18;
  uint uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  int iStack_4;
  
                    // 0xa910  54
                    // ?Available_Render_APIs@Engine@Threedee_Engine@Housemarque@@QAIXAAV?$vector@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@V?$allocator@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@@std@@@Z
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_10022263;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  srStringTable::srStringTable(local_28);
  iStack_4 = 0;
  uVar3 = srSystem::scanLibraries(local_28,(char *)0x0,s_srDD__10035e54);
  uStack_38 = 0;
  if (uVar3 != 0) {
    do {
      pcVar4 = srStringTable::getString(local_28,uStack_38);
      _Memory = srDynamicLibrary::load(pcVar4);
      if (_Memory != (void *)0x0) {
        pcVar5 = (code *)srDynamicLibrary::getFunction(_Memory,s_srDDGetDeviceCount_10035e5c);
        if (((pcVar5 != (code *)0x0) && (iVar6 = (*pcVar5)(), iVar6 != 0)) &&
           (pcVar5 = (code *)srDynamicLibrary::getFunction(_Memory,s_srDDGetDriverName_10035e70),
           pcVar5 != (code *)0x0)) {
          pcVar4 = (char *)(*pcVar5)();
          pcStack_18 = (char *)0x0;
          uVar7 = 0xffffffff;
          pcVar9 = pcVar4;
          do {
            if (uVar7 == 0) break;
            uVar7 = uVar7 - 1;
            cVar1 = *pcVar9;
            pcVar9 = pcVar9 + 1;
          } while (cVar1 != '\0');
          uVar7 = ~uVar7 - 1;
          uStack_14 = 0;
          uStack_10 = 0;
          bVar2 = std::basic_string<>::_Grow(abStack_1c,uVar7,true);
          if (bVar2) {
            pcVar9 = pcStack_18;
            for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
              *(undefined4 *)pcVar9 = *(undefined4 *)pcVar4;
              pcVar4 = pcVar4 + 4;
              pcVar9 = pcVar9 + 4;
            }
            for (uVar8 = uVar7 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
              *pcVar9 = *pcVar4;
              pcVar4 = pcVar4 + 1;
              pcVar9 = pcVar9 + 1;
            }
            pcStack_18[uVar7] = '\0';
            uStack_14 = uVar7;
          }
          iStack_4._0_1_ = 1;
          FUN_10015800(in_EDX,*(basic_string<> **)((int)in_EDX + 8),(basic_string<> *)0x1,abStack_1c
                      );
          iStack_4 = (uint)iStack_4._1_3_ << 8;
          if (pcStack_18 != (char *)0x0) {
            pcVar4 = pcStack_18 + -1;
            cVar1 = *pcVar4;
            if ((cVar1 == '\0') || (cVar1 == -1)) {
              FUN_100212a4(pcVar4);
            }
            else {
              *pcVar4 = cVar1 + -1;
            }
          }
          pcStack_18 = (char *)0x0;
          uStack_14 = 0;
          uStack_10 = 0;
        }
        srDynamicLibrary::free(_Memory);
      }
      uStack_38 = uStack_38 + 1;
    } while (uStack_38 < uVar3);
  }
  iStack_4 = 0xffffffff;
  srStringTable::~srStringTable(local_28);
  ExceptionList = pvStack_c;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Select_Render_API(class
// std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &)

void __fastcall Housemarque::Threedee_Engine::Engine::Select_Render_API(basic_string<> *param_1)

{
  basic_string<> *this;
  char *pcVar1;
  char cVar2;
  int iVar3;
  bool bVar4;
  uint uVar5;
  code *pcVar6;
  uint uVar7;
  basic_string<> *in_EDX;
  uint uVar8;
  code *pcVar9;
  
                    // 0xaab0  125
                    // ?Select_Render_API@Engine@Threedee_Engine@Housemarque@@QAIXABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@Z
  uVar7 = *(uint *)npos_exref;
  uVar5 = *(uint *)(in_EDX + 8);
  this = param_1 + 4;
  uVar8 = uVar5;
  if (uVar7 < uVar5) {
    uVar8 = uVar7;
  }
  if (this == in_EDX) {
    if (*(uint *)(param_1 + 0xc) < uVar8) {
      std::_Xran();
    }
    std::basic_string<>::_Split(this);
    uVar5 = *(int *)(param_1 + 0xc) - uVar8;
    if (uVar5 < uVar7) {
      uVar7 = uVar5;
    }
    if (uVar7 != 0) {
      memmove((void *)(uVar8 + *(int *)(param_1 + 8)),
              (void *)((int)(uVar8 + *(int *)(param_1 + 8)) + uVar7),uVar5 - uVar7);
      iVar3 = *(int *)(param_1 + 0xc);
      bVar4 = std::basic_string<>::_Grow(this,iVar3 - uVar7,false);
      if (bVar4) {
        std::basic_string<>::_Eos(this,iVar3 - uVar7);
      }
    }
    std::basic_string<>::_Split(this);
    return;
  }
  if ((uVar8 != 0) && (uVar8 == uVar5)) {
    pcVar6 = *(code **)(in_EDX + 4);
    if (*(code **)(in_EDX + 4) == (code *)0x0) {
      pcVar6 = _C_exref;
    }
    if ((byte)pcVar6[-1] < 0xfe) {
      if (*(int *)(param_1 + 8) != 0) {
        pcVar1 = (char *)(*(int *)(param_1 + 8) + -1);
        cVar2 = *pcVar1;
        if ((cVar2 == '\0') || (cVar2 == -1)) {
          FUN_100212a4(pcVar1);
        }
        else {
          *pcVar1 = cVar2 + -1;
        }
      }
      *(undefined4 *)(param_1 + 8) = 0;
      *(undefined4 *)(param_1 + 0xc) = 0;
      *(undefined4 *)(param_1 + 0x10) = 0;
      pcVar6 = *(code **)(in_EDX + 4);
      if (*(code **)(in_EDX + 4) == (code *)0x0) {
        pcVar6 = _C_exref;
      }
      *(code **)(param_1 + 8) = pcVar6;
      *(undefined4 *)(param_1 + 0xc) = *(undefined4 *)(in_EDX + 8);
      *(undefined4 *)(param_1 + 0x10) = *(undefined4 *)(in_EDX + 0xc);
      pcVar6[-1] = (code)((char)pcVar6[-1] + '\x01');
      return;
    }
  }
  bVar4 = std::basic_string<>::_Grow(this,uVar8,true);
  if (bVar4) {
    pcVar6 = _C_exref;
    if (*(code **)(in_EDX + 4) != (code *)0x0) {
      pcVar6 = *(code **)(in_EDX + 4);
    }
    pcVar9 = *(code **)(param_1 + 8);
    for (uVar7 = uVar8 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
      *(undefined4 *)pcVar9 = *(undefined4 *)pcVar6;
      pcVar6 = pcVar6 + 4;
      pcVar9 = pcVar9 + 4;
    }
    for (uVar7 = uVar8 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
      *pcVar9 = *pcVar6;
      pcVar6 = pcVar6 + 1;
      pcVar9 = pcVar9 + 1;
    }
    *(uint *)(param_1 + 0xc) = uVar8;
    *(undefined *)(*(int *)(param_1 + 8) + uVar8) = 0;
  }
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Available_Cards(class
// std::vector<struct Housemarque::Threedee_Engine::Engine::Device,class std::allocator<struct
// Housemarque::Threedee_Engine::Engine::Device> > &)

void __fastcall Housemarque::Threedee_Engine::Engine::Available_Cards(vector<> *param_1)

{
  char cVar1;
  bool bVar2;
  uint uVar3;
  char *pcVar4;
  void *_Memory;
  code *pcVar5;
  code *pcVar6;
  uint uVar7;
  char *pcVar8;
  uint uVar9;
  void *in_EDX;
  uint uVar10;
  basic_string<> bStack_55;
  uint uStack_54;
  srStringTable local_3c [12];
  basic_string<> abStack_30 [4];
  int iStack_2c;
  undefined4 uStack_28;
  undefined4 uStack_24;
  uint uStack_20;
  basic_string<> abStack_1c [4];
  int iStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  uint uStack_4;
  
                    // 0xabf0  53
                    // ?Available_Cards@Engine@Threedee_Engine@Housemarque@@QAIXAAV?$vector@UDevice@Engine@Threedee_Engine@Housemarque@@V?$allocator@UDevice@Engine@Threedee_Engine@Housemarque@@@std@@@std@@@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10022287;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  srStringTable::srStringTable(local_3c);
  uStack_4 = 0;
  uVar3 = srSystem::scanLibraries(local_3c,(char *)0x0,s_srDD__10035e84);
  uStack_54 = 0;
  if (uVar3 != 0) {
    do {
      pcVar4 = srStringTable::getString(local_3c,uStack_54);
      _Memory = srDynamicLibrary::load(pcVar4);
      if (_Memory != (void *)0x0) {
        pcVar5 = (code *)srDynamicLibrary::getFunction(_Memory,s_srDDConfigureDriver_10035e8c);
        if (pcVar5 != (code *)0x0) {
          (*pcVar5)(s_DISABLENONDISPLAYDEVICES_0_DISAB_10035ea0);
        }
        pcVar5 = (code *)srDynamicLibrary::getFunction(_Memory,s_srDDGetDriverName_10035ee8);
        pcVar4 = (char *)(*pcVar5)();
        bVar2 = std::operator==(pcVar4,(basic_string<> *)(param_1 + 4));
        if (bVar2) {
          pcVar5 = (code *)srDynamicLibrary::getFunction(_Memory,s_srDDGetDeviceName_10035efc);
          pcVar6 = (code *)srDynamicLibrary::getFunction(_Memory,s_srDDGetDeviceCount_10035f10);
          uVar7 = (*pcVar6)();
          uVar10 = 0;
          if (uVar7 != 0) {
            do {
              pcVar8 = (char *)(*pcVar5)(uVar10);
              uVar9 = 0xffffffff;
              abStack_30[0] = bStack_55;
              pcVar4 = pcVar8;
              do {
                if (uVar9 == 0) break;
                uVar9 = uVar9 - 1;
                cVar1 = *pcVar4;
                pcVar4 = pcVar4 + 1;
              } while (cVar1 != '\0');
              iStack_2c = 0;
              uStack_28 = 0;
              uStack_24 = 0;
              std::basic_string<>::assign(abStack_30,pcVar8,~uVar9 - 1);
              abStack_1c[0] = abStack_30[0];
              uStack_4._0_1_ = 1;
              uStack_20 = uVar10;
              std::basic_string<>::_Tidy(abStack_1c,false);
              std::basic_string<>::assign(abStack_1c,abStack_30,0,*(uint *)npos_exref);
              uStack_4 = CONCAT31(uStack_4._1_3_,2);
              FUN_10015b10(in_EDX,*(undefined4 **)((int)in_EDX + 8),(undefined4 *)0x1,&uStack_20);
              if (iStack_18 != 0) {
                pcVar4 = (char *)(iStack_18 + -1);
                cVar1 = *pcVar4;
                if ((cVar1 == '\0') || (cVar1 == -1)) {
                  FUN_100212a4(pcVar4);
                }
                else {
                  *pcVar4 = cVar1 + -1;
                }
              }
              iStack_18 = 0;
              uStack_14 = 0;
              uStack_10 = 0;
              uStack_4 = uStack_4 & 0xffffff00;
              if (iStack_2c != 0) {
                pcVar4 = (char *)(iStack_2c + -1);
                cVar1 = *pcVar4;
                if ((cVar1 == '\0') || (cVar1 == -1)) {
                  FUN_100212a4(pcVar4);
                }
                else {
                  *pcVar4 = cVar1 + -1;
                }
              }
              uVar10 = uVar10 + 1;
              iStack_2c = 0;
              uStack_28 = 0;
              uStack_24 = 0;
            } while (uVar10 < uVar7);
          }
        }
        srDynamicLibrary::free(_Memory);
      }
      uStack_54 = uStack_54 + 1;
    } while (uStack_54 < uVar3);
  }
  uStack_4 = 0xffffffff;
  srStringTable::~srStringTable(local_3c);
  ExceptionList = pvStack_c;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Select_Card(struct
// Housemarque::Threedee_Engine::Engine::Device)

void __fastcall
Housemarque::Threedee_Engine::Engine::Select_Card
          (int param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,undefined *param_5,
          uint param_6,undefined4 param_7)

{
  basic_string<> *this;
  char *pcVar1;
  char cVar2;
  uint uVar3;
  bool bVar4;
  code *pcVar5;
  uint uVar6;
  uint uVar7;
  code *pcVar8;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  uVar3 = param_6;
                    // 0xae20  124
                    // ?Select_Card@Engine@Threedee_Engine@Housemarque@@QAIXUDevice@123@@Z
  pcVar8 = (code *)param_5;
  puStack_8 = &LAB_10022299;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined4 *)(param_1 + 0x14) = param_3;
  this = (basic_string<> *)(param_1 + 0x18);
  uVar6 = *(uint *)npos_exref;
  local_4 = 0;
  uVar7 = param_6;
  if (uVar6 < param_6) {
    uVar7 = uVar6;
  }
  if (this == (basic_string<> *)&param_4) {
    std::basic_string<>::erase(this,uVar7,uVar6);
    std::basic_string<>::erase(this,0,0);
    pcVar8 = (code *)param_5;
  }
  else {
    if ((uVar7 != 0) && (uVar7 == param_6)) {
      pcVar5 = _C_exref;
      if (param_5 != (undefined *)0x0) {
        pcVar5 = (code *)param_5;
      }
      if ((byte)pcVar5[-1] < 0xfe) {
        if (*(int *)(param_1 + 0x1c) != 0) {
          pcVar1 = (char *)(*(int *)(param_1 + 0x1c) + -1);
          cVar2 = *pcVar1;
          if ((cVar2 == '\0') || (cVar2 == -1)) {
            FUN_100212a4(pcVar1);
          }
          else {
            *pcVar1 = cVar2 + -1;
          }
        }
        *(undefined4 *)(param_1 + 0x1c) = 0;
        *(undefined4 *)(param_1 + 0x20) = 0;
        *(undefined4 *)(param_1 + 0x24) = 0;
        pcVar5 = _C_exref;
        if (pcVar8 != (code *)0x0) {
          pcVar5 = pcVar8;
        }
        *(undefined4 *)(param_1 + 0x24) = param_7;
        *(code **)(param_1 + 0x1c) = pcVar5;
        *(uint *)(param_1 + 0x20) = uVar3;
        pcVar5[-1] = (code)((char)pcVar5[-1] + '\x01');
        goto LAB_1000af27;
      }
    }
    bVar4 = std::basic_string<>::_Grow(this,uVar7,true);
    if (bVar4) {
      if (pcVar8 == (code *)0x0) {
        pcVar8 = _C_exref;
      }
      pcVar5 = *(code **)(param_1 + 0x1c);
      for (uVar6 = uVar7 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
        *(undefined4 *)pcVar5 = *(undefined4 *)pcVar8;
        pcVar8 = pcVar8 + 4;
        pcVar5 = pcVar5 + 4;
      }
      for (uVar6 = uVar7 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
        *pcVar5 = *pcVar8;
        pcVar8 = pcVar8 + 1;
        pcVar5 = pcVar5 + 1;
      }
      std::basic_string<>::_Eos(this,uVar7);
      pcVar8 = (code *)param_5;
    }
  }
LAB_1000af27:
  if (pcVar8 != (code *)0x0) {
    cVar2 = (char)pcVar8[-1];
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar8 + -1);
    }
    else {
      pcVar8[-1] = (code)(cVar2 + -1);
    }
  }
  ExceptionList = pvStack_c;
  return;
}



// WARNING: Removing unreachable block (ram,0x1000b193)
// WARNING: Removing unreachable block (ram,0x1000b1ae)
// WARNING: Removing unreachable block (ram,0x1000b1cc)
// WARNING: Removing unreachable block (ram,0x1000b1d0)
// WARNING: Removing unreachable block (ram,0x1000b1d7)
// public: void __fastcall Housemarque::Threedee_Engine::Engine::Available_Resolutions(class
// std::vector<struct Housemarque::Threedee_Engine::Engine::Resolution,class std::allocator<struct
// Housemarque::Threedee_Engine::Engine::Resolution> > &)

void __fastcall Housemarque::Threedee_Engine::Engine::Available_Resolutions(vector<> *param_1)

{
  bool bVar1;
  srGERD *this;
  ulong uVar2;
  undefined4 *puVar3;
  void *in_EDX;
  int iVar4;
  int iVar5;
  code *pcVar6;
  undefined4 *puVar7;
  uint uVar8;
  undefined4 *puVar9;
  undefined4 *puVar10;
  uint uStack_28;
  undefined4 uStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  undefined4 uStack_c;
  undefined4 uStack_8;
  undefined4 uStack_4;
  
                    // 0xaf60  56
                    // ?Available_Resolutions@Engine@Threedee_Engine@Housemarque@@QAIXAAV?$vector@UResolution@Engine@Threedee_Engine@Housemarque@@V?$allocator@UResolution@Engine@Threedee_Engine@Housemarque@@@std@@@std@@@Z
  bVar1 = std::operator==((basic_string<> *)(param_1 + 4),s_OpenGL_10035f24);
  if (bVar1) {
    GetAvailableOpenGLResolutions(in_EDX);
    return;
  }
  pcVar6 = *(code **)(param_1 + 8);
  if (*(code **)(param_1 + 8) == (code *)0x0) {
    pcVar6 = _C_exref;
  }
  this = srGERD::loadDevice((char *)pcVar6,(char *)0x0,*(ulong *)(param_1 + 0x14));
  if (this == (srGERD *)0x0) {
    GetAvailableOpenGLResolutions(in_EDX);
  }
  srGERD::createContext(this,*(ulong *)(param_1 + 0x48));
  uVar2 = srGERD::getDisplayModeCount(this);
  uStack_28 = 0;
  if (uVar2 != 0) {
    do {
      srGERD::getDisplayModeInfo(this,uStack_28,(DisplayModeInfo *)&uStack_c);
      uStack_10 = uStack_4;
      uStack_18 = uStack_c;
      uStack_14 = uStack_8;
      puVar10 = *(undefined4 **)((int)in_EDX + 8);
      iVar4 = *(int *)((int)in_EDX + 0xc) - (int)puVar10;
      iVar5 = iVar4 >> 0x1f;
      if (iVar4 / 0xc + iVar5 == iVar5) {
        iVar5 = *(int *)((int)in_EDX + 4);
        if ((iVar5 == 0) || (uVar8 = ((int)puVar10 - iVar5) / 0xc, uVar8 < 2)) {
          uVar8 = 1;
        }
        if (iVar5 == 0) {
          iVar5 = 0;
        }
        else {
          iVar5 = ((int)puVar10 - iVar5) / 0xc;
        }
        iVar5 = iVar5 + uVar8;
        iVar4 = iVar5;
        if (iVar5 < 0) {
          iVar4 = 0;
        }
        puVar3 = (undefined4 *)operator_new(iVar4 * 0xc);
        puVar7 = puVar3;
        for (puVar9 = *(undefined4 **)((int)in_EDX + 4); puVar9 != puVar10; puVar9 = puVar9 + 3) {
          FUN_10016880(puVar7,puVar9);
          puVar7 = puVar7 + 3;
        }
        FUN_10016280(puVar7,1,&uStack_18);
        FUN_10016240(puVar10,*(undefined4 **)((int)in_EDX + 8),puVar7 + 3);
        FUN_10016230();
        FUN_100212a4(*(void **)((int)in_EDX + 4));
        *(undefined4 **)((int)in_EDX + 0xc) = puVar3 + iVar5 * 3;
        iVar5 = FUN_10016210((int)in_EDX);
        *(undefined4 **)((int)in_EDX + 8) = puVar3 + (iVar5 + 1) * 3;
        *(undefined4 **)((int)in_EDX + 4) = puVar3;
      }
      else {
        FUN_10016240(puVar10,puVar10,puVar10 + 3);
        FUN_10016280(*(undefined4 **)((int)in_EDX + 8),
                     1 - ((int)*(undefined4 **)((int)in_EDX + 8) - (int)puVar10) / 0xc,&uStack_18);
        puVar9 = *(undefined4 **)((int)in_EDX + 8);
        for (; puVar10 != puVar9; puVar10 = puVar10 + 3) {
          *puVar10 = uStack_18;
          puVar10[1] = uStack_14;
          puVar10[2] = uStack_10;
        }
        *(int *)((int)in_EDX + 8) = *(int *)((int)in_EDX + 8) + 0xc;
      }
      uStack_28 = uStack_28 + 1;
    } while (uStack_28 < uVar2);
  }
  srGERD::deleteContext(this);
  if (this != (srGERD *)0x0) {
    (*(code *)**(undefined4 **)this)(1);
  }
  return;
}



void __cdecl GetAvailableOpenGLResolutions(void *param_1)

{
  undefined4 local_c;
  undefined4 local_8;
  undefined4 local_4;
  
  local_c = 0x280;
  local_8 = 0x1e0;
  local_4 = 0x10;
  AddResolutionPairToGui16Bit(param_1,*(undefined4 **)((int)param_1 + 8),&local_c);
  local_c = 800;
  local_8 = 600;
  local_4 = 0x10;
  AddResolutionPairToGui16Bit(param_1,*(undefined4 **)((int)param_1 + 8),&local_c);
  local_c = 0x400;
  local_8 = 0x300;
  local_4 = 0x10;
  AddResolutionPairToGui16Bit(param_1,*(undefined4 **)((int)param_1 + 8),&local_c);
  local_c = 0x500;
  local_8 = 0x400;
  local_4 = 0x10;
  AddResolutionPairToGui16Bit(param_1,*(undefined4 **)((int)param_1 + 8),&local_c);
  local_c = 0x640;
  local_8 = 0x4b0;
  local_4 = 0x10;
  AddResolutionPairToGui32Bit(param_1,*(undefined4 **)((int)param_1 + 8),1,&local_c);
  local_c = 0x280;
  local_8 = 0x1e0;
  local_4 = 0x20;
  AddResolutionPairToGui32Bit(param_1,*(undefined4 **)((int)param_1 + 8),1,&local_c);
  local_c = 800;
  local_8 = 600;
  local_4 = 0x20;
  AddResolutionPairToGui32Bit(param_1,*(undefined4 **)((int)param_1 + 8),1,&local_c);
  local_c = 0x400;
  local_8 = 0x300;
  local_4 = 0x20;
  AddResolutionPairToGui32Bit(param_1,*(undefined4 **)((int)param_1 + 8),1,&local_c);
  local_c = 0x500;
  local_8 = 0x400;
  local_4 = 0x20;
  AddResolutionPairToGui32Bit(param_1,*(undefined4 **)((int)param_1 + 8),1,&local_c);
  local_c = 0x640;
  local_8 = 0x4b0;
  local_4 = 0x20;
  AddResolutionPairToGui32Bit(param_1,*(undefined4 **)((int)param_1 + 8),1,&local_c);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Set_Resolution(struct
// Housemarque::Threedee_Engine::Engine::Resolution)

void __fastcall
Housemarque::Threedee_Engine::Engine::Set_Resolution
          (int param_1,undefined4 param_2,undefined4 param_3,undefined4 param_4,undefined4 param_5)

{
                    // 0xb3c0  144
                    // ?Set_Resolution@Engine@Threedee_Engine@Housemarque@@QAIXUResolution@123@@Z
  *(undefined4 *)(param_1 + 0x28) = param_3;
  *(undefined4 *)(param_1 + 0x2c) = param_4;
  *(undefined4 *)(param_1 + 0x30) = param_5;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Open_Display(class
// Housemarque::Threedee_Engine::Engine::Mode &,bool)

void __fastcall Housemarque::Threedee_Engine::Engine::Open_Display(Mode *param_1,bool param_2)

{
  bool bVar1;
  code *pcVar2;
  srGERD *psVar3;
  int iVar4;
  basic_ostream<> *pbVar5;
  char *pcVar6;
  char *pcVar7;
  Error_Log_Inserter aEStack_20 [8];
  undefined4 uStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0xb3e0  112
                    // ?Open_Display@Engine@Threedee_Engine@Housemarque@@QAIXAAVMode@123@_N@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_100222bb;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (*(srGERD **)(param_1 + 0x44) != (srGERD *)0x0) {
    ExceptionList = &pvStack_c;
    srGERD::deleteContext(*(srGERD **)(param_1 + 0x44));
    srGERD::closeWindow(*(srGERD **)(param_1 + 0x44),1);
  }
  *(undefined4 *)(param_1 + 0x44) = 0;
  pcVar2 = *(code **)(param_1 + 8);
  if (*(code **)(param_1 + 8) == (code *)0x0) {
    pcVar2 = _C_exref;
  }
  psVar3 = srGERD::loadDevice((char *)pcVar2,(char *)0x0,*(ulong *)(param_1 + 0x14));
  *(srGERD **)(param_1 + 0x44) = psVar3;
  if (psVar3 == (srGERD *)0x0) {
    psVar3 = srGERD::getFirst();
    *(srGERD **)(param_1 + 0x44) = psVar3;
    bVar1 = Housemarque::Kernel::Logging_Enabled();
    if (bVar1) {
      iVar4 = Housemarque::Kernel::Error_Log(aEStack_20);
      uStack_4 = 0;
      pbVar5 = std::operator<<(*(basic_ostream<> **)(iVar4 + 4),s_Cant_find_render_device__10035f44)
      ;
      pcVar6 = srRuntimeClass::getName(*(srRuntimeClass **)(param_1 + 0x44));
      pcVar7 = s_Using_best_available__10035f2c;
      pbVar5 = std::basic_ostream<>::operator<<(pbVar5,*(int *)(param_1 + 0x14));
      pbVar5 = std::operator<<(pbVar5,pcVar7);
      std::operator<<(pbVar5,pcVar6);
    }
    uStack_4 = 0xffffffff;
    if (bVar1) {
      Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter(aEStack_20);
    }
    uStack_18 = 0;
    uStack_14 = 0;
    uStack_10 = 0;
    param_2 = SUB41(&uStack_18,0);
  }
  Open_Window(param_1,param_2);
  ExceptionList = pvStack_c;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Set_Fog_Range(float,float)

void __fastcall Housemarque::Threedee_Engine::Engine::Set_Fog_Range(float param_1,float param_2)

{
  undefined4 local_10 [3];
  undefined4 local_4;
  
                    // 0xb500  135  ?Set_Fog_Range@Engine@Threedee_Engine@Housemarque@@QAIXMM@Z
  local_10[0] = 1;
  local_4 = 0x3f800000;
  srGERD::extCommand(*(srGERD **)((int)param_1 + 0x44),0xf00,local_10,0x10);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Set_Fog_Color(class
// Housemarque::Template_Library::Vector_3<float> const &)

void __fastcall Housemarque::Threedee_Engine::Engine::Set_Fog_Color(Vector_3<float> *param_1)

{
  undefined4 *in_EDX;
  undefined4 local_c;
  undefined4 local_8;
  undefined4 local_4;
  
                    // 0xb540  134
                    // ?Set_Fog_Color@Engine@Threedee_Engine@Housemarque@@QAIXABV?$Vector_3@M@Template_Library@3@@Z
  local_c = *in_EDX;
  local_8 = in_EDX[1];
  local_4 = in_EDX[2];
  srGERD::setFogColor(*(srGERD **)(param_1 + 0x44),(srVector3T<float> *)&local_c);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Textures_To_16Bit(bool)

void __fastcall Housemarque::Threedee_Engine::Engine::Textures_To_16Bit(bool param_1)

{
  undefined3 in_register_00000005;
  undefined in_DL;
  
                    // 0xb570  151  ?Textures_To_16Bit@Engine@Threedee_Engine@Housemarque@@QAIX_N@Z
  *(undefined *)(CONCAT31(in_register_00000005,param_1) + 0x50) = in_DL;
  return;
}



// public: class Housemarque::Threedee_Engine::Engine::Buffer * __fastcall
// Housemarque::Threedee_Engine::Engine::Lock_Buffer(void)

Buffer * __fastcall Housemarque::Threedee_Engine::Engine::Lock_Buffer(void)

{
  Buffer *pBVar1;
  int in_ECX;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0xb580  111
                    // ?Lock_Buffer@Engine@Threedee_Engine@Housemarque@@QAIPAVBuffer@123@XZ
  local_4 = 0xffffffff;
  puStack_8 = &LAB_100222d0;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  pBVar1 = (Buffer *)operator_new(4);
  local_4 = 0;
  if (pBVar1 != (Buffer *)0x0) {
    pBVar1 = (Buffer *)Buffer::Buffer(pBVar1,*(srGERD **)(in_ECX + 0x44));
    *(Buffer **)(in_ECX + 0x4c) = pBVar1;
    ExceptionList = local_c;
    return pBVar1;
  }
  *(undefined4 *)(in_ECX + 0x4c) = 0;
  ExceptionList = local_c;
  return (Buffer *)0x0;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Release_Buffer(void)

void __fastcall Housemarque::Threedee_Engine::Engine::Release_Buffer(void)

{
  Buffer *this;
  int in_ECX;
  
                    // 0xb5f0  119  ?Release_Buffer@Engine@Threedee_Engine@Housemarque@@QAIXXZ
  this = *(Buffer **)(in_ECX + 0x4c);
  if (this != (Buffer *)0x0) {
    Buffer::~Buffer(this);
    FUN_100212a4(this);
  }
  *(undefined4 *)(in_ECX + 0x4c) = 0;
  return;
}



// private: void __fastcall Housemarque::Threedee_Engine::Engine::Open_Window(class
// Housemarque::Threedee_Engine::Engine::Mode &,bool)

void __fastcall Housemarque::Threedee_Engine::Engine::Open_Window(Mode *param_1,bool param_2)

{
  bool bVar1;
  e_error eVar2;
  ulong uVar3;
  int iVar4;
  basic_ostream<> *pbVar5;
  char *pcVar6;
  undefined3 in_register_00000009;
  uint *puVar7;
  uint uVar8;
  char in_stack_00000004;
  char *pcVar9;
  char *pcVar10;
  uint uVar11;
  char *pcVar12;
  Error_Log_Inserter aEStack_20 [8];
  uint uStack_18;
  uint uStack_14;
  uint uStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  puVar7 = (uint *)CONCAT31(in_register_00000009,param_2);
                    // 0xb620  113
                    // ?Open_Window@Engine@Threedee_Engine@Housemarque@@AAIXAAVMode@123@_N@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_100222f2;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  eVar2 = srGERD::createContext(*(srGERD **)(param_1 + 0x44),*(ulong *)(param_1 + 0x48));
  if (eVar2 != 0) {
    Housemarque::Kernel::Throw_Error
              ((char *)0x0,s__sr_gerd_>createContext_win_hand_10035f78,
               s_E__work_ht_3de_3de_cpp_10035f60,0x232);
  }
  srGERD::setSwapInterval(*(srGERD **)(param_1 + 0x44),0);
  if (((*puVar7 == 0) && (puVar7[1] == 0)) && (puVar7[2] == 0)) {
    eVar2 = srGERD::openWindow(*(srGERD **)(param_1 + 0x44),0);
    if (eVar2 != 0) {
      Housemarque::Kernel::Throw_Error
                ((char *)0x0,s__sr_gerd_>openWindow_0__10035fb4,s_E__work_ht_3de_3de_cpp_10035f9c,
                 0x23c);
    }
  }
  else {
    if (in_stack_00000004 == '\0') {
      srGERD::openWindow(*(srGERD **)(param_1 + 0x44));
    }
    else {
      uVar3 = srGERD::getDisplayModeCount(*(srGERD **)(param_1 + 0x44));
      uVar8 = 0;
      if (uVar3 != 0) {
        do {
          srGERD::getDisplayModeInfo
                    (*(srGERD **)(param_1 + 0x44),uVar8,(DisplayModeInfo *)&uStack_18);
          if (((*puVar7 == uStack_18) && (puVar7[1] == uStack_14)) && (puVar7[2] == uStack_10)) {
            srGERD::openWindow(*(srGERD **)(param_1 + 0x44),uVar8);
            break;
          }
          uVar8 = uVar8 + 1;
        } while (uVar8 < uVar3);
      }
    }
    iVar4 = srGERD::isWindowOpen(*(srGERD **)(param_1 + 0x44));
    if (iVar4 == 0) {
      bVar1 = Housemarque::Kernel::Logging_Enabled();
      if (bVar1) {
        iVar4 = Housemarque::Kernel::Error_Log(aEStack_20);
        uStack_4 = 0;
        pbVar5 = std::operator<<(*(basic_ostream<> **)(iVar4 + 4),
                                 s_Cant_open_required_display_mode__10035fdc);
        pcVar6 = srRuntimeClass::getName(*(srRuntimeClass **)(param_1 + 0x44));
        uVar8 = puVar7[1];
        uVar11 = puVar7[2];
        pcVar12 = &DAT_10035fcc;
        pcVar10 = &DAT_10035fd4;
        pcVar9 = &DAT_10035fd8;
        pbVar5 = std::basic_ostream<>::operator<<(pbVar5,*puVar7);
        pbVar5 = std::operator<<(pbVar5,pcVar9);
        pbVar5 = std::basic_ostream<>::operator<<(pbVar5,uVar8);
        pbVar5 = std::operator<<(pbVar5,pcVar10);
        pbVar5 = std::basic_ostream<>::operator<<(pbVar5,uVar11);
        pbVar5 = std::operator<<(pbVar5,pcVar12);
        std::operator<<(pbVar5,pcVar6);
      }
      uStack_4 = 0xffffffff;
      if (bVar1) {
        Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter(aEStack_20);
      }
      uVar3 = srGERD::getDisplayModeCount(*(srGERD **)(param_1 + 0x44));
      if (uVar3 == 0) {
        srGERD::openWindow(*(srGERD **)(param_1 + 0x44));
      }
      else {
        srGERD::openWindow(*(srGERD **)(param_1 + 0x44),0);
      }
    }
  }
  ExceptionList = pvStack_c;
  return;
}



void __cdecl FUN_1000b810(void *param_1,basic_string<> *param_2)

{
  char *pcVar1;
  char cVar2;
  void *this;
  basic_string<> *this_00;
  void **ppvVar3;
  bool bVar4;
  undefined4 uVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  uint uVar9;
  basic_string<> *pbVar10;
  code *pcVar11;
  code *pcVar12;
  undefined auStack_3c [4];
  int iStack_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined auStack_2c [4];
  int iStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined auStack_1c [4];
  int iStack_18;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  this_00 = param_2;
  this = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10022317;
  pvStack_c = ExceptionList;
  pbVar10 = *(basic_string<> **)((int)param_1 + 4);
  iVar8 = 0;
  ExceptionList = &pvStack_c;
  ppvVar3 = &pvStack_c;
  if (pbVar10 != *(basic_string<> **)((int)param_1 + 8)) {
    do {
      bVar4 = std::operator==(pbVar10,this_00);
      if (bVar4) {
        iVar8 = iVar8 + 1;
      }
      pbVar10 = pbVar10 + 0x10;
      ppvVar3 = (void **)ExceptionList;
    } while (pbVar10 != *(basic_string<> **)((int)this + 8));
  }
  ExceptionList = ppvVar3;
  FUN_10015800(this,*(basic_string<> **)((int)this + 8),(basic_string<> *)0x1,this_00);
  if (iVar8 != 0) {
    param_1 = (void *)(iVar8 + 1);
    uVar5 = Housemarque::Grandma::To_String(auStack_1c,&param_1);
    uStack_4 = 0;
    uVar5 = std::operator+(auStack_2c,0x28,uVar5);
    uStack_4._0_1_ = 1;
    iVar8 = std::operator+(auStack_3c,uVar5,0x29);
    uVar9 = *(uint *)npos_exref;
    uStack_4 = CONCAT31(uStack_4._1_3_,2);
    uVar7 = uVar9;
    if (*(uint *)(iVar8 + 8) < uVar9) {
      uVar7 = *(uint *)(iVar8 + 8);
    }
    if (uVar9 - *(int *)(this_00 + 8) <= uVar7) {
      std::_Xlen();
    }
    if (uVar7 != 0) {
      uVar9 = uVar7 + *(int *)(this_00 + 8);
      bVar4 = std::basic_string<>::_Grow(this_00,uVar9,false);
      if (bVar4) {
        pcVar11 = *(code **)(iVar8 + 4);
        if (*(code **)(iVar8 + 4) == (code *)0x0) {
          pcVar11 = _C_exref;
        }
        pcVar12 = (code *)(*(int *)(this_00 + 4) + *(int *)(this_00 + 8));
        for (uVar6 = uVar7 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
          *(undefined4 *)pcVar12 = *(undefined4 *)pcVar11;
          pcVar11 = pcVar11 + 4;
          pcVar12 = pcVar12 + 4;
        }
        for (uVar7 = uVar7 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
          *pcVar12 = *pcVar11;
          pcVar11 = pcVar11 + 1;
          pcVar12 = pcVar12 + 1;
        }
        *(uint *)(this_00 + 8) = uVar9;
        *(undefined *)(*(int *)(this_00 + 4) + uVar9) = 0;
      }
    }
    if (iStack_38 != 0) {
      pcVar1 = (char *)(iStack_38 + -1);
      cVar2 = *pcVar1;
      if ((cVar2 == '\0') || (cVar2 == -1)) {
        FUN_100212a4(pcVar1);
      }
      else {
        *pcVar1 = cVar2 + -1;
      }
    }
    iStack_38 = 0;
    uStack_34 = 0;
    uStack_30 = 0;
    if (iStack_28 != 0) {
      pcVar1 = (char *)(iStack_28 + -1);
      cVar2 = *pcVar1;
      if ((cVar2 == '\0') || (cVar2 == -1)) {
        FUN_100212a4(pcVar1);
      }
      else {
        *pcVar1 = cVar2 + -1;
      }
    }
    iStack_28 = 0;
    uStack_24 = 0;
    uStack_20 = 0;
    if (iStack_18 != 0) {
      pcVar1 = (char *)(iStack_18 + -1);
      cVar2 = *pcVar1;
      if ((cVar2 != '\0') && (cVar2 != -1)) {
        *pcVar1 = cVar2 + -1;
        ExceptionList = pvStack_c;
        return;
      }
      FUN_100212a4(pcVar1);
    }
  }
  ExceptionList = pvStack_c;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Available_Render_Device_List(class
// std::list<class std::basic_string<char,struct std::char_traits<char>,class std::allocator<char>
// >,class std::allocator<class std::basic_string<char,struct std::char_traits<char>,class
// std::allocator<char> > > > &)

void __fastcall Housemarque::Threedee_Engine::Engine::Available_Render_Device_List(list<> *param_1)

{
  char cVar1;
  undefined4 *puVar2;
  basic_string<> *pbVar3;
  bool bVar4;
  char *pcVar5;
  int iVar6;
  uint uVar7;
  uint uVar8;
  int in_EDX;
  basic_string<> *this;
  char *pcVar9;
  uint uStack_40;
  srStringTable local_38 [8];
  uint uStack_30;
  basic_string<> abStack_2c [4];
  char *pcStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined auStack_1c [4];
  basic_string<> *pbStack_18;
  basic_string<> *pbStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  int iStack_4;
  
                    // 0xb9d0  55
                    // ?Available_Render_Device_List@Engine@Threedee_Engine@Housemarque@@QAIXAAV?$list@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@V?$allocator@V?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@2@@std@@@Z
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_1002233b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  srStringTable::srStringTable(local_38);
  iStack_4 = 0;
  srGERD::scanDevices((char *)0x0,local_38);
  *(undefined4 *)(param_1 + 0x44) = 0;
  pbStack_18 = (basic_string<> *)0x0;
  pbStack_14 = (basic_string<> *)0x0;
  uStack_10 = 0;
  iStack_4._0_1_ = 1;
  uStack_40 = 0;
  if (uStack_30 != 0) {
    do {
      pcVar5 = srStringTable::getString(local_38,uStack_40);
      uVar7 = 0xffffffff;
      pcStack_28 = (char *)0x0;
      uStack_24 = 0;
      uStack_20 = 0;
      pcVar9 = pcVar5;
      do {
        if (uVar7 == 0) break;
        uVar7 = uVar7 - 1;
        cVar1 = *pcVar9;
        pcVar9 = pcVar9 + 1;
      } while (cVar1 != '\0');
      uVar7 = ~uVar7 - 1;
      bVar4 = std::basic_string<>::_Grow(abStack_2c,uVar7,true);
      if (bVar4) {
        pcVar9 = pcStack_28;
        for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
          *(undefined4 *)pcVar9 = *(undefined4 *)pcVar5;
          pcVar5 = pcVar5 + 4;
          pcVar9 = pcVar9 + 4;
        }
        for (uVar8 = uVar7 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
          *pcVar9 = *pcVar5;
          pcVar5 = pcVar5 + 1;
          pcVar9 = pcVar9 + 1;
        }
        std::basic_string<>::_Eos(abStack_2c,uVar7);
      }
      iStack_4._0_1_ = 2;
      FUN_1000b810(auStack_1c,abStack_2c);
      puVar2 = (undefined4 *)**(undefined4 **)(in_EDX + 4);
      iVar6 = FUN_10016140(puVar2,puVar2[1]);
      puVar2[1] = iVar6;
      **(int **)(iVar6 + 4) = iVar6;
      FUN_10016560((basic_string<> *)(iVar6 + 8),abStack_2c);
      *(int *)(in_EDX + 8) = *(int *)(in_EDX + 8) + 1;
      iStack_4._0_1_ = 1;
      if (pcStack_28 != (char *)0x0) {
        pcVar5 = pcStack_28 + -1;
        cVar1 = *pcVar5;
        if ((cVar1 == '\0') || (cVar1 == -1)) {
          FUN_100212a4(pcVar5);
        }
        else {
          *pcVar5 = cVar1 + -1;
        }
      }
      uStack_40 = uStack_40 + 1;
      pcStack_28 = (char *)0x0;
      uStack_24 = 0;
      uStack_20 = 0;
    } while (uStack_40 < uStack_30);
  }
  pbVar3 = pbStack_14;
  iStack_4 = (uint)iStack_4._1_3_ << 8;
  for (this = pbStack_18; this != pbVar3; this = this + 0x10) {
    std::basic_string<>::_Tidy(this,true);
  }
  FUN_100212a4(pbStack_18);
  pbStack_18 = (basic_string<> *)0x0;
  pbStack_14 = (basic_string<> *)0x0;
  uStack_10 = 0;
  iStack_4 = 0xffffffff;
  srStringTable::~srStringTable(local_38);
  ExceptionList = pvStack_c;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Get_Display_Modes(class
// std::list<class Housemarque::Threedee_Engine::Engine::Mode,class std::allocator<class
// Housemarque::Threedee_Engine::Engine::Mode> > &,class std::basic_string<char,struct
// std::char_traits<char>,class std::allocator<char> > const &)

void __fastcall
Housemarque::Threedee_Engine::Engine::Get_Display_Modes(list<> *param_1,basic_string<> *param_2)

{
  int *piVar1;
  bool bVar2;
  basic_string<> *pbVar3;
  bool bVar4;
  uint uVar5;
  srGERD *this;
  char *pcVar6;
  undefined4 *puVar7;
  basic_ostream<> *pbVar8;
  int *piVar9;
  uint uVar10;
  uint uVar11;
  int *piVar12;
  basic_string<> *this_00;
  code *pcVar13;
  int *piVar14;
  Inserter IStack00000004;
  char *pcVar15;
  char cVar16;
  int iVar17;
  ulong uStack_50;
  srGERD *local_48;
  int iStack_38;
  char *pcStack_34;
  int iStack_30;
  basic_string<> abStack_2c [4];
  char *pcStack_28;
  int iStack_24;
  Inserter local_1c [4];
  basic_string<> *local_18;
  basic_string<> *local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined *puStack_8;
  int local_4;
  
                    // 0xbbb0  76
                    // ?Get_Display_Modes@Engine@Threedee_Engine@Housemarque@@QAIXAAV?$list@VMode@Engine@Threedee_Engine@Housemarque@@V?$allocator@VMode@Engine@Threedee_Engine@Housemarque@@@std@@@std@@ABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@5@@Z
  puStack_8 = &LAB_1002236e;
  pvStack_c = ExceptionList;
  bVar2 = false;
  local_1c[0] = IStack00000004;
  local_18 = (basic_string<> *)0x0;
  local_14 = (basic_string<> *)0x0;
  local_10 = 0;
  local_4 = 0;
  local_48 = (srGERD *)0x0;
  ExceptionList = &pvStack_c;
  uVar5 = srGERD::getGERDCount();
  uStack_50 = 0;
  if (uVar5 != 0) {
    do {
      this = srGERD::getGERD(uStack_50);
      srGERD::createContext(this,*(ulong *)(param_1 + 0x48));
      pcVar6 = srRuntimeClass::getName((srRuntimeClass *)this);
      abStack_2c[0] = (basic_string<>)IStack00000004;
      std::basic_string<>::_Tidy(abStack_2c,false);
      uVar10 = 0xffffffff;
      pcVar15 = pcVar6;
      do {
        if (uVar10 == 0) break;
        uVar10 = uVar10 - 1;
        cVar16 = *pcVar15;
        pcVar15 = pcVar15 + 1;
      } while (cVar16 != '\0');
      uVar10 = ~uVar10 - 1;
      bVar4 = std::basic_string<>::_Grow(abStack_2c,uVar10,true);
      if (bVar4) {
        pcVar15 = pcStack_28;
        for (uVar11 = uVar10 >> 2; uVar11 != 0; uVar11 = uVar11 - 1) {
          *(undefined4 *)pcVar15 = *(undefined4 *)pcVar6;
          pcVar6 = pcVar6 + 4;
          pcVar15 = pcVar15 + 4;
        }
        for (uVar11 = uVar10 & 3; uVar11 != 0; uVar11 = uVar11 - 1) {
          *pcVar15 = *pcVar6;
          pcVar6 = pcVar6 + 1;
          pcVar15 = pcVar15 + 1;
        }
        std::basic_string<>::_Eos(abStack_2c,uVar10);
      }
      local_4._0_1_ = 1;
      FUN_1000b810(local_1c,abStack_2c);
      srGERD::deleteContext(this);
      bVar4 = std::operator==(abStack_2c,_IStack00000004);
      if (bVar4) {
        local_4 = (uint)local_4._1_3_ << 8;
        std::basic_string<>::_Tidy(abStack_2c,true);
        local_48 = this;
        if (this != (srGERD *)0x0) goto LAB_1000bd71;
        break;
      }
      local_4 = CONCAT31(local_4._1_3_,bVar4);
      std::basic_string<>::_Tidy(abStack_2c,true);
      uStack_50 = uStack_50 + 1;
    } while (uStack_50 < uVar5);
  }
  pcVar13 = *(code **)(_IStack00000004 + 4);
  if (*(code **)(_IStack00000004 + 4) == (code *)0x0) {
    pcVar13 = _C_exref;
  }
  bVar2 = true;
  puVar7 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&stack0x00000004);
  local_4 = CONCAT31(local_4._1_3_,2);
  pbVar8 = std::operator<<((basic_ostream<> *)*puVar7,s_3DE__Cant_find_render_device_nam_10036018);
  cVar16 = '\0';
  pbVar8 = std::operator<<(pbVar8,(char *)pcVar13);
  std::operator<<(pbVar8,cVar16);
  iVar17 = 0x28b;
  pcVar15 = s_E__work_ht_3de_3de_cpp_10036000;
  pcVar6 = Housemarque::Kernel::Error::Tmp_Str();
  Housemarque::Kernel::Throw_Error(s_local_gerd_10036040,pcVar6,pcVar15,iVar17);
LAB_1000bd71:
  uVar5 = 0;
  local_4 = 0;
  if (bVar2) {
    Housemarque::Kernel::Inserter::~Inserter(&stack0x00000004);
  }
  srGERD::createContext(local_48,*(ulong *)(param_1 + 0x48));
  _IStack00000004 = (basic_string<> *)srGERD::getDisplayModeCount(local_48);
  if (_IStack00000004 != (basic_string<> *)0x0) {
    do {
      srGERD::getDisplayModeInfo(*(srGERD **)(param_1 + 0x44),uVar5,(DisplayModeInfo *)&iStack_38);
      iVar17 = iStack_38;
      piVar1 = (int *)**(undefined4 **)(param_2 + 4);
      piVar14 = (int *)piVar1[1];
      pcStack_28 = pcStack_34;
      iStack_24 = iStack_30;
      piVar9 = (int *)operator_new(0x14);
      piVar12 = piVar1;
      if (piVar1 == (int *)0x0) {
        piVar12 = piVar9;
      }
      *piVar9 = (int)piVar12;
      if (piVar14 == (int *)0x0) {
        piVar14 = piVar9;
      }
      piVar9[1] = (int)piVar14;
      piVar1[1] = (int)piVar9;
      *(int **)piVar9[1] = piVar9;
      if (piVar9 + 2 != (int *)0x0) {
        piVar9[2] = iVar17;
        piVar9[3] = (int)pcStack_28;
        piVar9[4] = iStack_24;
      }
      *(int *)(param_2 + 8) = *(int *)(param_2 + 8) + 1;
      uVar5 = uVar5 + 1;
    } while (uVar5 < _IStack00000004);
  }
  srGERD::deleteContext(local_48);
  pbVar3 = local_14;
  local_4 = 0xffffffff;
  for (this_00 = local_18; this_00 != pbVar3; this_00 = this_00 + 0x10) {
    std::basic_string<>::_Tidy(this_00,true);
  }
  FUN_100212a4(local_18);
  ExceptionList = pvStack_c;
  return;
}



// public: bool __fastcall Housemarque::Threedee_Engine::Engine::Set_Render_Device(class
// std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &,class
// Housemarque::Threedee_Engine::Engine::Mode &,bool)

bool __fastcall
Housemarque::Threedee_Engine::Engine::Set_Render_Device
          (basic_string<> *param_1,Mode *param_2,bool param_3)

{
  char cVar1;
  basic_string<> *pbVar2;
  undefined4 *puVar3;
  basic_ostream<> *pbVar4;
  char *pcVar5;
  srGERD *psVar6;
  uint uVar7;
  uint uVar8;
  basic_string<> *pbVar9;
  bool bVar10;
  char *pcVar11;
  int iVar12;
  uint uStack_4c;
  Mode *local_48;
  uint uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  srStringTable local_38 [8];
  uint uStack_30;
  undefined auStack_2c [4];
  basic_string<> *pbStack_28;
  basic_string<> *pbStack_24;
  undefined4 uStack_20;
  basic_string<> abStack_1c [4];
  char *pcStack_18;
  uint uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  uint uStack_4;
  
                    // 0xbe80  142
                    // ?Set_Render_Device@Engine@Threedee_Engine@Housemarque@@QAI_NABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@AAVMode@123@_N@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_100223c2;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_48 = param_2;
  if (*(srGERD **)(param_1 + 0x44) != (srGERD *)0x0) {
    ExceptionList = &pvStack_c;
    srGERD::deleteContext(*(srGERD **)(param_1 + 0x44));
    srGERD::closeWindow(*(srGERD **)(param_1 + 0x44),1);
  }
  srStringTable::srStringTable(local_38);
  uStack_4 = 0;
  srGERD::scanDevices((char *)0x0,local_38);
  bVar10 = uStack_30 == 0;
  if (bVar10) {
    puVar3 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&uStack_4c);
    uStack_4 = CONCAT31(uStack_4._1_3_,1);
    pbVar4 = std::operator<<((basic_ostream<> *)*puVar3,s_No_render_devices_found__10036064);
    std::operator<<(pbVar4,'\0');
    iVar12 = 0x2a3;
    pcVar11 = s_E__work_ht_3de_3de_cpp_1003604c;
    pcVar5 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(s_string_table_getCount___10036080,pcVar5,pcVar11,iVar12);
  }
  uStack_4 = 0;
  if (bVar10) {
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&uStack_4c);
  }
  *(undefined4 *)(param_1 + 0x44) = 0;
  pbStack_28 = (basic_string<> *)0x0;
  pbStack_24 = (basic_string<> *)0x0;
  uStack_20 = 0;
  uStack_4._0_1_ = 2;
  uStack_44 = uStack_30;
  uStack_4c = 0;
  if (uStack_30 != 0) {
    do {
      pcVar5 = srStringTable::getString(local_38,uStack_4c);
      std::basic_string<>::_Tidy(abStack_1c,false);
      uVar7 = 0xffffffff;
      pcVar11 = pcVar5;
      do {
        if (uVar7 == 0) break;
        uVar7 = uVar7 - 1;
        cVar1 = *pcVar11;
        pcVar11 = pcVar11 + 1;
      } while (cVar1 != '\0');
      uVar7 = ~uVar7 - 1;
      bVar10 = std::basic_string<>::_Grow(abStack_1c,uVar7,true);
      if (bVar10) {
        pcVar11 = pcStack_18;
        for (uVar8 = uVar7 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
          *(undefined4 *)pcVar11 = *(undefined4 *)pcVar5;
          pcVar5 = pcVar5 + 4;
          pcVar11 = pcVar11 + 4;
        }
        for (uVar8 = uVar7 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
          *pcVar11 = *pcVar5;
          pcVar5 = pcVar5 + 1;
          pcVar11 = pcVar11 + 1;
        }
        pcStack_18[uVar7] = '\0';
        uStack_14 = uVar7;
      }
      uStack_4._0_1_ = 3;
      FUN_1000b810(auStack_2c,abStack_1c);
      bVar10 = std::operator==(abStack_1c,(basic_string<> *)local_48);
      if (bVar10) {
        psVar6 = srGERD::loadDevice(local_38,uStack_4c);
        *(srGERD **)(param_1 + 0x44) = psVar6;
        srGERD::createContext(psVar6,*(ulong *)(param_1 + 0x48));
        uStack_4._0_1_ = 2;
        std::basic_string<>::_Tidy(abStack_1c,true);
        break;
      }
      uStack_4._0_1_ = 2;
      if (pcStack_18 != (char *)0x0) {
        pcVar5 = pcStack_18 + -1;
        cVar1 = *pcVar5;
        if ((cVar1 == '\0') || (cVar1 == -1)) {
          FUN_100212a4(pcVar5);
        }
        else {
          *pcVar5 = cVar1 + -1;
        }
      }
      uStack_4c = uStack_4c + 1;
      pcStack_18 = (char *)0x0;
      uStack_14 = 0;
      uStack_10 = 0;
    } while (uStack_4c < uStack_44);
  }
  if (*(int *)(param_1 + 0x44) == 0) {
    psVar6 = srGERD::loadDevice(local_38,0);
    *(srGERD **)(param_1 + 0x44) = psVar6;
    srGERD::createContext(psVar6,*(ulong *)(param_1 + 0x48));
    bVar10 = Housemarque::Kernel::Logging_Enabled();
    if (bVar10) {
      iVar12 = Housemarque::Kernel::Error_Log(&uStack_44);
      uStack_4 = CONCAT31(uStack_4._1_3_,4);
      pbVar4 = std::operator<<(*(basic_ostream<> **)(iVar12 + 4),s_Cant_find_render_device__100360b0
                              );
      pcVar5 = srRuntimeClass::getName(*(srRuntimeClass **)(param_1 + 0x44));
      pcVar11 = s_Using_best_available__10036098;
      pbVar4 = std::operator<<(pbVar4,(basic_string<> *)local_48);
      pbVar4 = std::operator<<(pbVar4,pcVar11);
      std::operator<<(pbVar4,pcVar5);
    }
    uStack_4 = 2;
    if (bVar10) {
      Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter((Error_Log_Inserter *)&uStack_44)
      ;
    }
    uStack_44 = 0;
    uStack_40 = 0;
    uStack_3c = 0;
    Open_Window((Mode *)param_1,SUB41(&uStack_44,0));
    pbVar2 = pbStack_24;
    uStack_4 = uStack_4 & 0xffffff00;
    for (pbVar9 = pbStack_28; pbVar9 != pbVar2; pbVar9 = pbVar9 + 0x10) {
      std::basic_string<>::_Tidy(pbVar9,true);
    }
    FUN_100212a4(pbStack_28);
    pbStack_28 = (basic_string<> *)0x0;
    pbStack_24 = (basic_string<> *)0x0;
    uStack_20 = 0;
    uStack_4 = 0xffffffff;
    srStringTable::~srStringTable(local_38);
    bVar10 = false;
  }
  else {
    Open_Window((Mode *)param_1,param_3);
    pbVar2 = pbStack_24;
    uStack_4 = (uint)uStack_4._1_3_ << 8;
    for (pbVar9 = pbStack_28; pbVar9 != pbVar2; pbVar9 = pbVar9 + 0x10) {
      std::basic_string<>::_Tidy(pbVar9,true);
    }
    FUN_100212a4(pbStack_28);
    pbStack_28 = (basic_string<> *)0x0;
    pbStack_24 = (basic_string<> *)0x0;
    uStack_20 = 0;
    uStack_4 = 0xffffffff;
    srStringTable::~srStringTable(local_38);
    bVar10 = true;
  }
  ExceptionList = pvStack_c;
  return bVar10;
}



// public: bool __fastcall Housemarque::Threedee_Engine::Engine::Set_Render_Device(unsigned
// int,class Housemarque::Threedee_Engine::Engine::Mode &,bool)

bool __fastcall
Housemarque::Threedee_Engine::Engine::Set_Render_Device(uint param_1,Mode *param_2,bool param_3)

{
  bool bVar1;
  Mode *pMVar2;
  srGERD *psVar3;
  int iVar4;
  basic_ostream<> *pbVar5;
  char *pcVar6;
  char *pcVar7;
  Error_Log_Inserter aEStack_20 [8];
  undefined4 uStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0xc220  143
                    // ?Set_Render_Device@Engine@Threedee_Engine@Housemarque@@QAI_NIAAVMode@123@_N@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_100223e4;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (*(srGERD **)(param_1 + 0x44) != (srGERD *)0x0) {
    ExceptionList = &pvStack_c;
    srGERD::deleteContext(*(srGERD **)(param_1 + 0x44));
    srGERD::closeWindow(*(srGERD **)(param_1 + 0x44),1);
  }
  *(undefined4 *)(param_1 + 0x44) = 0;
  pMVar2 = (Mode *)srGERD::getGERDCount();
  if (pMVar2 <= param_2) {
    param_2 = pMVar2 + -1;
  }
  psVar3 = srGERD::getGERD((ulong)param_2);
  *(srGERD **)(param_1 + 0x44) = psVar3;
  if (psVar3 == (srGERD *)0x0) {
    psVar3 = srGERD::getFirst();
    *(srGERD **)(param_1 + 0x44) = psVar3;
    bVar1 = Housemarque::Kernel::Logging_Enabled();
    if (bVar1) {
      iVar4 = Housemarque::Kernel::Error_Log(aEStack_20);
      uStack_4 = 0;
      pbVar5 = std::operator<<(*(basic_ostream<> **)(iVar4 + 4),s_Cant_find_render_device__100360e4)
      ;
      pcVar6 = srRuntimeClass::getName(*(srRuntimeClass **)(param_1 + 0x44));
      pcVar7 = s_Using_best_available__100360cc;
      pbVar5 = std::basic_ostream<>::operator<<(pbVar5,(uint)param_2);
      pbVar5 = std::operator<<(pbVar5,pcVar7);
      std::operator<<(pbVar5,pcVar6);
    }
    uStack_4 = 0xffffffff;
    if (bVar1) {
      Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter(aEStack_20);
    }
    uStack_18 = 0;
    uStack_14 = 0;
    uStack_10 = 0;
    Open_Window((Mode *)param_1,SUB41(&uStack_18,0));
    ExceptionList = pvStack_c;
    return false;
  }
  Open_Window((Mode *)param_1,param_3);
  ExceptionList = pvStack_c;
  return true;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Get_Render_Device(class
// std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > &)

void __fastcall Housemarque::Threedee_Engine::Engine::Get_Render_Device(basic_string<> *param_1)

{
  char cVar1;
  int iVar2;
  char *pcVar3;
  uint uVar4;
  uint uVar5;
  basic_string<> *in_EDX;
  char *pcVar6;
  
                    // 0xc360  89
                    // ?Get_Render_Device@Engine@Threedee_Engine@Housemarque@@QAIXAAV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@Z
  pcVar3 = srRuntimeClass::getName(*(srRuntimeClass **)(param_1 + 0x44));
  uVar4 = 0xffffffff;
  pcVar6 = pcVar3;
  do {
    if (uVar4 == 0) break;
    uVar4 = uVar4 - 1;
    cVar1 = *pcVar6;
    pcVar6 = pcVar6 + 1;
  } while (cVar1 != '\0');
  uVar4 = ~uVar4 - 1;
  if (0xfffffffd < uVar4) {
    std::_Xlen();
  }
  iVar2 = *(int *)(in_EDX + 4);
  if (((iVar2 == 0) || (cVar1 = *(char *)(iVar2 + -1), cVar1 == '\0')) || (cVar1 == -1)) {
    if (uVar4 == 0) {
      std::basic_string<>::_Tidy(in_EDX,true);
      return;
    }
    if ((*(uint *)(in_EDX + 0xc) < 0x20) && (uVar4 <= *(uint *)(in_EDX + 0xc))) goto LAB_1000c3e7;
    std::basic_string<>::_Tidy(in_EDX,true);
  }
  else if (uVar4 == 0) {
    *(char *)(iVar2 + -1) = cVar1 + -1;
    *(undefined4 *)(in_EDX + 4) = 0;
    *(undefined4 *)(in_EDX + 8) = 0;
    *(undefined4 *)(in_EDX + 0xc) = 0;
    return;
  }
  std::basic_string<>::_Copy(in_EDX,uVar4);
LAB_1000c3e7:
  pcVar6 = *(char **)(in_EDX + 4);
  for (uVar5 = uVar4 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
    *(undefined4 *)pcVar6 = *(undefined4 *)pcVar3;
    pcVar3 = pcVar3 + 4;
    pcVar6 = pcVar6 + 4;
  }
  for (uVar5 = uVar4 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
    *pcVar6 = *pcVar3;
    pcVar3 = pcVar3 + 1;
    pcVar6 = pcVar6 + 1;
  }
  *(uint *)(in_EDX + 8) = uVar4;
  *(undefined *)(*(int *)(in_EDX + 4) + uVar4) = 0;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Dump(class
// std::basic_ostream<char,struct std::char_traits<char> > &)

void __fastcall Housemarque::Threedee_Engine::Engine::Dump(basic_ostream<> *param_1)

{
  basic_ostream<> *in_EDX;
  
                    // 0xc410  64
                    // ?Dump@Engine@Threedee_Engine@Housemarque@@QAIXAAV?$basic_ostream@DU?$char_traits@D@std@@@std@@@Z
  std::operator<<(in_EDX,s_GERD__10036100);
  (**(code **)(**(int **)(param_1 + 0x44) + 0x18))();
  std::operator<<(in_EDX,s_srCore_10036108);
  srCore::dump((srCore *)srCore_exref,in_EDX);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Reset_Statistics(void)

void __fastcall Housemarque::Threedee_Engine::Engine::Reset_Statistics(void)

{
  int in_ECX;
  
                    // 0xc450  123  ?Reset_Statistics@Engine@Threedee_Engine@Housemarque@@QAIXXZ
                    // WARNING: Could not recover jumptable at 0x1000c453. Too many branches
                    // WARNING: Treating indirect jump as call
  srGERD::resetStatistics(*(srGERD **)(in_ECX + 0x44));
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Flush_RD(void)

void __fastcall Housemarque::Threedee_Engine::Engine::Flush_RD(void)

{
                    // 0xc460  71  ?Flush_RD@Engine@Threedee_Engine@Housemarque@@QAIXXZ
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Engine::Get_Resolution(unsigned int
// &,unsigned int &)

void __fastcall Housemarque::Threedee_Engine::Engine::Get_Resolution(uint *param_1,uint *param_2)

{
  uint uVar1;
  long lVar2;
  long *in_stack_00000004;
  
                    // 0xc470  90  ?Get_Resolution@Engine@Threedee_Engine@Housemarque@@QAIXAAI0@Z
  uVar1 = srGERD::getWidth((srGERD *)param_1[0x11]);
  *param_2 = uVar1;
  lVar2 = srGERD::getHeight((srGERD *)param_1[0x11]);
  *in_stack_00000004 = lVar2;
  return;
}



// WARNING: Removing unreachable block (ram,0x1000c605)
// public: void __fastcall Housemarque::Threedee_Engine::Engine::Add_Texture_Directory(class
// std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &)

void __fastcall Housemarque::Threedee_Engine::Engine::Add_Texture_Directory(basic_string<> *param_1)

{
  basic_string<> *pbVar1;
  int iVar2;
  int iVar3;
  basic_string<> *pbVar4;
  basic_string<> *pbVar5;
  uint uVar6;
  basic_string<> *in_EDX;
  basic_string<> *pbVar7;
  
                    // 0xc4a0  51
                    // ?Add_Texture_Directory@Engine@Threedee_Engine@Housemarque@@QAIXABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@Z
  pbVar1 = *(basic_string<> **)(param_1 + 0x3c);
  if (*(int *)(param_1 + 0x40) - (int)pbVar1 >> 4 != 0) {
    FUN_10016170(pbVar1,pbVar1,pbVar1 + 0x10);
    FUN_100161b0(*(basic_string<> **)(param_1 + 0x3c),
                 1 - ((int)*(basic_string<> **)(param_1 + 0x3c) - (int)pbVar1 >> 4),in_EDX);
    FUN_100162b0(pbVar1,*(basic_string<> **)(param_1 + 0x3c),in_EDX);
    *(int *)(param_1 + 0x3c) = *(int *)(param_1 + 0x3c) + 0x10;
    return;
  }
  iVar2 = *(int *)(param_1 + 0x38);
  if ((iVar2 == 0) || (uVar6 = (int)pbVar1 - iVar2 >> 4, uVar6 < 2)) {
    uVar6 = 1;
  }
  if (iVar2 == 0) {
    iVar2 = 0;
  }
  else {
    iVar2 = (int)pbVar1 - iVar2 >> 4;
  }
  iVar2 = iVar2 + uVar6;
  iVar3 = iVar2;
  if (iVar2 < 0) {
    iVar3 = 0;
  }
  pbVar4 = (basic_string<> *)operator_new(iVar3 << 4);
  pbVar7 = pbVar4;
  for (pbVar5 = *(basic_string<> **)(param_1 + 0x38); pbVar5 != pbVar1; pbVar5 = pbVar5 + 0x10) {
    FUN_10016560(pbVar7,pbVar5);
    pbVar7 = pbVar7 + 0x10;
  }
  FUN_100161b0(pbVar7,1,in_EDX);
  FUN_10016170(pbVar1,*(basic_string<> **)(param_1 + 0x3c),pbVar7 + 0x10);
  FUN_100151c0(*(int *)(param_1 + 0x38),*(int *)(param_1 + 0x3c));
  FUN_100212a4(*(void **)(param_1 + 0x38));
  *(basic_string<> **)(param_1 + 0x40) = pbVar4 + iVar2 * 0x10;
  if (*(int *)(param_1 + 0x38) != 0) {
    *(basic_string<> **)(param_1 + 0x3c) =
         pbVar4 + ((*(int *)(param_1 + 0x3c) - *(int *)(param_1 + 0x38) >> 4) + 1) * 0x10;
    *(basic_string<> **)(param_1 + 0x38) = pbVar4;
    return;
  }
  *(basic_string<> **)(param_1 + 0x3c) = pbVar4 + 0x10;
  *(basic_string<> **)(param_1 + 0x38) = pbVar4;
  return;
}



// WARNING: Removing unreachable block (ram,0x1000c653)
// public: void __fastcall Housemarque::Threedee_Engine::Engine::Clear_Texture_Directories(void)

void __fastcall Housemarque::Threedee_Engine::Engine::Clear_Texture_Directories(void)

{
  basic_string<> *this;
  int *piVar1;
  int *piVar2;
  int *piVar3;
  uchar *puVar4;
  int in_ECX;
  int *piVar5;
  
                    // 0xc640  60
                    // ?Clear_Texture_Directories@Engine@Threedee_Engine@Housemarque@@QAIXXZ
  piVar2 = *(int **)(in_ECX + 0x38);
  piVar3 = *(int **)(in_ECX + 0x3c);
  if (piVar2 != piVar3) {
    piVar5 = piVar2 + 1;
    do {
      if ((char *)*piVar5 != (char *)0x0) {
        this = (basic_string<> *)(piVar5 + -1);
        puVar4 = std::basic_string<>::_Refcnt(this,(char *)*piVar5);
        if ((*puVar4 == '\0') ||
           (puVar4 = std::basic_string<>::_Refcnt(this,(char *)*piVar5), *puVar4 == 0xff)) {
          FUN_100151b0((void *)(*piVar5 + -1));
        }
        else {
          puVar4 = std::basic_string<>::_Refcnt(this,(char *)*piVar5);
          *puVar4 = *puVar4 + 0xff;
        }
      }
      *piVar5 = 0;
      piVar5[1] = 0;
      piVar5[2] = 0;
      piVar1 = piVar5 + 3;
      piVar5 = piVar5 + 4;
    } while (piVar1 != piVar3);
  }
  *(int **)(in_ECX + 0x3c) = piVar2;
  return;
}



basic_string<> * __thiscall FUN_1000c6f0(void *this,basic_string<> *param_1,basic_string<> *param_2)

{
  char cVar1;
  bool bVar2;
  undefined4 uVar3;
  code *pcVar4;
  int iVar5;
  uint uVar6;
  int iVar7;
  uint uVar8;
  code *pcVar9;
  char *pcVar10;
  basic_string<> local_120 [4];
  code *pcStack_11c;
  uint uStack_118;
  void *local_110;
  basic_string<> abStack_10c [4];
  int iStack_108;
  uint uStack_104;
  basic_string<> abStack_fc [16];
  basic_string<> local_ec [16];
  basic_string<> local_dc [4];
  int iStack_d8;
  basic_string<> abStack_cc [16];
  basic_string<> abStack_bc [16];
  basic_string<> local_ac [16];
  code *pcStack_9c;
  byte abStack_98 [4];
  basic_streambuf<> abStack_94 [84];
  code *apcStack_40 [13];
  void *pvStack_c;
  undefined *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_100224a8;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_110 = this;
  FUN_1000cfc0(local_dc,(int)param_2);
  local_4 = 1;
  FUN_1000ce70(local_ec,(int)param_2);
  iVar7 = *(int *)((int)this + 0x38);
  local_4._0_1_ = 2;
  if (iVar7 != *(int *)((int)this + 0x3c)) {
    do {
      uVar3 = std::operator+(local_ac,local_dc,iVar7);
      local_4._0_1_ = 3;
      std::operator+(local_120,uVar3,local_ec);
      local_4._0_1_ = 5;
      std::basic_string<>::_Tidy(local_ac,true);
      FUN_1000cbc0(abStack_cc,(int)local_120);
      local_4._0_1_ = 6;
      bVar2 = std::operator==(abStack_cc,&DAT_10036114);
      if (bVar2) {
        std::basic_string<>::_Tidy(abStack_fc,false);
        uVar6 = 0xffffffff;
        pcVar10 = &DAT_10036118;
        do {
          if (uVar6 == 0) break;
          uVar6 = uVar6 - 1;
          cVar1 = *pcVar10;
          pcVar10 = pcVar10 + 1;
        } while (cVar1 != '\0');
        std::basic_string<>::assign(abStack_fc,&DAT_10036118,~uVar6 - 1);
        local_4._0_1_ = 7;
        std::basic_string<>::_Tidy(abStack_10c,false);
        uVar6 = 0xffffffff;
        pcVar10 = &DAT_1003611c;
        do {
          if (uVar6 == 0) break;
          uVar6 = uVar6 - 1;
          cVar1 = *pcVar10;
          pcVar10 = pcVar10 + 1;
        } while (cVar1 != '\0');
        std::basic_string<>::assign(abStack_10c,&DAT_1003611c,~uVar6 - 1);
        local_4._0_1_ = 8;
        FUN_1000cd20(abStack_bc,(int)local_120);
        local_4 = CONCAT31(local_4._1_3_,9);
        FUN_1000d120(abStack_bc);
        bVar2 = std::operator==(abStack_bc,abStack_fc);
        if ((bVar2) && (uVar6 = uStack_118 - 3, this = local_110, uVar6 != 0)) {
          uVar8 = 0;
          do {
            pcVar9 = _C_exref;
            if ((uVar8 <= uStack_104) && (iStack_108 != 0)) {
              std::basic_string<>::_Freeze(abStack_10c);
              pcVar9 = (code *)(iStack_108 + uVar8);
            }
            pcVar4 = _C_exref;
            if ((uVar6 <= uStack_118) && (pcStack_11c != (code *)0x0)) {
              std::basic_string<>::_Freeze(local_120);
              pcVar4 = pcStack_11c + uVar6;
            }
            uVar6 = uVar6 + 1;
            uVar8 = uVar8 + 1;
            *pcVar4 = *pcVar9;
            this = local_110;
          } while (uVar8 < 3);
        }
        local_4._0_1_ = 8;
        std::basic_string<>::_Tidy(abStack_bc,true);
        local_4._0_1_ = 7;
        std::basic_string<>::_Tidy(abStack_10c,true);
        local_4._0_1_ = 6;
        std::basic_string<>::_Tidy(abStack_fc,true);
      }
      pcVar9 = pcStack_11c;
      if (pcStack_11c == (code *)0x0) {
        pcVar9 = _C_exref;
      }
      pcStack_9c = _vbtable__exref;
      std::basic_ios<>::basic_ios<>((basic_ios<> *)apcStack_40);
      local_4 = CONCAT31(local_4._1_3_,10);
      std::basic_istream<>::basic_istream<>((basic_istream<> *)&pcStack_9c,abStack_94,false);
      local_4 = 0xb;
      std::basic_filebuf<>::basic_filebuf<>((basic_filebuf<> *)abStack_94,(_iobuf *)0x0);
      local_4 = CONCAT31(local_4._1_3_,0xc);
      *(code **)(abStack_98 + *(int *)(pcStack_9c + 4) + -4) = _vftable__exref;
      iVar5 = std::basic_filebuf<>::open((char *)pcVar9,1);
      if (iVar5 == 0) {
        std::basic_ios<>::setstate
                  ((basic_ios<> *)(abStack_98 + *(int *)(pcStack_9c + 4) + -4),2,false);
      }
      local_4._0_1_ = 0xd;
      if ((abStack_98[*(int *)(pcStack_9c + 4)] & 6) == 0) {
        *param_1 = local_120[0];
        std::basic_string<>::_Tidy(param_1,false);
        std::basic_string<>::assign(param_1,local_120,0,*(uint *)npos_exref);
        local_4._0_1_ = 6;
        std::basic_ifstream<>::~basic_ifstream<>((basic_ifstream<> *)apcStack_40);
        std::basic_ios<>::~basic_ios<>((basic_ios<> *)apcStack_40);
        local_4._0_1_ = 5;
        std::basic_string<>::_Tidy(abStack_cc,true);
        local_4._0_1_ = 2;
        std::basic_string<>::_Tidy(local_120,true);
        local_4._0_1_ = 1;
        std::basic_string<>::_Tidy(local_ec,true);
        local_4 = (uint)local_4._1_3_ << 8;
        std::basic_string<>::_Tidy(local_dc,true);
        ExceptionList = pvStack_c;
        return param_1;
      }
      local_4._0_1_ = 6;
      std::basic_ifstream<>::~basic_ifstream<>((basic_ifstream<> *)apcStack_40);
      apcStack_40[0] = _vftable__exref;
      std::ios_base::~ios_base((ios_base *)apcStack_40);
      local_4._0_1_ = 5;
      std::basic_string<>::_Tidy(abStack_cc,true);
      local_4._0_1_ = 2;
      std::basic_string<>::_Tidy(local_120,true);
      iVar7 = iVar7 + 0x10;
    } while (iVar7 != *(int *)((int)this + 0x3c));
  }
  *param_1 = *param_2;
  std::basic_string<>::_Tidy(param_1,false);
  std::basic_string<>::assign(param_1,param_2,0,*(uint *)npos_exref);
  local_4 = CONCAT31(local_4._1_3_,1);
  std::basic_string<>::_Tidy(local_ec,true);
  if (iStack_d8 != 0) {
    cVar1 = *(char *)(iStack_d8 + -1);
    if ((cVar1 == '\0') || (cVar1 == -1)) {
      FUN_100212a4((void *)(iStack_d8 + -1));
    }
    else {
      *(char *)(iStack_d8 + -1) = cVar1 + -1;
    }
  }
  ExceptionList = pvStack_c;
  return param_1;
}



basic_string<> * __cdecl FUN_1000cbc0(basic_string<> *param_1,int param_2)

{
  char *pcVar1;
  code cVar2;
  char cVar3;
  int iVar4;
  int iVar5;
  code *pcVar6;
  basic_string<> *pbVar7;
  uint uVar8;
  int iVar9;
  bool bVar10;
  basic_string<> local_2c [4];
  int iStack_28;
  undefined auStack_1c [4];
  int iStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  int iStack_4;
  
  iVar5 = param_2;
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_100224c4;
  pvStack_c = ExceptionList;
  local_2c[0] = param_2._0_1_;
  ExceptionList = &pvStack_c;
  std::basic_string<>::_Tidy(local_2c,false);
  uVar8 = *(uint *)(param_2 + 8) - 1;
  iVar9 = 0;
  iStack_4 = 0;
  if (uVar8 < *(uint *)(param_2 + 8)) {
    do {
      iVar4 = *(int *)(iVar5 + 4);
      pcVar6 = _C_exref;
      if (iVar4 != 0) {
        pcVar6 = (code *)(iVar4 + uVar8);
      }
      cVar2 = *pcVar6;
      param_2 = (int)(byte)cVar2;
      if ((cVar2 == (code)0x2f) || (cVar2 == (code)0x5c)) {
        iVar9 = iVar9 + 1;
        bVar10 = iVar9 == 1;
LAB_1000cca7:
        if (iVar9 != 0 && !bVar10) break;
      }
      else {
        bVar10 = iVar9 == 1;
        if (!bVar10) goto LAB_1000cca7;
        pbVar7 = (basic_string<> *)std::operator+(auStack_1c,param_2,local_2c);
        iStack_4._0_1_ = 1;
        std::basic_string<>::assign(local_2c,pbVar7,0,*(uint *)npos_exref);
        iStack_4 = (uint)iStack_4._1_3_ << 8;
        if (iStack_18 == 0) {
LAB_1000cc95:
          iStack_18 = 0;
          uStack_14 = 0;
          uStack_10 = 0;
        }
        else {
          pcVar1 = (char *)(iStack_18 + -1);
          cVar3 = *pcVar1;
          if ((cVar3 == '\0') || (cVar3 == -1)) {
            FUN_100212a4(pcVar1);
            goto LAB_1000cc95;
          }
          *pcVar1 = cVar3 + -1;
          iStack_18 = 0;
          uStack_14 = 0;
          uStack_10 = 0;
        }
      }
      uVar8 = uVar8 - 1;
    } while (uVar8 < *(uint *)(iVar5 + 8));
  }
  *param_1 = local_2c[0];
  *(undefined4 *)(param_1 + 4) = 0;
  *(undefined4 *)(param_1 + 8) = 0;
  *(undefined4 *)(param_1 + 0xc) = 0;
  std::basic_string<>::assign(param_1,local_2c,0,*(uint *)npos_exref);
  if (iStack_28 != 0) {
    cVar3 = *(char *)(iStack_28 + -1);
    if ((cVar3 == '\0') || (cVar3 == -1)) {
      FUN_100212a4((void *)(iStack_28 + -1));
    }
    else {
      *(char *)(iStack_28 + -1) = cVar3 + -1;
    }
  }
  ExceptionList = pvStack_c;
  return param_1;
}



basic_string<> * __cdecl FUN_1000cd20(basic_string<> *param_1,int param_2)

{
  char *pcVar1;
  char cVar2;
  int iVar3;
  int iVar4;
  code *pcVar5;
  basic_string<> *pbVar6;
  uint uVar7;
  basic_string<> local_2c [4];
  int iStack_28;
  undefined auStack_1c [4];
  int iStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  int iStack_4;
  
  iVar4 = param_2;
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_100224e0;
  pvStack_c = ExceptionList;
  local_2c[0] = param_2._0_1_;
  ExceptionList = &pvStack_c;
  std::basic_string<>::_Tidy(local_2c,false);
  uVar7 = *(uint *)(param_2 + 8) - 1;
  iStack_4 = 0;
  if (uVar7 < *(uint *)(param_2 + 8)) {
    do {
      iVar3 = *(int *)(iVar4 + 4);
      pcVar5 = _C_exref;
      if (iVar3 != 0) {
        pcVar5 = (code *)(iVar3 + uVar7);
      }
      param_2 = (int)(byte)*pcVar5;
      if (*pcVar5 == (code)0x2e) break;
      pbVar6 = (basic_string<> *)std::operator+(auStack_1c,param_2,local_2c);
      iStack_4._0_1_ = 1;
      std::basic_string<>::assign(local_2c,pbVar6,0,*(uint *)npos_exref);
      iStack_4 = (uint)iStack_4._1_3_ << 8;
      if (iStack_18 != 0) {
        pcVar1 = (char *)(iStack_18 + -1);
        cVar2 = *pcVar1;
        if ((cVar2 == '\0') || (cVar2 == -1)) {
          FUN_100212a4(pcVar1);
        }
        else {
          *pcVar1 = cVar2 + -1;
        }
      }
      uVar7 = uVar7 - 1;
      iStack_18 = 0;
      uStack_14 = 0;
      uStack_10 = 0;
    } while (uVar7 < *(uint *)(iVar4 + 8));
  }
  *param_1 = local_2c[0];
  *(undefined4 *)(param_1 + 4) = 0;
  *(undefined4 *)(param_1 + 8) = 0;
  *(undefined4 *)(param_1 + 0xc) = 0;
  std::basic_string<>::assign(param_1,local_2c,0,*(uint *)npos_exref);
  if (iStack_28 != 0) {
    cVar2 = *(char *)(iStack_28 + -1);
    if ((cVar2 != '\0') && (cVar2 != -1)) {
      *(char *)(iStack_28 + -1) = cVar2 + -1;
      ExceptionList = pvStack_c;
      return param_1;
    }
    FUN_100212a4((void *)(iStack_28 + -1));
  }
  ExceptionList = pvStack_c;
  return param_1;
}



basic_string<> * __cdecl FUN_1000ce70(basic_string<> *param_1,int param_2)

{
  char *pcVar1;
  code cVar2;
  char cVar3;
  int iVar4;
  int iVar5;
  code *pcVar6;
  basic_string<> *pbVar7;
  uint uVar8;
  basic_string<> local_2c [4];
  int iStack_28;
  undefined auStack_1c [4];
  int iStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  int iStack_4;
  
  iVar5 = param_2;
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_100224fc;
  pvStack_c = ExceptionList;
  local_2c[0] = param_2._0_1_;
  ExceptionList = &pvStack_c;
  std::basic_string<>::_Tidy(local_2c,false);
  uVar8 = *(uint *)(param_2 + 8) - 1;
  iStack_4 = 0;
  if (uVar8 < *(uint *)(param_2 + 8)) {
    do {
      iVar4 = *(int *)(iVar5 + 4);
      pcVar6 = _C_exref;
      if (iVar4 != 0) {
        pcVar6 = (code *)(iVar4 + uVar8);
      }
      cVar2 = *pcVar6;
      param_2 = (int)(byte)cVar2;
      if ((cVar2 == (code)0x2f) || (cVar2 == (code)0x5c)) break;
      pbVar7 = (basic_string<> *)std::operator+(auStack_1c,param_2,local_2c);
      iStack_4._0_1_ = 1;
      std::basic_string<>::assign(local_2c,pbVar7,0,*(uint *)npos_exref);
      iStack_4 = (uint)iStack_4._1_3_ << 8;
      if (iStack_18 != 0) {
        pcVar1 = (char *)(iStack_18 + -1);
        cVar3 = *pcVar1;
        if ((cVar3 == '\0') || (cVar3 == -1)) {
          FUN_100212a4(pcVar1);
        }
        else {
          *pcVar1 = cVar3 + -1;
        }
      }
      uVar8 = uVar8 - 1;
      iStack_18 = 0;
      uStack_14 = 0;
      uStack_10 = 0;
    } while (uVar8 < *(uint *)(iVar5 + 8));
  }
  *param_1 = local_2c[0];
  *(undefined4 *)(param_1 + 4) = 0;
  *(undefined4 *)(param_1 + 8) = 0;
  *(undefined4 *)(param_1 + 0xc) = 0;
  std::basic_string<>::assign(param_1,local_2c,0,*(uint *)npos_exref);
  if (iStack_28 != 0) {
    cVar3 = *(char *)(iStack_28 + -1);
    if ((cVar3 != '\0') && (cVar3 != -1)) {
      *(char *)(iStack_28 + -1) = cVar3 + -1;
      ExceptionList = pvStack_c;
      return param_1;
    }
    FUN_100212a4((void *)(iStack_28 + -1));
  }
  ExceptionList = pvStack_c;
  return param_1;
}



basic_string<> * __cdecl FUN_1000cfc0(basic_string<> *param_1,int param_2)

{
  char *pcVar1;
  code cVar2;
  char cVar3;
  int iVar4;
  bool bVar5;
  int iVar6;
  code *pcVar7;
  basic_string<> *pbVar8;
  uint uVar9;
  basic_string<> local_2c [4];
  int iStack_28;
  undefined auStack_1c [4];
  int iStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  int iStack_4;
  
  iVar6 = param_2;
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_10022518;
  pvStack_c = ExceptionList;
  local_2c[0] = param_2._0_1_;
  ExceptionList = &pvStack_c;
  std::basic_string<>::_Tidy(local_2c,false);
  uVar9 = *(uint *)(param_2 + 8) - 1;
  iStack_4 = 0;
  bVar5 = false;
  if (uVar9 < *(uint *)(param_2 + 8)) {
    do {
      iVar4 = *(int *)(iVar6 + 4);
      pcVar7 = _C_exref;
      if (iVar4 != 0) {
        pcVar7 = (code *)(iVar4 + uVar9);
      }
      cVar2 = *pcVar7;
      param_2 = (int)(byte)cVar2;
      if ((cVar2 == (code)0x2f) || (cVar2 == (code)0x5c)) {
        bVar5 = true;
LAB_1000d039:
        pbVar8 = (basic_string<> *)std::operator+(auStack_1c,param_2,local_2c);
        iStack_4._0_1_ = 1;
        std::basic_string<>::assign(local_2c,pbVar8,0,*(uint *)npos_exref);
        iStack_4 = (uint)iStack_4._1_3_ << 8;
        if (iStack_18 != 0) {
          pcVar1 = (char *)(iStack_18 + -1);
          cVar3 = *pcVar1;
          if ((cVar3 == '\0') || (cVar3 == -1)) {
            FUN_100212a4(pcVar1);
          }
          else {
            *pcVar1 = cVar3 + -1;
          }
        }
        iStack_18 = 0;
        uStack_14 = 0;
        uStack_10 = 0;
      }
      else if (bVar5) goto LAB_1000d039;
      uVar9 = uVar9 - 1;
    } while (uVar9 < *(uint *)(iVar6 + 8));
  }
  *param_1 = local_2c[0];
  *(undefined4 *)(param_1 + 4) = 0;
  *(undefined4 *)(param_1 + 8) = 0;
  *(undefined4 *)(param_1 + 0xc) = 0;
  std::basic_string<>::assign(param_1,local_2c,0,*(uint *)npos_exref);
  if (iStack_28 != 0) {
    cVar3 = *(char *)(iStack_28 + -1);
    if ((cVar3 != '\0') && (cVar3 != -1)) {
      *(char *)(iStack_28 + -1) = cVar3 + -1;
      ExceptionList = pvStack_c;
      return param_1;
    }
    FUN_100212a4((void *)(iStack_28 + -1));
  }
  ExceptionList = pvStack_c;
  return param_1;
}



void __cdecl FUN_1000d120(basic_string<> *param_1)

{
  char cVar1;
  uint uVar2;
  int iVar3;
  uint uVar4;
  code *pcVar5;
  uint uVar6;
  code *pcVar7;
  
  uVar6 = *(uint *)(param_1 + 8);
  uVar4 = 0;
  if (uVar6 != 0) {
    do {
      pcVar5 = _C_exref;
      if (*(int *)(param_1 + 4) != 0) {
        cVar1 = *(char *)(*(int *)(param_1 + 4) + -1);
        if ((cVar1 != '\0') && (cVar1 != -1)) {
          uVar2 = std::basic_string<>::max_size(param_1);
          if (uVar2 < uVar6) {
            std::_Xlen();
          }
          iVar3 = *(int *)(param_1 + 4);
          if (((iVar3 == 0) || (cVar1 = *(char *)(iVar3 + -1), cVar1 == '\0')) || (cVar1 == -1)) {
            if (uVar6 == 0) {
              if (iVar3 != 0) {
                std::basic_string<>::_Eos(param_1,0);
              }
            }
            else if (*(uint *)(param_1 + 0xc) < uVar6) goto LAB_1000d19b;
          }
          else if (uVar6 == 0) {
            *(char *)(iVar3 + -1) = cVar1 + -1;
            std::basic_string<>::_Tidy(param_1,false);
          }
          else {
LAB_1000d19b:
            std::basic_string<>::_Copy(param_1,uVar6);
          }
        }
        if (*(int *)(param_1 + 4) != 0) {
          *(undefined *)(*(int *)(param_1 + 4) + -1) = 0xff;
        }
        pcVar5 = (code *)(*(int *)(param_1 + 4) + uVar4);
      }
      uVar6 = *(uint *)(param_1 + 8);
      pcVar7 = _C_exref;
      if ((uVar4 <= uVar6) && (*(int *)(param_1 + 4) != 0)) {
        cVar1 = *(char *)(*(int *)(param_1 + 4) + -1);
        if ((cVar1 != '\0') && (cVar1 != -1)) {
          uVar2 = std::basic_string<>::max_size(param_1);
          if (uVar2 < uVar6) {
            std::_Xlen();
          }
          iVar3 = *(int *)(param_1 + 4);
          if (((iVar3 == 0) || (cVar1 = *(char *)(iVar3 + -1), cVar1 == '\0')) || (cVar1 == -1)) {
            if (uVar6 == 0) {
              if (iVar3 != 0) {
                std::basic_string<>::_Eos(param_1,0);
              }
            }
            else if (*(uint *)(param_1 + 0xc) < uVar6) goto LAB_1000d225;
          }
          else if (uVar6 == 0) {
            *(char *)(iVar3 + -1) = cVar1 + -1;
            std::basic_string<>::_Tidy(param_1,false);
          }
          else {
LAB_1000d225:
            std::basic_string<>::_Copy(param_1,uVar6);
          }
        }
        if (*(int *)(param_1 + 4) != 0) {
          *(undefined *)(*(int *)(param_1 + 4) + -1) = 0xff;
        }
        pcVar7 = (code *)(*(int *)(param_1 + 4) + uVar4);
      }
      iVar3 = toupper((int)(char)*pcVar5);
      *pcVar7 = SUB41(iVar3,0);
      uVar6 = *(uint *)(param_1 + 8);
      uVar4 = uVar4 + 1;
    } while (uVar4 < uVar6);
  }
  return;
}



// public: __thiscall Housemarque::Threedee_Engine::Scene::Scene(char const *,class
// Housemarque::Threedee_Engine::Engine *)

Scene * __thiscall
Housemarque::Threedee_Engine::Scene::Scene(Scene *this,char *param_1,Engine *param_2)

{
  srScene *psVar1;
  srCamera *this_00;
  int iVar2;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0xd270  19  ??0Scene@Threedee_Engine@Housemarque@@QAE@PBDPAVEngine@12@@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10022538;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *this = (Scene)0x0;
  this[1] = (Scene)0x1;
  *(undefined4 *)(this + 8) = 0;
  *(undefined4 *)(this + 0xc) = 0;
  *(undefined4 *)(this + 0x20) = 0;
  psVar1 = (srScene *)srHeap::allocate((srHeap *)srHeap_exref,0x188);
  uStack_4 = 0;
  if (psVar1 == (srScene *)0x0) {
    psVar1 = (srScene *)0x0;
  }
  else {
    srScene::srScene(psVar1,(srNode *)0x0);
    *(undefined ***)psVar1 = &PTR_FUN_10025be4;
  }
  uStack_4 = 0xffffffff;
  *(srScene **)(this + 8) = psVar1;
  srRuntimeClass::setName((srRuntimeClass *)psVar1,param_1);
  this_00 = (srCamera *)srHeap::allocate((srHeap *)srHeap_exref,0x188);
  uStack_4 = 1;
  if (this_00 == (srCamera *)0x0) {
    this_00 = (srCamera *)0x0;
  }
  else {
    srCamera::srCamera(this_00,(srNode *)0x0);
    *(undefined ***)this_00 = &PTR_FUN_10025ba4;
  }
  uStack_4 = 0xffffffff;
  *(srCamera **)(this + 0xc) = this_00;
  srRuntimeClass::setName((srRuntimeClass *)this_00,param_1);
  iVar2 = 1;
  psVar1 = Get_Sr_Scene();
  srNode::setParent(*(srNode **)(this + 0xc),(srNode *)psVar1,iVar2);
  *(Engine **)(this + 4) = param_2;
  *(undefined4 *)(this + 0x10) = 0;
  *(undefined4 *)(this + 0x14) = 0;
  *(undefined4 *)(this + 0x18) = 0;
  *(undefined4 *)(this + 0x1c) = 0;
  iVar2 = *(int *)(this + 8);
  *(undefined4 *)(iVar2 + 0x16c) = 0;
  *(undefined4 *)(iVar2 + 0x170) = 0;
  *(undefined4 *)(iVar2 + 0x174) = 0;
  ExceptionList = pvStack_c;
  return this;
}



// public: __thiscall Housemarque::Threedee_Engine::Scene::~Scene(void)

void __thiscall Housemarque::Threedee_Engine::Scene::~Scene(Scene *this)

{
                    // 0xd380  39  ??1Scene@Threedee_Engine@Housemarque@@QAE@XZ
  if (*(undefined4 **)(this + 0xc) != (undefined4 *)0x0) {
    (**(code **)**(undefined4 **)(this + 0xc))(1);
  }
  if (*(undefined4 **)(this + 8) != (undefined4 *)0x0) {
    (**(code **)**(undefined4 **)(this + 8))(1);
  }
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Scene::Set_Sort(bool)

void __fastcall Housemarque::Threedee_Engine::Scene::Set_Sort(bool param_1)

{
  undefined3 in_register_00000005;
  undefined in_DL;
  
                    // 0xd3a0  145  ?Set_Sort@Scene@Threedee_Engine@Housemarque@@QAIX_N@Z
  *(undefined *)CONCAT31(in_register_00000005,param_1) = in_DL;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Scene::Set_Clear(bool)

void __fastcall Housemarque::Threedee_Engine::Scene::Set_Clear(bool param_1)

{
  undefined3 in_register_00000005;
  undefined in_DL;
  
                    // 0xd3b0  133  ?Set_Clear@Scene@Threedee_Engine@Housemarque@@QAIX_N@Z
  *(undefined *)(CONCAT31(in_register_00000005,param_1) + 1) = in_DL;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Scene::Set_Ranges(float,float)

void __fastcall Housemarque::Threedee_Engine::Scene::Set_Ranges(float param_1,float param_2)

{
  float in_stack_00000004;
  float in_stack_00000008;
  
                    // 0xd3c0  141  ?Set_Ranges@Scene@Threedee_Engine@Housemarque@@QAIXMM@Z
  srCamera::setClipRange
            (*(srCamera **)((int)param_1 + 0xc),(double)in_stack_00000004,(double)in_stack_00000008)
  ;
  srCamera::setEnvironmentScale(*(srCamera **)((int)param_1 + 0xc),1.0,1.0);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Scene::Set_Ambient_Color(float,float,float)

void __fastcall
Housemarque::Threedee_Engine::Scene::Set_Ambient_Color(float param_1,float param_2,float param_3)

{
  int iVar1;
  undefined4 in_stack_00000008;
  undefined4 in_stack_0000000c;
  
                    // 0xd400  127  ?Set_Ambient_Color@Scene@Threedee_Engine@Housemarque@@QAIXMMM@Z
  iVar1 = *(int *)((int)param_1 + 8);
  *(float *)(iVar1 + 0x16c) = param_3;
  *(undefined4 *)(iVar1 + 0x170) = in_stack_00000008;
  *(undefined4 *)(iVar1 + 0x174) = in_stack_0000000c;
  return;
}



// public: void __fastcall
// Housemarque::Threedee_Engine::Scene::Set_Background_Color(float,float,float)

void __fastcall
Housemarque::Threedee_Engine::Scene::Set_Background_Color(float param_1,float param_2,float param_3)

{
  undefined4 in_stack_00000008;
  undefined4 in_stack_0000000c;
  
                    // 0xd420  128
                    // ?Set_Background_Color@Scene@Threedee_Engine@Housemarque@@QAIXMMM@Z
  *(float *)((int)param_1 + 0x10) = param_3;
  *(undefined4 *)((int)param_1 + 0x14) = in_stack_00000008;
  *(undefined4 *)((int)param_1 + 0x18) = in_stack_0000000c;
  *(undefined4 *)((int)param_1 + 0x1c) = 0;
  return;
}



// public: void __fastcall
// Housemarque::Threedee_Engine::Scene::Set_Plane(float,float,float,float,float)

void __fastcall
Housemarque::Threedee_Engine::Scene::Set_Plane
          (float param_1,float param_2,float param_3,float param_4,float param_5)

{
  float in_stack_00000014;
  
                    // 0xd440  139  ?Set_Plane@Scene@Threedee_Engine@Housemarque@@QAIXMMMMM@Z
  srCamera::setViewPlane
            (*(srCamera **)((int)param_1 + 0xc),(Rect *)&stack0xffffffe0,
             (double)CONCAT44(SUB84((double)param_3,0),
                              (int)((ulonglong)(double)in_stack_00000014 >> 0x20)));
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Scene::Get_Plane(float &,float &,float
// &,float &,float &)

void __fastcall
Housemarque::Threedee_Engine::Scene::Get_Plane
          (float *param_1,float *param_2,float *param_3,float *param_4,float *param_5)

{
  float *in_stack_00000010;
  double local_28;
  double local_20;
  double dStack_18;
  double dStack_10;
  double dStack_8;
  
                    // 0xd490  83  ?Get_Plane@Scene@Threedee_Engine@Housemarque@@QAIXAAM0000@Z
  srCamera::getViewPlane((srCamera *)param_1[3],(Rect *)&local_20,&local_28);
  *in_stack_00000010 = (float)local_28;
  *param_2 = (float)local_20;
  *param_3 = (float)dStack_18;
  *param_4 = (float)dStack_10;
  *param_5 = (float)dStack_8;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Scene::Set_Polygon_Mode(enum
// Housemarque::Threedee_Engine::Scene::Polygon_Mode)

void __fastcall Housemarque::Threedee_Engine::Scene::Set_Polygon_Mode(Polygon_Mode param_1)

{
  undefined4 in_EDX;
  
                    // 0xd4e0  140
                    // ?Set_Polygon_Mode@Scene@Threedee_Engine@Housemarque@@QAIXW4Polygon_Mode@123@@Z
  *(undefined4 *)(param_1 + 0x20) = in_EDX;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Scene::Begin_Frame(void)

void __fastcall Housemarque::Threedee_Engine::Scene::Begin_Frame(void)

{
  srGERD *this;
  bool bVar1;
  int in_ECX;
  e_polygonMode eVar2;
  undefined4 uStack_14;
  undefined4 uStack_10;
  undefined4 uStack_c;
  undefined4 uStack_8;
  undefined4 uStack_4;
  
                    // 0xd4f0  57  ?Begin_Frame@Scene@Threedee_Engine@Housemarque@@QAIXXZ
  this = *(srGERD **)(*(int *)(in_ECX + 4) + 0x44);
  bVar1 = false;
  if (*(int *)(in_ECX + 0x20) == 1) {
    eVar2 = 1;
  }
  else {
    if (*(int *)(in_ECX + 0x20) != 2) {
      srGERD::setPolygonMode(this,2);
      goto LAB_1000d525;
    }
    eVar2 = 0;
  }
  srGERD::setPolygonMode(this,eVar2);
  bVar1 = true;
LAB_1000d525:
  srGERD::beginFrame(this);
  if (bVar1) {
    uStack_10 = *(undefined4 *)(in_ECX + 0x10);
    uStack_c = *(undefined4 *)(in_ECX + 0x14);
    uStack_8 = *(undefined4 *)(in_ECX + 0x18);
    uStack_4 = *(undefined4 *)(in_ECX + 0x1c);
    srGERD::setClearColor(this,(srVector4T<float> *)&uStack_10);
    uStack_14 = 3;
    srGERD::clear(this,(srFlags<> *)&uStack_14);
  }
  uStack_14 = 2;
  srGERD::clear(this,(srFlags<> *)&uStack_14);
  if (*(int *)(this + 0x1648) != 0) {
    *(undefined4 *)(this + 0x1648) = 0;
    *(uint *)(this + 0x24) = *(uint *)(this + 0x24) | 0x2000;
  }
  srGERD::setDepthRange(this,0.004999999888241291,0.9950000047683716);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Scene::End_Frame(void)

void __fastcall Housemarque::Threedee_Engine::Scene::End_Frame(void)

{
  srGERD *this;
  int in_ECX;
  
                    // 0xd5c0  69  ?End_Frame@Scene@Threedee_Engine@Housemarque@@QAIXXZ
  this = *(srGERD **)(*(int *)(in_ECX + 4) + 0x44);
  srGERD::endFrame(this);
  srGERD::flipFrame(this);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Scene::Render(void)

void __fastcall Housemarque::Threedee_Engine::Scene::Render(void)

{
  srGERD *this;
  char *in_ECX;
  
                    // 0xd5e0  122  ?Render@Scene@Threedee_Engine@Housemarque@@QAIXXZ
  this = *(srGERD **)(*(int *)(in_ECX + 4) + 0x44);
  if ((*in_ECX != '\0') && (((byte)this[0x20] & 2) == 0)) {
    srGERD::toggle(this,1);
  }
  srScene::render(*(srScene **)(in_ECX + 8),this,*(srCamera **)(in_ECX + 0xc));
  if (*in_ECX != '\0') {
    srGERD::flushRenderers(this);
    if (((byte)this[0x20] & 2) != 0) {
      srGERD::toggle(this,1);
    }
  }
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Scene::Set_Camera(class
// Housemarque::Template_Library::Vector_3<float> const &,class
// Housemarque::Template_Library::Matrix_3x3<float> const &)

void __fastcall
Housemarque::Threedee_Engine::Scene::Set_Camera(Vector_3<float> *param_1,Matrix_3x3<float> *param_2)

{
  int iVar1;
  srMatrix3T<float> *psVar2;
  undefined4 *in_stack_00000004;
  double local_3c;
  double local_34;
  double local_2c;
  srMatrix3T<float> local_24 [36];
  
                    // 0xd630  132
                    // ?Set_Camera@Scene@Threedee_Engine@Housemarque@@QAIXABV?$Vector_3@M@Template_Library@3@ABV?$Matrix_3x3@M@53@@Z
  local_3c = (double)*(float *)param_2;
  local_34 = (double)*(float *)(param_2 + 4);
  local_2c = (double)*(float *)(param_2 + 8);
  psVar2 = local_24;
  for (iVar1 = 9; iVar1 != 0; iVar1 = iVar1 + -1) {
    *(undefined4 *)psVar2 = *in_stack_00000004;
    in_stack_00000004 = in_stack_00000004 + 1;
    psVar2 = psVar2 + 4;
  }
  srNode::setLocation(*(srNode **)(param_1 + 0xc),(srVector3T<double> *)&local_3c);
  srNode::setRotation(*(srNode **)(param_1 + 0xc),local_24);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Scene::Get_Camera(class
// Housemarque::Template_Library::Vector_3<float> &,class
// Housemarque::Template_Library::Matrix_3x3<float> &)

void __fastcall
Housemarque::Threedee_Engine::Scene::Get_Camera(Vector_3<float> *param_1,Matrix_3x3<float> *param_2)

{
  int iVar1;
  srMatrix3T<float> *psVar2;
  undefined4 *in_stack_00000004;
  undefined4 local_30;
  undefined4 uStack_2c;
  undefined4 uStack_28;
  srMatrix3T<float> asStack_24 [36];
  
                    // 0xd690  75
                    // ?Get_Camera@Scene@Threedee_Engine@Housemarque@@QAIXAAV?$Vector_3@M@Template_Library@3@AAV?$Matrix_3x3@M@53@@Z
  srNode::getLocation(*(srNode **)(param_1 + 0xc),(srVector3T<float> *)&local_30);
  srNode::getRotation(*(srNode **)(param_1 + 0xc),asStack_24);
  *(undefined4 *)param_2 = local_30;
  *(undefined4 *)(param_2 + 4) = uStack_2c;
  *(undefined4 *)(param_2 + 8) = uStack_28;
  psVar2 = asStack_24;
  for (iVar1 = 9; iVar1 != 0; iVar1 = iVar1 + -1) {
    *in_stack_00000004 = *(undefined4 *)psVar2;
    psVar2 = psVar2 + 4;
    in_stack_00000004 = in_stack_00000004 + 1;
  }
  return;
}



// public: class srScene * __fastcall Housemarque::Threedee_Engine::Scene::Get_Sr_Scene(void)

srScene * __fastcall Housemarque::Threedee_Engine::Scene::Get_Sr_Scene(void)

{
  int in_ECX;
  
                    // 0xd6e0  95
                    // ?Get_Sr_Scene@Scene@Threedee_Engine@Housemarque@@QAIPAVsrScene@@XZ
  return *(srScene **)(in_ECX + 8);
}



// public: void __fastcall Housemarque::Threedee_Engine::Scene::Dump(class
// std::basic_ostream<char,struct std::char_traits<char> > &)

void __fastcall Housemarque::Threedee_Engine::Scene::Dump(basic_ostream<> *param_1)

{
  basic_ostream<> *in_EDX;
  
                    // 0xd6f0  65
                    // ?Dump@Scene@Threedee_Engine@Housemarque@@QAIXAAV?$basic_ostream@DU?$char_traits@D@std@@@std@@@Z
  (**(code **)(**(int **)(param_1 + 8) + 0x18))();
  std::operator<<(in_EDX,s_Camera__10036120);
  (**(code **)(**(int **)(param_1 + 0xc) + 0x18))();
  return;
}



// public: __thiscall Housemarque::Threedee_Engine::Trimesh_Config::Trimesh_Config(void)

Trimesh_Config * __thiscall
Housemarque::Threedee_Engine::Trimesh_Config::Trimesh_Config(Trimesh_Config *this)

{
  basic_string<> *pbVar1;
  char cVar2;
  uint uVar3;
  char *pcVar4;
  basic_string<> bStack_59;
  basic_string<> *pbStack_58;
  Trimesh_Config *local_54;
  basic_string<> *pbStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  basic_string<> abStack_3c [16];
  basic_string<> abStack_2c [16];
  basic_string<> abStack_1c [16];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0xd720  25  ??0Trimesh_Config@Threedee_Engine@Housemarque@@QAE@XZ
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_100225d2;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_54 = this;
  Housemarque::Template_Library::Config_Base::Config_Base((Config_Base *)this);
  uStack_4 = 0;
  abStack_3c[0] = bStack_59;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_not_defined_1003612c;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_not_defined_1003612c,~uVar3 - 1);
  uStack_4c = CONCAT31(uStack_4c._1_3_,bStack_59);
  uStack_4._0_1_ = 1;
  std::basic_string<>::_Tidy((basic_string<> *)&uStack_4c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_hx_file_10036138;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign((basic_string<> *)&uStack_4c,s_hx_file_10036138,~uVar3 - 1);
  pbVar1 = (basic_string<> *)(this + 0xc);
  uStack_4._0_1_ = 2;
  pbStack_58 = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_3c);
  uStack_4._0_1_ = 3;
  pbStack_50 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 4;
  if (pbStack_50 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_50,(basic_string<> *)&uStack_4c,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 7;
  std::basic_string<>::_Tidy((basic_string<> *)&uStack_4c,true);
  uStack_4._0_1_ = 6;
  std::basic_string<>::_Tidy(abStack_3c,true);
  abStack_1c[0] = bStack_59;
  std::basic_string<>::_Tidy(abStack_1c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_not_defined_10036140;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_1c,s_not_defined_10036140,~uVar3 - 1);
  uStack_4._0_1_ = 8;
  abStack_2c[0] = bStack_59;
  std::basic_string<>::_Tidy(abStack_2c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_hx_mesh_name_space_model_1003614c;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_2c,s_hx_mesh_name_space_model_1003614c,~uVar3 - 1);
  pbVar1 = (basic_string<> *)(this + 0x3c);
  uStack_4._0_1_ = 9;
  pbStack_50 = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_1c);
  uStack_4._0_1_ = 10;
  pbStack_58 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0xb;
  if (pbStack_58 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_58,abStack_2c,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0xe;
  std::basic_string<>::_Tidy(abStack_2c,true);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_1c,true);
  abStack_3c[0] = bStack_59;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_minimum_visible_size_in_pixels_10036168;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_minimum_visible_size_in_pixels_10036168,~uVar3 - 1);
  uStack_4._0_1_ = 0xf;
  pbStack_58 = (basic_string<> *)0x0;
  FUN_10014500(this + 0xec,(Config_Base *)this,abStack_3c,&pbStack_58);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_3c,true);
  abStack_3c[0] = bStack_59;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_align_to_camera_10036188;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_align_to_camera_10036188,~uVar3 - 1);
  uStack_4._0_1_ = 0x10;
  bStack_59 = (basic_string<>)0x0;
  FUN_10014460(this + 0xf0,(Config_Base *)this,abStack_3c,&bStack_59);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_3c,true);
  abStack_3c[0] = bStack_59;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_scale_10036198;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_scale_10036198,~uVar3 - 1);
  uStack_4c = 0x3c23d70a;
  *(undefined4 *)(this + 0xf4) = 0x3c23d70a;
  uStack_48 = 0x3c23d70a;
  uStack_44 = 0x3c23d70a;
  *(undefined4 *)(this + 0xf8) = 0x3c23d70a;
  uStack_4._0_1_ = 0x11;
  *(undefined4 *)(this + 0xfc) = 0x3c23d70a;
  pbStack_50 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0x12;
  if (pbStack_50 != (basic_string<> *)0x0) {
    FUN_100155a0(pbStack_50,abStack_3c,(Config_Base *)this,this + 0xf4);
  }
  uStack_4 = CONCAT31(uStack_4._1_3_,0xd);
  std::basic_string<>::_Tidy(abStack_3c,true);
  FUN_1000daa0((Config_Base *)this,(int)(this + 0x1c),(int)(this + 0xcc),(int)(this + 0x4c),
               (int)(this + 0x6c),(int)(this + 0x8c),(int)(this + 0xac));
  ExceptionList = pvStack_c;
  return this;
}



void __cdecl
FUN_1000daa0(Config_Base *param_1,int param_2,int param_3,int param_4,int param_5,int param_6,
            int param_7)

{
  char cVar1;
  bool bVar2;
  basic_string<> *pbVar3;
  basic_string<> *pbVar4;
  void *pvVar5;
  undefined4 uVar6;
  undefined4 *puVar7;
  Item_Ref_Base *this;
  uint uVar8;
  uint uVar9;
  undefined4 *puVar10;
  char *pcVar11;
  undefined4 *puVar12;
  basic_string<> local_169;
  uint local_168;
  basic_string<> *local_164;
  int local_160;
  undefined4 *puStack_15c;
  undefined local_158 [4];
  int iStack_154;
  undefined4 uStack_150;
  undefined4 uStack_14c;
  basic_string<> abStack_148 [4];
  undefined4 *puStack_144;
  basic_string<> *pbStack_140;
  undefined4 uStack_13c;
  undefined4 uStack_138;
  undefined4 uStack_134;
  undefined4 uStack_130;
  basic_string<> abStack_12c [4];
  undefined4 *puStack_128;
  basic_string<> abStack_11c [4];
  undefined4 *puStack_118;
  basic_string<> abStack_10c [16];
  basic_string<> abStack_fc [16];
  basic_string<> local_ec [16];
  undefined auStack_dc [4];
  int iStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  basic_string<> abStack_cc [16];
  CPP_Identifier aCStack_bc [16];
  basic_string<> abStack_ac [16];
  basic_string<> abStack_9c [16];
  basic_string<> abStack_8c [16];
  basic_string<> abStack_7c [16];
  basic_string<> abStack_6c [16];
  basic_string<> abStack_5c [16];
  basic_string<> abStack_4c [16];
  basic_string<> abStack_3c [16];
  basic_string<> abStack_2c [16];
  basic_string<> abStack_1c [16];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10022898;
  pvStack_c = ExceptionList;
  uVar9 = 0;
  local_168 = 0;
  local_160 = 0;
  ExceptionList = &pvStack_c;
  do {
    Housemarque::Grandma::To_String(local_158,&local_160);
    local_4 = 0;
    pbVar3 = (basic_string<> *)operator_new(0x10);
    local_4._0_1_ = 1;
    local_164 = pbVar3;
    if (pbVar3 == (basic_string<> *)0x0) {
      pbVar3 = (basic_string<> *)0x0;
    }
    else {
      local_ec[0] = local_169;
      std::basic_string<>::_Tidy(local_ec,false);
      uVar8 = 0xffffffff;
      pcVar11 = s_not_defined_100361a0;
      do {
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        cVar1 = *pcVar11;
        pcVar11 = pcVar11 + 1;
      } while (cVar1 != '\0');
      std::basic_string<>::assign(local_ec,s_not_defined_100361a0,~uVar8 - 1);
      local_168 = uVar9 | 1;
      abStack_cc[0] = local_169;
      local_4 = CONCAT31(local_4._1_3_,2);
      std::basic_string<>::_Tidy(abStack_cc,false);
      uVar8 = 0xffffffff;
      pcVar11 = s_hx_mesh_name_lod_100361ac;
      do {
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        cVar1 = *pcVar11;
        pcVar11 = pcVar11 + 1;
      } while (cVar1 != '\0');
      std::basic_string<>::assign(abStack_cc,s_hx_mesh_name_lod_100361ac,~uVar8 - 1);
      local_168 = uVar9 | 3;
      local_4 = 3;
      pbVar4 = (basic_string<> *)std::operator+(abStack_4c,abStack_cc,local_158);
      uVar9 = uVar9 | 7;
      local_4 = 4;
      local_168 = uVar9;
      std::basic_string<>::basic_string<>(pbVar3,local_ec);
      local_4 = 5;
      puStack_15c = (undefined4 *)operator_new(0x20);
      local_4 = CONCAT31(local_4._1_3_,6);
      if (puStack_15c != (undefined4 *)0x0) {
        FUN_10014030(puStack_15c,pbVar4,param_1,pbVar3);
      }
    }
    *(basic_string<> **)(param_2 + local_160 * 4) = pbVar3;
    local_4 = 8;
    if ((uVar9 & 4) != 0) {
      uVar9 = uVar9 & 0xfffffffb;
      local_168 = uVar9;
      std::basic_string<>::_Tidy(abStack_4c,true);
    }
    local_4 = 7;
    if ((uVar9 & 2) != 0) {
      uVar9 = uVar9 & 0xfffffffd;
      local_168 = uVar9;
      std::basic_string<>::_Tidy(abStack_cc,true);
    }
    local_4 = 0;
    if ((uVar9 & 1) != 0) {
      uVar9 = uVar9 & 0xfffffffe;
      std::basic_string<>::_Tidy(local_ec,true);
    }
    pvVar5 = operator_new(4);
    local_4._0_1_ = 10;
    puStack_15c = (undefined4 *)pvVar5;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      abStack_10c[0] = local_169;
      std::basic_string<>::_Tidy(abStack_10c,false);
      uVar8 = 0xffffffff;
      pcVar11 = &DAT_100361d0;
      do {
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        cVar1 = *pcVar11;
        pcVar11 = pcVar11 + 1;
      } while (cVar1 != '\0');
      std::basic_string<>::assign(abStack_10c,&DAT_100361d0,~uVar8 - 1);
      local_168 = uVar9 | 8;
      local_4 = CONCAT31(local_4._1_3_,0xb);
      uVar6 = std::operator+(abStack_ac,abStack_10c,local_158);
      local_168 = uVar9 | 0x18;
      local_4 = 0xc;
      pbVar3 = (basic_string<> *)std::operator+(abStack_8c,uVar6,s__size_in_pixels_100361c0);
      uVar9 = uVar9 | 0x38;
      local_4 = 0xd;
      local_164 = (basic_string<> *)0x0;
      local_168 = uVar9;
      FUN_100145a0(pvVar5,param_1,pbVar3,&local_164);
    }
    *(void **)(param_3 + local_160 * 4) = pvVar5;
    local_4 = 0xf;
    if ((uVar9 & 0x20) != 0) {
      uVar9 = uVar9 & 0xffffffdf;
      local_168 = uVar9;
      std::basic_string<>::_Tidy(abStack_8c,true);
    }
    local_4 = 0xe;
    if ((uVar9 & 0x10) != 0) {
      uVar9 = uVar9 & 0xffffffef;
      local_168 = uVar9;
      std::basic_string<>::_Tidy(abStack_ac,true);
    }
    local_4 = 0;
    if ((uVar9 & 8) != 0) {
      uVar9 = uVar9 & 0xfffffff7;
      std::basic_string<>::_Tidy(abStack_10c,true);
    }
    pvVar5 = operator_new(4);
    local_4._0_1_ = 0x11;
    puStack_15c = (undefined4 *)pvVar5;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      abStack_fc[0] = local_169;
      std::basic_string<>::_Tidy(abStack_fc,false);
      uVar8 = 0xffffffff;
      pcVar11 = &DAT_100361e8;
      do {
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        cVar1 = *pcVar11;
        pcVar11 = pcVar11 + 1;
      } while (cVar1 != '\0');
      std::basic_string<>::assign(abStack_fc,&DAT_100361e8,~uVar8 - 1);
      local_168 = uVar9 | 0x40;
      local_4 = CONCAT31(local_4._1_3_,0x12);
      uVar6 = std::operator+(abStack_2c,abStack_fc,local_158);
      local_168 = uVar9 | 0xc0;
      local_4 = 0x13;
      pbVar3 = (basic_string<> *)std::operator+(abStack_6c,uVar6,s__ambient_coeff_100361d8);
      uVar9 = uVar9 | 0x1c0;
      local_4 = 0x14;
      local_164 = (basic_string<> *)0x3f800000;
      local_168 = uVar9;
      FUN_100145a0(pvVar5,param_1,pbVar3,&local_164);
    }
    *(void **)(param_4 + local_160 * 4) = pvVar5;
    local_4 = 0x16;
    if ((uVar9 & 0x100) != 0) {
      uVar9 = uVar9 & 0xfffffeff;
      local_168 = uVar9;
      std::basic_string<>::_Tidy(abStack_6c,true);
    }
    local_4 = 0x15;
    if ((uVar9 & 0x80) != 0) {
      uVar9 = uVar9 & 0xffffff7f;
      local_168 = uVar9;
      std::basic_string<>::_Tidy(abStack_2c,true);
    }
    local_4 = 0;
    if ((uVar9 & 0x40) != 0) {
      uVar9 = uVar9 & 0xffffffbf;
      std::basic_string<>::_Tidy(abStack_fc,true);
    }
    pvVar5 = operator_new(4);
    local_4 = CONCAT31(local_4._1_3_,0x18);
    puStack_15c = (undefined4 *)pvVar5;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      abStack_12c[0] = local_169;
      std::basic_string<>::_Tidy(abStack_12c,false);
      uVar8 = 0xffffffff;
      pcVar11 = &DAT_100361fc;
      do {
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        cVar1 = *pcVar11;
        pcVar11 = pcVar11 + 1;
      } while (cVar1 != '\0');
      pbVar3 = (basic_string<> *)(~uVar8 - 1);
      local_164 = pbVar3;
      bVar2 = std::basic_string<>::_Grow(abStack_12c,(uint)pbVar3,true);
      if (bVar2) {
        puVar7 = (undefined4 *)&DAT_100361fc;
        puVar10 = puStack_128;
        for (uVar8 = (uint)pbVar3 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
          *puVar10 = *puVar7;
          puVar7 = puVar7 + 1;
          puVar10 = puVar10 + 1;
        }
        for (uVar8 = (uint)pbVar3 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
          *(undefined *)puVar10 = *(undefined *)puVar7;
          puVar7 = (undefined4 *)((int)puVar7 + 1);
          puVar10 = (undefined4 *)((int)puVar10 + 1);
        }
        std::basic_string<>::_Eos(abStack_12c,(uint)pbVar3);
      }
      local_168 = uVar9 | 0x200;
      local_4 = CONCAT31(local_4._1_3_,0x19);
      uVar6 = std::operator+(abStack_7c,abStack_12c,local_158);
      local_168 = uVar9 | 0x600;
      local_4 = 0x1a;
      pbVar3 = (basic_string<> *)std::operator+(abStack_9c,uVar6,s__sun_coeff_100361f0);
      uVar9 = uVar9 | 0xe00;
      local_4 = 0x1b;
      local_164 = (basic_string<> *)0x3f800000;
      local_168 = uVar9;
      FUN_100145a0(pvVar5,param_1,pbVar3,&local_164);
    }
    *(void **)(param_5 + local_160 * 4) = pvVar5;
    local_4 = 0x1d;
    if ((uVar9 & 0x800) != 0) {
      uVar9 = uVar9 & 0xfffff7ff;
      local_168 = uVar9;
      std::basic_string<>::_Tidy(abStack_9c,true);
    }
    local_4 = 0x1c;
    if ((uVar9 & 0x400) != 0) {
      uVar9 = uVar9 & 0xfffffbff;
      local_168 = uVar9;
      std::basic_string<>::_Tidy(abStack_7c,true);
    }
    local_4 = 0;
    if ((uVar9 & 0x200) != 0) {
      uVar9 = uVar9 & 0xfffffdff;
      std::basic_string<>::_Tidy(abStack_12c,true);
    }
    pvVar5 = operator_new(1);
    local_4 = CONCAT31(local_4._1_3_,0x1f);
    puStack_15c = (undefined4 *)pvVar5;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      abStack_11c[0] = local_169;
      std::basic_string<>::_Tidy(abStack_11c,false);
      uVar8 = 0xffffffff;
      pcVar11 = &DAT_10036214;
      do {
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        cVar1 = *pcVar11;
        pcVar11 = pcVar11 + 1;
      } while (cVar1 != '\0');
      pbVar3 = (basic_string<> *)(~uVar8 - 1);
      local_164 = pbVar3;
      bVar2 = std::basic_string<>::_Grow(abStack_11c,(uint)pbVar3,true);
      if (bVar2) {
        puVar7 = (undefined4 *)&DAT_10036214;
        puVar10 = puStack_118;
        for (uVar8 = (uint)pbVar3 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
          *puVar10 = *puVar7;
          puVar7 = puVar7 + 1;
          puVar10 = puVar10 + 1;
        }
        for (uVar8 = (uint)pbVar3 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
          *(undefined *)puVar10 = *(undefined *)puVar7;
          puVar7 = (undefined4 *)((int)puVar7 + 1);
          puVar10 = (undefined4 *)((int)puVar10 + 1);
        }
        std::basic_string<>::_Eos(abStack_11c,(uint)pbVar3);
      }
      local_168 = uVar9 | 0x1000;
      local_4 = CONCAT31(local_4._1_3_,0x20);
      uVar6 = std::operator+(abStack_3c,abStack_11c,local_158);
      local_168 = uVar9 | 0x3000;
      local_4 = 0x21;
      pbVar3 = (basic_string<> *)std::operator+(abStack_5c,uVar6,s__face_to_sun_10036204);
      uVar9 = uVar9 | 0x7000;
      local_4 = 0x22;
      local_169 = (basic_string<>)0x0;
      local_168 = uVar9;
      FUN_10014460(pvVar5,param_1,pbVar3,&local_169);
    }
    *(void **)(param_6 + local_160 * 4) = pvVar5;
    local_4 = 0x24;
    if ((uVar9 & 0x4000) != 0) {
      uVar9 = uVar9 & 0xffffbfff;
      local_168 = uVar9;
      std::basic_string<>::_Tidy(abStack_5c,true);
    }
    local_4 = 0x23;
    if ((uVar9 & 0x2000) != 0) {
      uVar9 = uVar9 & 0xffffdfff;
      local_168 = uVar9;
      std::basic_string<>::_Tidy(abStack_3c,true);
    }
    local_4 = 0;
    if ((uVar9 & 0x1000) != 0) {
      uVar9 = uVar9 & 0xffffefff;
      std::basic_string<>::_Tidy(abStack_11c,true);
    }
    puVar7 = (undefined4 *)operator_new(0xc);
    local_4._0_1_ = 0x26;
    puStack_15c = puVar7;
    if (puVar7 == (undefined4 *)0x0) {
      puVar7 = (undefined4 *)0x0;
    }
    else {
      uStack_138 = 0;
      uStack_134 = 0;
      uStack_130 = 0;
      abStack_148[0] = local_169;
      std::basic_string<>::_Tidy(abStack_148,false);
      uVar8 = 0xffffffff;
      pcVar11 = &DAT_10036228;
      do {
        if (uVar8 == 0) break;
        uVar8 = uVar8 - 1;
        cVar1 = *pcVar11;
        pcVar11 = pcVar11 + 1;
      } while (cVar1 != '\0');
      pbVar3 = (basic_string<> *)(~uVar8 - 1);
      local_164 = pbVar3;
      bVar2 = std::basic_string<>::_Grow(abStack_148,(uint)pbVar3,true);
      if (bVar2) {
        puVar10 = (undefined4 *)&DAT_10036228;
        puVar12 = puStack_144;
        for (uVar8 = (uint)pbVar3 >> 2; uVar8 != 0; uVar8 = uVar8 - 1) {
          *puVar12 = *puVar10;
          puVar10 = puVar10 + 1;
          puVar12 = puVar12 + 1;
        }
        for (uVar8 = (uint)pbVar3 & 3; uVar8 != 0; uVar8 = uVar8 - 1) {
          *(undefined *)puVar12 = *(undefined *)puVar10;
          puVar10 = (undefined4 *)((int)puVar10 + 1);
          puVar12 = (undefined4 *)((int)puVar12 + 1);
        }
        *(basic_string<> *)((int)puStack_144 + (int)pbVar3) = (basic_string<>)0x0;
        pbStack_140 = pbVar3;
      }
      local_168 = uVar9 | 0x8000;
      local_4 = CONCAT31(local_4._1_3_,0x27);
      uVar6 = std::operator+(auStack_dc,abStack_148,local_158);
      local_168 = uVar9 | 0x18000;
      local_4 = 0x28;
      pbVar3 = (basic_string<> *)std::operator+(abStack_1c,uVar6,s__base_color_1003621c);
      uVar9 = uVar9 | 0x38000;
      *puVar7 = uStack_138;
      puVar7[1] = uStack_134;
      local_4 = 0x29;
      puVar7[2] = uStack_130;
      local_168 = uVar9;
      this = (Item_Ref_Base *)operator_new(0x20);
      local_4 = 0x2a;
      local_164 = (basic_string<> *)this;
      if (this != (Item_Ref_Base *)0x0) {
        Housemarque::Template_Library::CPP_Identifier::CPP_Identifier(aCStack_bc,pbVar3);
        local_4._0_1_ = 0x2b;
        Housemarque::Template_Library::Item_Ref_Base::Item_Ref_Base(this,aCStack_bc,param_1);
        local_4 = CONCAT31(local_4._1_3_,0x2d);
        Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier(aCStack_bc);
        *(undefined4 **)(this + 0x1c) = puVar7;
        *(undefined ***)this = &PTR_FUN_10025c44;
      }
    }
    *(undefined4 **)(param_7 + local_160 * 4) = puVar7;
    local_4 = 0x2f;
    if ((uVar9 & 0x20000) != 0) {
      uVar9 = uVar9 & 0xfffdffff;
      local_168 = uVar9;
      std::basic_string<>::_Tidy(abStack_1c,true);
    }
    if ((uVar9 & 0x10000) != 0) {
      uVar9 = uVar9 & 0xfffeffff;
      if (iStack_d8 != 0) {
        pcVar11 = (char *)(iStack_d8 + -1);
        cVar1 = *pcVar11;
        if ((cVar1 == '\0') || (cVar1 == -1)) {
          FUN_100212a4(pcVar11);
        }
        else {
          *pcVar11 = cVar1 + -1;
        }
      }
      iStack_d8 = 0;
      uStack_d4 = 0;
      uStack_d0 = 0;
    }
    if ((uVar9 & 0x8000) != 0) {
      uVar9 = uVar9 & 0xffff7fff;
      if (puStack_144 != (undefined4 *)0x0) {
        pcVar11 = (char *)((int)puStack_144 + -1);
        cVar1 = *pcVar11;
        if ((cVar1 == '\0') || (cVar1 == -1)) {
          FUN_100212a4(pcVar11);
        }
        else {
          *pcVar11 = cVar1 + -1;
        }
      }
      puStack_144 = (undefined4 *)0x0;
      pbStack_140 = (basic_string<> *)0x0;
      uStack_13c = 0;
    }
    local_4 = 0xffffffff;
    if (iStack_154 != 0) {
      cVar1 = *(char *)(iStack_154 + -1);
      if ((cVar1 == '\0') || (cVar1 == -1)) {
        FUN_100212a4((void *)(iStack_154 + -1));
      }
      else {
        *(char *)(iStack_154 + -1) = cVar1 + -1;
      }
    }
    local_160 = local_160 + 1;
    iStack_154 = 0;
    uStack_150 = 0;
    uStack_14c = 0;
    if (7 < local_160) {
      ExceptionList = pvStack_c;
      return;
    }
  } while( true );
}



// public: __thiscall Housemarque::Threedee_Engine::Trimesh_Config::Trimesh_Config(char const *)

Trimesh_Config * __thiscall
Housemarque::Threedee_Engine::Trimesh_Config::Trimesh_Config(Trimesh_Config *this,char *param_1)

{
  basic_string<> *pbVar1;
  int iVar2;
  undefined4 *puVar3;
  basic_ostream<> *pbVar4;
  char *pcVar5;
  uint uVar6;
  char *pcVar7;
  char cVar8;
  basic_string<> bStack_e9;
  void *local_e8;
  basic_string<> *pbStack_e4;
  basic_string<> abStack_e0 [16];
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  basic_string<> abStack_c0 [16];
  basic_string<> abStack_b0 [16];
  Trimesh_Config *local_a0;
  code *pcStack_9c;
  byte abStack_98 [4];
  basic_streambuf<> abStack_94 [84];
  basic_ios<> abStack_40 [52];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0xe500  24  ??0Trimesh_Config@Threedee_Engine@Housemarque@@QAE@PBD@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_100229cb;
  pvStack_c = ExceptionList;
  local_e8 = (void *)0x0;
  ExceptionList = &pvStack_c;
  local_a0 = this;
  Housemarque::Template_Library::Config_Base::Config_Base((Config_Base *)this);
  uStack_4 = 0;
  abStack_e0[0] = bStack_e9;
  std::basic_string<>::_Tidy(abStack_e0,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_not_defined_10036230;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_e0,s_not_defined_10036230,~uVar6 - 1);
  uStack_d0 = CONCAT31(uStack_d0._1_3_,bStack_e9);
  uStack_4._0_1_ = 1;
  std::basic_string<>::_Tidy((basic_string<> *)&uStack_d0,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_hx_file_1003623c;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign((basic_string<> *)&uStack_d0,s_hx_file_1003623c,~uVar6 - 1);
  pbVar1 = (basic_string<> *)(this + 0xc);
  uStack_4._0_1_ = 2;
  pbStack_e4 = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_e0);
  uStack_4._0_1_ = 3;
  local_e8 = operator_new(0x20);
  uStack_4._0_1_ = 4;
  if (local_e8 != (void *)0x0) {
    FUN_10014030(local_e8,(basic_string<> *)&uStack_d0,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 7;
  std::basic_string<>::_Tidy((basic_string<> *)&uStack_d0,true);
  uStack_4._0_1_ = 6;
  std::basic_string<>::_Tidy(abStack_e0,true);
  abStack_b0[0] = bStack_e9;
  std::basic_string<>::_Tidy(abStack_b0,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_not_defined_10036244;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_b0,s_not_defined_10036244,~uVar6 - 1);
  uStack_4._0_1_ = 8;
  abStack_c0[0] = bStack_e9;
  std::basic_string<>::_Tidy(abStack_c0,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_hx_mesh_name_space_model_10036250;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_c0,s_hx_mesh_name_space_model_10036250,~uVar6 - 1);
  pbVar1 = (basic_string<> *)(this + 0x3c);
  uStack_4._0_1_ = 9;
  pbStack_e4 = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_b0);
  uStack_4._0_1_ = 10;
  local_e8 = operator_new(0x20);
  uStack_4._0_1_ = 0xb;
  if (local_e8 != (void *)0x0) {
    FUN_10014030(local_e8,abStack_c0,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0xe;
  std::basic_string<>::_Tidy(abStack_c0,true);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_b0,true);
  abStack_e0[0] = bStack_e9;
  std::basic_string<>::_Tidy(abStack_e0,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_minimum_visible_size_in_pixels_1003626c;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_e0,s_minimum_visible_size_in_pixels_1003626c,~uVar6 - 1);
  uStack_4._0_1_ = 0xf;
  local_e8 = (void *)0x0;
  FUN_10014500(this + 0xec,(Config_Base *)this,abStack_e0,&local_e8);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_e0,true);
  abStack_e0[0] = bStack_e9;
  std::basic_string<>::_Tidy(abStack_e0,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_align_to_camera_1003628c;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_e0,s_align_to_camera_1003628c,~uVar6 - 1);
  uStack_4._0_1_ = 0x10;
  bStack_e9 = (basic_string<>)0x0;
  FUN_10014460(this + 0xf0,(Config_Base *)this,abStack_e0,&bStack_e9);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_e0,true);
  abStack_e0[0] = bStack_e9;
  std::basic_string<>::_Tidy(abStack_e0,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_scale_1003629c;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_e0,s_scale_1003629c,~uVar6 - 1);
  uStack_d0 = 0x3c23d70a;
  *(undefined4 *)(this + 0xf4) = 0x3c23d70a;
  uStack_cc = 0x3c23d70a;
  uStack_c8 = 0x3c23d70a;
  *(undefined4 *)(this + 0xf8) = 0x3c23d70a;
  uStack_4._0_1_ = 0x11;
  *(undefined4 *)(this + 0xfc) = 0x3c23d70a;
  pbStack_e4 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0x12;
  if (pbStack_e4 != (basic_string<> *)0x0) {
    FUN_100155a0(pbStack_e4,abStack_e0,(Config_Base *)this,this + 0xf4);
  }
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_e0,true);
  FUN_1000daa0((Config_Base *)this,(int)(this + 0x1c),(int)(this + 0xcc),(int)(this + 0x4c),
               (int)(this + 0x6c),(int)(this + 0x8c),(int)(this + 0xac));
  pcStack_9c = _vbtable__exref;
  std::basic_ios<>::basic_ios<>(abStack_40);
  local_e8 = (void *)0x2;
  uStack_4 = CONCAT31(uStack_4._1_3_,0x13);
  std::basic_istream<>::basic_istream<>((basic_istream<> *)&pcStack_9c,abStack_94,false);
  uStack_4 = 0x14;
  std::basic_filebuf<>::basic_filebuf<>((basic_filebuf<> *)abStack_94,(_iobuf *)0x0);
  uStack_4 = CONCAT31(uStack_4._1_3_,0x15);
  *(code **)(abStack_98 + *(int *)(pcStack_9c + 4) + -4) = _vftable__exref;
  iVar2 = std::basic_filebuf<>::open(param_1,1);
  if (iVar2 == 0) {
    std::basic_ios<>::setstate((basic_ios<> *)(abStack_98 + *(int *)(pcStack_9c + 4) + -4),2,false);
  }
  uStack_4._0_1_ = 0x16;
  if ((abStack_98[*(int *)(pcStack_9c + 4)] & 6) != 0) {
    local_e8 = (void *)0x3;
    puVar3 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&pbStack_e4);
    uStack_4 = CONCAT31(uStack_4._1_3_,0x17);
    pbVar4 = std::operator<<((basic_ostream<> *)*puVar3,s_3de__cant_find_file__100362bc);
    cVar8 = '\0';
    pbVar4 = std::operator<<(pbVar4,param_1);
    std::operator<<(pbVar4,cVar8);
    iVar2 = 0x41b;
    pcVar7 = s_E__work_ht_3de_3de_cpp_100362a4;
    pcVar5 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(s__conf_file_fail___100362d4,pcVar5,pcVar7,iVar2);
  }
  uStack_4 = 0x16;
  if (((uint)local_e8 & 1) != 0) {
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&pbStack_e4);
  }
  Housemarque::Template_Library::operator>>((basic_istream<> *)&pcStack_9c,(Config_Base *)this);
  pbStack_e4 = (basic_string<> *)abStack_40;
  *(code **)(abStack_98 + *(int *)(pcStack_9c + 4) + -4) = _vftable__exref;
  uStack_4._0_1_ = 0x18;
  std::basic_filebuf<>::~basic_filebuf<>((basic_filebuf<> *)abStack_94);
  uStack_4 = CONCAT31(uStack_4._1_3_,0xd);
  *(code **)(abStack_98 + *(int *)(pcStack_9c + 4) + -4) = _vftable__exref;
  std::basic_ios<>::~basic_ios<>(abStack_40);
  ExceptionList = pvStack_c;
  return this;
}



// public: __thiscall Housemarque::Threedee_Engine::Trimesh_Config::Trimesh_Config(class
// std::basic_istream<char,struct std::char_traits<char> > &)

Trimesh_Config * __thiscall
Housemarque::Threedee_Engine::Trimesh_Config::Trimesh_Config
          (Trimesh_Config *this,basic_istream<> *param_1)

{
  basic_string<> *pbVar1;
  char cVar2;
  uint uVar3;
  char *pcVar4;
  undefined uStack_59;
  basic_string<> *pbStack_58;
  Trimesh_Config *local_54;
  basic_string<> *pbStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  basic_string<> abStack_3c [16];
  basic_string<> abStack_2c [16];
  basic_string<> abStack_1c [16];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0xea40  23
                    // ??0Trimesh_Config@Threedee_Engine@Housemarque@@QAE@AAV?$basic_istream@DU?$char_traits@D@std@@@std@@@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10022a65;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_54 = this;
  Housemarque::Template_Library::Config_Base::Config_Base((Config_Base *)this);
  uStack_4 = 0;
  abStack_3c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_not_defined_100362e8;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_not_defined_100362e8,~uVar3 - 1);
  uStack_4c = CONCAT31(uStack_4c._1_3_,param_1._0_1_);
  uStack_4._0_1_ = 1;
  std::basic_string<>::_Tidy((basic_string<> *)&uStack_4c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_hx_file_100362f4;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign((basic_string<> *)&uStack_4c,s_hx_file_100362f4,~uVar3 - 1);
  pbVar1 = (basic_string<> *)(this + 0xc);
  uStack_4._0_1_ = 2;
  pbStack_58 = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_3c);
  uStack_4._0_1_ = 3;
  pbStack_50 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 4;
  if (pbStack_50 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_50,(basic_string<> *)&uStack_4c,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 7;
  std::basic_string<>::_Tidy((basic_string<> *)&uStack_4c,true);
  uStack_4._0_1_ = 6;
  std::basic_string<>::_Tidy(abStack_3c,true);
  abStack_1c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_1c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_not_defined_100362fc;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_1c,s_not_defined_100362fc,~uVar3 - 1);
  uStack_4._0_1_ = 8;
  abStack_2c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_2c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_hx_mesh_name_space_model_10036308;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_2c,s_hx_mesh_name_space_model_10036308,~uVar3 - 1);
  pbVar1 = (basic_string<> *)(this + 0x3c);
  uStack_4._0_1_ = 9;
  pbStack_50 = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_1c);
  uStack_4._0_1_ = 10;
  pbStack_58 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0xb;
  if (pbStack_58 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_58,abStack_2c,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0xe;
  std::basic_string<>::_Tidy(abStack_2c,true);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_1c,true);
  abStack_3c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_minimum_visible_size_in_pixels_10036324;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_minimum_visible_size_in_pixels_10036324,~uVar3 - 1);
  uStack_4._0_1_ = 0xf;
  pbStack_58 = (basic_string<> *)0x0;
  FUN_10014500(this + 0xec,(Config_Base *)this,abStack_3c,&pbStack_58);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_3c,true);
  abStack_3c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_align_to_camera_10036344;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_align_to_camera_10036344,~uVar3 - 1);
  uStack_4._0_1_ = 0x10;
  uStack_59 = 0;
  FUN_10014460(this + 0xf0,(Config_Base *)this,abStack_3c,&uStack_59);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_3c,true);
  abStack_3c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_scale_10036354;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_scale_10036354,~uVar3 - 1);
  uStack_4c = 0x3c23d70a;
  *(undefined4 *)(this + 0xf4) = 0x3c23d70a;
  uStack_48 = 0x3c23d70a;
  uStack_44 = 0x3c23d70a;
  *(undefined4 *)(this + 0xf8) = 0x3c23d70a;
  uStack_4._0_1_ = 0x11;
  *(undefined4 *)(this + 0xfc) = 0x3c23d70a;
  pbStack_50 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0x12;
  if (pbStack_50 != (basic_string<> *)0x0) {
    FUN_100155a0(pbStack_50,abStack_3c,(Config_Base *)this,this + 0xf4);
  }
  uStack_4 = CONCAT31(uStack_4._1_3_,0xd);
  std::basic_string<>::_Tidy(abStack_3c,true);
  FUN_1000daa0((Config_Base *)this,(int)(this + 0x1c),(int)(this + 0xcc),(int)(this + 0x4c),
               (int)(this + 0x6c),(int)(this + 0x8c),(int)(this + 0xac));
  Housemarque::Template_Library::operator>>(param_1,(Config_Base *)this);
  ExceptionList = pvStack_c;
  return this;
}



// public: __thiscall Housemarque::Threedee_Engine::Trimesh_Config::~Trimesh_Config(void)

void __thiscall Housemarque::Threedee_Engine::Trimesh_Config::~Trimesh_Config(Trimesh_Config *this)

{
  char *pcVar1;
  char cVar2;
  void *pvVar3;
  Trimesh_Config *pTVar4;
  int local_4;
  
                    // 0xedd0  43  ??1Trimesh_Config@Threedee_Engine@Housemarque@@QAE@XZ
  pTVar4 = this + 0xcc;
  local_4 = 8;
  do {
    pvVar3 = *(void **)(pTVar4 + -0xb0);
    if (pvVar3 != (void *)0x0) {
      if (*(int *)((int)pvVar3 + 4) != 0) {
        pcVar1 = (char *)(*(int *)((int)pvVar3 + 4) + -1);
        cVar2 = *pcVar1;
        if ((cVar2 == '\0') || (cVar2 == -1)) {
          FUN_100212a4(pcVar1);
        }
        else {
          *pcVar1 = cVar2 + -1;
        }
      }
      *(undefined4 *)((int)pvVar3 + 4) = 0;
      *(undefined4 *)((int)pvVar3 + 8) = 0;
      *(undefined4 *)((int)pvVar3 + 0xc) = 0;
      FUN_100212a4(pvVar3);
    }
    FUN_100212a4(*(void **)pTVar4);
    FUN_100212a4(*(void **)(pTVar4 + -0x80));
    FUN_100212a4(*(void **)(pTVar4 + -0x60));
    FUN_100212a4(*(void **)(pTVar4 + -0x40));
    FUN_100212a4(*(void **)(pTVar4 + -0x20));
    pTVar4 = pTVar4 + 4;
    local_4 = local_4 + -1;
  } while (local_4 != 0);
  if (*(int *)(this + 0x40) != 0) {
    pcVar1 = (char *)(*(int *)(this + 0x40) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(this + 0x40) = 0;
  *(undefined4 *)(this + 0x44) = 0;
  *(undefined4 *)(this + 0x48) = 0;
  if (*(int *)(this + 0x10) != 0) {
    pcVar1 = (char *)(*(int *)(this + 0x10) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(this + 0x10) = 0;
  *(undefined4 *)(this + 0x14) = 0;
  *(undefined4 *)(this + 0x18) = 0;
  Housemarque::Template_Library::Config_Base::~Config_Base((Config_Base *)this);
  return;
}



void FUN_1000eed0(void)

{
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Trimesh_Config::Dump_Statistics(class
// std::basic_ostream<char,struct std::char_traits<char> > &)

void __fastcall
Housemarque::Threedee_Engine::Trimesh_Config::Dump_Statistics(basic_ostream<> *param_1)

{
  int *piVar1;
  int *piVar2;
  bool bVar3;
  code *pcVar4;
  basic_ostream<> *pbVar5;
  srMeshModel *psVar6;
  list<> *plVar7;
  basic_ostream<> *in_EDX;
  int iVar8;
  code *pcVar9;
  uint uVar10;
  int *piVar11;
  char *pcVar12;
  undefined local_b4 [20];
  char local_a0;
  char local_9f;
  srGFInterface local_54 [44];
  reOrderData local_28 [4];
  int *local_24;
  int local_20;
  basic_ostream<> *local_18;
  undefined *local_14;
  void *local_10;
  undefined *puStack_c;
  undefined4 local_8;
  
                    // 0xeee0  66
                    // ?Dump_Statistics@Trimesh_Config@Threedee_Engine@Housemarque@@QAIXAAV?$basic_ostream@DU?$char_traits@D@std@@@std@@@Z
  puStack_c = &LAB_10022a7f;
  local_10 = ExceptionList;
  local_14 = &stack0xffffff40;
  local_8 = 0;
  iVar8 = 3;
  do {
    iVar8 = iVar8 + -1;
  } while (-1 < iVar8);
  ExceptionList = &local_10;
  local_18 = in_EDX;
  local_24 = (int *)operator_new(0xc);
  *local_24 = (int)local_24;
  local_24[1] = (int)local_24;
  local_20 = 0;
  local_8._0_1_ = 1;
  pcVar4 = *(code **)(param_1 + 0x10);
  if (*(code **)(param_1 + 0x10) == (code *)0x0) {
    pcVar4 = _C_exref;
  }
  srGFInterface::srGFInterface(local_54,(char *)pcVar4);
  pcVar4 = operator<<_exref;
  local_8 = CONCAT31(local_8._1_3_,2);
  pcVar9 = _C_exref;
  if (*(code **)(param_1 + 0x10) != (code *)0x0) {
    pcVar9 = *(code **)(param_1 + 0x10);
  }
  pbVar5 = std::operator<<(in_EDX,s_Trimesh_config_statistics_File__10036360);
  pbVar5 = std::operator<<(pbVar5,(char *)pcVar9);
  std::operator<<(pbVar5,&DAT_1003635c);
  for (uVar10 = 0; uVar10 < 8; uVar10 = uVar10 + 1) {
    bVar3 = std::operator!=(*(basic_string<> **)(param_1 + uVar10 * 4 + 0x1c),s_not_defined_10036384
                           );
    if (bVar3) {
      pcVar4 = _C_exref;
      if (*(code **)(*(int *)(param_1 + uVar10 * 4 + 0x1c) + 4) != (code *)0x0) {
        pcVar4 = *(code **)(*(int *)(param_1 + uVar10 * 4 + 0x1c) + 4);
      }
      pbVar5 = std::operator<<(local_18,s_Model__10036394);
      pbVar5 = std::operator<<(pbVar5,(char *)pcVar4);
      std::operator<<(pbVar5,&DAT_10036390);
      plVar7 = *(list<> **)(*(int *)(param_1 + uVar10 * 4 + 0x1c) + 4);
      if (*(list<> **)(*(int *)(param_1 + uVar10 * 4 + 0x1c) + 4) == (list<> *)0x0) {
        plVar7 = (list<> *)_C_exref;
      }
      psVar6 = srGFInterface::getModelByName
                         ((char *)local_54,plVar7,local_28,(Import_Statistics *)0x0,
                          SUB41(local_b4,0));
      srClass::release((srClass *)psVar6);
      FUN_10003dd0(local_b4,local_18);
      pcVar4 = operator<<_exref;
    }
  }
  bVar3 = std::operator!=((basic_string<> *)(param_1 + 0x3c),s_not_defined_100363a0);
  if (bVar3) {
    pcVar4 = _C_exref;
    if (*(code **)(param_1 + 0x40) != (code *)0x0) {
      pcVar4 = *(code **)(param_1 + 0x40);
    }
    pbVar5 = std::operator<<(local_18,s_Space_model__100363b0);
    pbVar5 = std::operator<<(pbVar5,(char *)pcVar4);
    std::operator<<(pbVar5,&DAT_100363ac);
    plVar7 = *(list<> **)(param_1 + 0x40);
    if (*(list<> **)(param_1 + 0x40) == (list<> *)0x0) {
      plVar7 = (list<> *)_C_exref;
    }
    psVar6 = srGFInterface::getModelByName
                       ((char *)local_54,plVar7,local_28,(Import_Statistics *)0x0,SUB41(local_b4,0))
    ;
    srClass::release((srClass *)psVar6);
    FUN_10003dd0(local_b4,local_18);
    if (local_a0 == '\0') {
      pcVar12 = s_WARNING__Space_model_is_not_soli_100363c0;
    }
    else {
      pcVar12 = s_Space_model_OK_100363e4;
    }
    std::operator<<(local_18,pcVar12);
    if (local_9f == '\0') {
      std::operator<<(local_18,s_WARNING_Space_model_base_is_not_r_100363f8);
    }
  }
  else {
    (*pcVar4)(local_18,s_No_space_model_10036444);
  }
  local_8 = CONCAT31(local_8._1_3_,1);
  srGFInterface::~srGFInterface(local_54);
  piVar2 = local_24;
  piVar11 = (int *)*local_24;
  while (piVar11 != piVar2) {
    piVar1 = (int *)*piVar11;
    *(int *)piVar11[1] = *piVar11;
    *(int *)(*piVar11 + 4) = piVar11[1];
    FUN_100212a4(piVar11);
    local_20 = local_20 + -1;
    piVar11 = piVar1;
  }
  FUN_100212a4(local_24);
  ExceptionList = local_10;
  return;
}



undefined4 Catch_1000f15d(void)

{
  basic_ostream<> *pbVar1;
  int unaff_EBP;
  char *pcVar2;
  
  pcVar2 = &DAT_10036458;
  pbVar1 = std::operator<<(*(basic_ostream<> **)(unaff_EBP + -0x14),s_ERROR__1003645c);
  pbVar1 = Housemarque::Kernel::operator<<(pbVar1,*(Error **)(unaff_EBP + -0x28));
  std::operator<<(pbVar1,pcVar2);
  return 0x1000f14c;
}



// public: __thiscall Housemarque::Threedee_Engine::Material_Config::Material_Config(char const *)

Material_Config * __thiscall
Housemarque::Threedee_Engine::Material_Config::Material_Config(Material_Config *this,char *param_1)

{
  basic_string<> *pbVar1;
  bool bVar2;
  undefined4 *puVar3;
  basic_ostream<> *pbVar4;
  char *pcVar5;
  uint uVar6;
  char *pcVar7;
  char cVar8;
  int iVar9;
  basic_string<> bStack_f1;
  basic_string<> *pbStack_f0;
  basic_string<> abStack_ec [16];
  basic_string<> abStack_dc [16];
  basic_string<> *apbStack_cc [2];
  basic_string<> abStack_c4 [16];
  basic_string<> abStack_b4 [16];
  uint local_a4;
  Material_Config *local_a0;
  int iStack_9c;
  byte abStack_98 [88];
  code *apcStack_40 [13];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0xf190  11  ??0Material_Config@Threedee_Engine@Housemarque@@QAE@PBD@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10022d5a;
  pvStack_c = ExceptionList;
  local_a4 = 0;
  ExceptionList = &pvStack_c;
  local_a0 = this;
  Housemarque::Template_Library::Config_Base::Config_Base((Config_Base *)this);
  uStack_4 = 0;
  abStack_ec[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_ec,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_not_defined_1003646c;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_ec,s_not_defined_1003646c,~uVar6 - 1);
  abStack_b4[0] = bStack_f1;
  uStack_4._0_1_ = 1;
  std::basic_string<>::_Tidy(abStack_b4,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_material_name_10036478;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_b4,s_material_name_10036478,~uVar6 - 1);
  pbVar1 = (basic_string<> *)(this + 0xc);
  uStack_4._0_1_ = 2;
  pbStack_f0 = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_ec);
  uStack_4._0_1_ = 3;
  apbStack_cc[0] = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 4;
  if (apbStack_cc[0] != (basic_string<> *)0x0) {
    FUN_10014030(apbStack_cc[0],abStack_b4,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 7;
  std::basic_string<>::_Tidy(abStack_b4,true);
  uStack_4._0_1_ = 6;
  std::basic_string<>::_Tidy(abStack_ec,true);
  abStack_dc[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_dc,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_not_defined_10036488;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_dc,s_not_defined_10036488,~uVar6 - 1);
  uStack_4._0_1_ = 8;
  abStack_c4[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_c4,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_tex_format_10036494;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_c4,s_tex_format_10036494,~uVar6 - 1);
  pbVar1 = (basic_string<> *)(this + 0x1c);
  uStack_4._0_1_ = 9;
  apbStack_cc[0] = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_dc);
  uStack_4._0_1_ = 10;
  pbStack_f0 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0xb;
  if (pbStack_f0 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_f0,abStack_c4,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0xe;
  std::basic_string<>::_Tidy(abStack_c4,true);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_dc,true);
  abStack_ec[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_ec,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_tex_width_100364a0;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_ec,s_tex_width_100364a0,~uVar6 - 1);
  uStack_4._0_1_ = 0xf;
  pbStack_f0 = (basic_string<> *)0x0;
  FUN_10014500(this + 0x2c,(Config_Base *)this,abStack_ec,&pbStack_f0);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_ec,true);
  abStack_ec[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_ec,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_tex_height_100364ac;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_ec,s_tex_height_100364ac,~uVar6 - 1);
  uStack_4._0_1_ = 0x10;
  pbStack_f0 = (basic_string<> *)0x0;
  FUN_10014500(this + 0x30,(Config_Base *)this,abStack_ec,&pbStack_f0);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_ec,true);
  abStack_ec[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_ec,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_tex_size_critical_100364b8;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_ec,s_tex_size_critical_100364b8,~uVar6 - 1);
  uStack_4._0_1_ = 0x11;
  bStack_f1 = (basic_string<>)0x0;
  FUN_10014460(this + 0x34,(Config_Base *)this,abStack_ec,&bStack_f1);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_ec,true);
  abStack_ec[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_ec,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_material_mipmap_100364cc;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_ec,s_material_mipmap_100364cc,~uVar6 - 1);
  uStack_4._0_1_ = 0x12;
  bStack_f1 = (basic_string<>)0x1;
  FUN_10014460(this + 0x35,(Config_Base *)this,abStack_ec,&bStack_f1);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_ec,true);
  abStack_b4[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_b4,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_normal_100364dc;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_b4,s_normal_100364dc,~uVar6 - 1);
  uStack_4._0_1_ = 0x13;
  abStack_ec[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_ec,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_material_type_100364e4;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_ec,s_material_type_100364e4,~uVar6 - 1);
  uStack_4._0_1_ = 0x14;
  pbVar1 = (basic_string<> *)(this + 0x38);
  apbStack_cc[0] = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_b4);
  uStack_4._0_1_ = 0x15;
  pbStack_f0 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0x16;
  if (pbStack_f0 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_f0,abStack_ec,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0x19;
  std::basic_string<>::_Tidy(abStack_ec,true);
  uStack_4._0_1_ = 0x18;
  std::basic_string<>::_Tidy(abStack_b4,true);
  abStack_c4[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_c4,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_not_defined_100364f4;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_c4,s_not_defined_100364f4,~uVar6 - 1);
  uStack_4._0_1_ = 0x1a;
  abStack_dc[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_dc,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_texture_10036500;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_dc,s_texture_10036500,~uVar6 - 1);
  pbVar1 = (basic_string<> *)(this + 0x48);
  uStack_4._0_1_ = 0x1b;
  apbStack_cc[0] = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_c4);
  uStack_4._0_1_ = 0x1c;
  pbStack_f0 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0x1d;
  if (pbStack_f0 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_f0,abStack_dc,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0x20;
  std::basic_string<>::_Tidy(abStack_dc,true);
  uStack_4._0_1_ = 0x1f;
  std::basic_string<>::_Tidy(abStack_c4,true);
  abStack_b4[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_b4,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_not_defined_10036508;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_b4,s_not_defined_10036508,~uVar6 - 1);
  abStack_ec[0] = bStack_f1;
  uStack_4._0_1_ = 0x21;
  std::basic_string<>::_Tidy(abStack_ec,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_envmap_texture_10036514;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_ec,s_envmap_texture_10036514,~uVar6 - 1);
  pbVar1 = (basic_string<> *)(this + 0x58);
  uStack_4._0_1_ = 0x22;
  apbStack_cc[0] = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_b4);
  uStack_4._0_1_ = 0x23;
  pbStack_f0 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0x24;
  if (pbStack_f0 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_f0,abStack_ec,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0x27;
  std::basic_string<>::_Tidy(abStack_ec,true);
  uStack_4._0_1_ = 0x26;
  std::basic_string<>::_Tidy(abStack_b4,true);
  abStack_c4[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_c4,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_not_defined_10036524;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_c4,s_not_defined_10036524,~uVar6 - 1);
  uStack_4._0_1_ = 0x28;
  abStack_dc[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_dc,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_envmap_tex_format_10036530;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_dc,s_envmap_tex_format_10036530,~uVar6 - 1);
  pbVar1 = (basic_string<> *)(this + 0x68);
  uStack_4._0_1_ = 0x29;
  apbStack_cc[0] = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_c4);
  uStack_4._0_1_ = 0x2a;
  pbStack_f0 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0x2b;
  if (pbStack_f0 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_f0,abStack_dc,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0x2e;
  std::basic_string<>::_Tidy(abStack_dc,true);
  uStack_4._0_1_ = 0x2d;
  std::basic_string<>::_Tidy(abStack_c4,true);
  abStack_ec[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_ec,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_envmap_tex_width_10036544;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_ec,s_envmap_tex_width_10036544,~uVar6 - 1);
  uStack_4._0_1_ = 0x2f;
  pbStack_f0 = (basic_string<> *)0x0;
  FUN_10014500(this + 0x78,(Config_Base *)this,abStack_ec,&pbStack_f0);
  uStack_4._0_1_ = 0x2d;
  std::basic_string<>::_Tidy(abStack_ec,true);
  abStack_ec[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_ec,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_envmap_tex_height_10036558;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_ec,s_envmap_tex_height_10036558,~uVar6 - 1);
  uStack_4._0_1_ = 0x30;
  pbStack_f0 = (basic_string<> *)0x0;
  FUN_10014500(this + 0x7c,(Config_Base *)this,abStack_ec,&pbStack_f0);
  uStack_4._0_1_ = 0x2d;
  std::basic_string<>::_Tidy(abStack_ec,true);
  abStack_ec[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_ec,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_envmap_tex_size_critical_1003656c;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_ec,s_envmap_tex_size_critical_1003656c,~uVar6 - 1);
  uStack_4._0_1_ = 0x31;
  bStack_f1 = (basic_string<>)0x0;
  FUN_10014460(this + 0x80,(Config_Base *)this,abStack_ec,&bStack_f1);
  uStack_4._0_1_ = 0x2d;
  std::basic_string<>::_Tidy(abStack_ec,true);
  abStack_ec[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_ec,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_overlay_10036588;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_ec,s_overlay_10036588,~uVar6 - 1);
  uStack_4._0_1_ = 0x32;
  bStack_f1 = (basic_string<>)0x0;
  FUN_10014460(this + 0x81,(Config_Base *)this,abStack_ec,&bStack_f1);
  uStack_4._0_1_ = 0x2d;
  std::basic_string<>::_Tidy(abStack_ec,true);
  abStack_ec[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_ec,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_alpha_test_10036590;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_ec,s_alpha_test_10036590,~uVar6 - 1);
  uStack_4._0_1_ = 0x33;
  bStack_f1 = (basic_string<>)0x0;
  FUN_10014460(this + 0x82,(Config_Base *)this,abStack_ec,&bStack_f1);
  uStack_4._0_1_ = 0x2d;
  std::basic_string<>::_Tidy(abStack_ec,true);
  abStack_b4[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_b4,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_not_defined_1003659c;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_b4,s_not_defined_1003659c,~uVar6 - 1);
  uStack_4._0_1_ = 0x34;
  abStack_ec[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_ec,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_additive_texture_100365a8;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_ec,s_additive_texture_100365a8,~uVar6 - 1);
  pbVar1 = (basic_string<> *)(this + 0x84);
  uStack_4._0_1_ = 0x35;
  apbStack_cc[0] = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_b4);
  uStack_4._0_1_ = 0x36;
  pbStack_f0 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0x37;
  if (pbStack_f0 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_f0,abStack_ec,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0x3a;
  std::basic_string<>::_Tidy(abStack_ec,true);
  uStack_4._0_1_ = 0x39;
  std::basic_string<>::_Tidy(abStack_b4,true);
  abStack_dc[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_dc,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_additive_envmap_100365bc;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_dc,s_additive_envmap_100365bc,~uVar6 - 1);
  uStack_4._0_1_ = 0x3b;
  bStack_f1 = (basic_string<>)0x0;
  FUN_10014460(this + 0x94,(Config_Base *)this,abStack_dc,&bStack_f1);
  uStack_4._0_1_ = 0x39;
  std::basic_string<>::_Tidy(abStack_dc,true);
  abStack_c4[0] = bStack_f1;
  std::basic_string<>::_Tidy(abStack_c4,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_not_defined_100365cc;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_c4,s_not_defined_100365cc,~uVar6 - 1);
  abStack_dc[0] = bStack_f1;
  uStack_4._0_1_ = 0x3c;
  std::basic_string<>::_Tidy(abStack_dc,false);
  uVar6 = 0xffffffff;
  pcVar5 = s_transparent_texture_100365d8;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar8 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar8 != '\0');
  std::basic_string<>::assign(abStack_dc,s_transparent_texture_100365d8,~uVar6 - 1);
  pbVar1 = (basic_string<> *)(this + 0x98);
  uStack_4._0_1_ = 0x3d;
  apbStack_cc[0] = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_c4);
  uStack_4._0_1_ = 0x3e;
  pbStack_f0 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0x3f;
  if (pbStack_f0 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_f0,abStack_dc,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0x42;
  std::basic_string<>::_Tidy(abStack_dc,true);
  uStack_4._0_1_ = 0x41;
  std::basic_string<>::_Tidy(abStack_c4,true);
  std::basic_ifstream<>::basic_ifstream<>((basic_ifstream<> *)&iStack_9c,param_1,1);
  uStack_4._0_1_ = 0x43;
  if ((abStack_98[*(int *)(iStack_9c + 4)] & 6) != 0) {
    local_a4 = 1;
    puVar3 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&pbStack_f0);
    uStack_4 = CONCAT31(uStack_4._1_3_,0x44);
    pbVar4 = std::operator<<((basic_ostream<> *)*puVar3,s_3de__cant_find_file__10036604);
    cVar8 = '\0';
    pbVar4 = std::operator<<(pbVar4,param_1);
    std::operator<<(pbVar4,cVar8);
    iVar9 = 0x49d;
    pcVar7 = s_E__work_ht_3de_3de_cpp_100365ec;
    pcVar5 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(s__conf_file_fail___1003661c,pcVar5,pcVar7,iVar9);
  }
  uStack_4 = 0x43;
  uVar6 = local_a4;
  if ((local_a4 & 1) != 0) {
    uVar6 = local_a4 & 0xfffffffe;
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&pbStack_f0);
  }
  bVar2 = Housemarque::Kernel::Logging_Enabled();
  if (bVar2) {
    uVar6 = uVar6 | 2;
    local_a4 = uVar6;
    iVar9 = Housemarque::Kernel::Error_Log(apbStack_cc);
    uStack_4 = CONCAT31(uStack_4._1_3_,0x45);
    pbVar4 = std::operator<<(*(basic_ostream<> **)(iVar9 + 4),s_filename__10036634);
    pcVar5 = &DAT_10036630;
    pbVar4 = std::operator<<(pbVar4,param_1);
    std::operator<<(pbVar4,pcVar5);
  }
  uStack_4 = 0x43;
  if ((uVar6 & 2) != 0) {
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter((Error_Log_Inserter *)apbStack_cc);
  }
  Housemarque::Template_Library::operator>>((basic_istream<> *)&iStack_9c,(Config_Base *)this);
  uStack_4 = CONCAT31(uStack_4._1_3_,0x41);
  std::basic_ifstream<>::~basic_ifstream<>((basic_ifstream<> *)apcStack_40);
  apcStack_40[0] = _vftable__exref;
  std::ios_base::~ios_base((ios_base *)apcStack_40);
  ExceptionList = pvStack_c;
  return this;
}



// public: __thiscall Housemarque::Threedee_Engine::Material_Config::Material_Config(class
// std::basic_istream<char,struct std::char_traits<char> > &)

Material_Config * __thiscall
Housemarque::Threedee_Engine::Material_Config::Material_Config
          (Material_Config *this,basic_istream<> *param_1)

{
  basic_string<> *pbVar1;
  char cVar2;
  uint uVar3;
  char *pcVar4;
  allocator<char> aStack_5a;
  allocator<char> aStack_59;
  basic_string<> *pbStack_58;
  Material_Config *local_54;
  basic_string<> *pbStack_50;
  basic_string<> abStack_4c [16];
  basic_string<> abStack_3c [16];
  basic_string<> abStack_2c [16];
  basic_string<> abStack_1c [16];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0xfe30  10
                    // ??0Material_Config@Threedee_Engine@Housemarque@@QAE@AAV?$basic_istream@DU?$char_traits@D@std@@@std@@@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10022f40;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_54 = this;
  Housemarque::Template_Library::Config_Base::Config_Base((Config_Base *)this);
  uStack_4 = 0;
  abStack_3c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_not_defined_10036640;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_not_defined_10036640,~uVar3 - 1);
  abStack_4c[0] = param_1._0_1_;
  uStack_4._0_1_ = 1;
  std::basic_string<>::_Tidy(abStack_4c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_material_name_1003664c;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_4c,s_material_name_1003664c,~uVar3 - 1);
  pbVar1 = (basic_string<> *)(this + 0xc);
  uStack_4._0_1_ = 2;
  pbStack_58 = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_3c);
  uStack_4._0_1_ = 3;
  pbStack_50 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 4;
  if (pbStack_50 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_50,abStack_4c,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 7;
  std::basic_string<>::_Tidy(abStack_4c,true);
  uStack_4._0_1_ = 6;
  std::basic_string<>::_Tidy(abStack_3c,true);
  abStack_1c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_1c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_not_defined_1003665c;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_1c,s_not_defined_1003665c,~uVar3 - 1);
  uStack_4._0_1_ = 8;
  abStack_2c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_2c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_tex_format_10036668;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_2c,s_tex_format_10036668,~uVar3 - 1);
  pbVar1 = (basic_string<> *)(this + 0x1c);
  uStack_4._0_1_ = 9;
  pbStack_50 = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_1c);
  uStack_4._0_1_ = 10;
  pbStack_58 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0xb;
  if (pbStack_58 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_58,abStack_2c,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0xe;
  std::basic_string<>::_Tidy(abStack_2c,true);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_1c,true);
  abStack_3c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_tex_width_10036674;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_tex_width_10036674,~uVar3 - 1);
  uStack_4._0_1_ = 0xf;
  pbStack_58 = (basic_string<> *)0x0;
  FUN_10014500(this + 0x2c,(Config_Base *)this,abStack_3c,&pbStack_58);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_3c,true);
  abStack_3c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_tex_height_10036680;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_tex_height_10036680,~uVar3 - 1);
  uStack_4._0_1_ = 0x10;
  pbStack_58 = (basic_string<> *)0x0;
  FUN_10014500(this + 0x30,(Config_Base *)this,abStack_3c,&pbStack_58);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_3c,true);
  abStack_3c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_tex_size_critical_1003668c;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_tex_size_critical_1003668c,~uVar3 - 1);
  uStack_4._0_1_ = 0x11;
  aStack_5a = (allocator<char>)0x0;
  FUN_10014460(this + 0x34,(Config_Base *)this,abStack_3c,&aStack_5a);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_3c,true);
  abStack_3c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_material_mipmap_100366a0;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_material_mipmap_100366a0,~uVar3 - 1);
  uStack_4._0_1_ = 0x12;
  aStack_5a = (allocator<char>)0x1;
  FUN_10014460(this + 0x35,(Config_Base *)this,abStack_3c,&aStack_5a);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_3c,true);
  abStack_4c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_4c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_normal_100366b0;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_4c,s_normal_100366b0,~uVar3 - 1);
  uStack_4._0_1_ = 0x13;
  abStack_3c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_material_type_100366b8;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_material_type_100366b8,~uVar3 - 1);
  uStack_4._0_1_ = 0x14;
  pbVar1 = (basic_string<> *)(this + 0x38);
  pbStack_50 = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_4c);
  uStack_4._0_1_ = 0x15;
  pbStack_58 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0x16;
  if (pbStack_58 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_58,abStack_3c,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0x19;
  std::basic_string<>::_Tidy(abStack_3c,true);
  uStack_4._0_1_ = 0x18;
  std::basic_string<>::_Tidy(abStack_4c,true);
  abStack_2c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_2c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_not_defined_100366c8;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_2c,s_not_defined_100366c8,~uVar3 - 1);
  uStack_4._0_1_ = 0x1a;
  abStack_1c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_1c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_texture_100366d4;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_1c,s_texture_100366d4,~uVar3 - 1);
  pbVar1 = (basic_string<> *)(this + 0x48);
  uStack_4._0_1_ = 0x1b;
  pbStack_50 = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_2c);
  uStack_4._0_1_ = 0x1c;
  pbStack_58 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0x1d;
  if (pbStack_58 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_58,abStack_1c,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0x20;
  std::basic_string<>::_Tidy(abStack_1c,true);
  uStack_4._0_1_ = 0x1f;
  std::basic_string<>::_Tidy(abStack_2c,true);
  abStack_4c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_4c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_not_defined_100366dc;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_4c,s_not_defined_100366dc,~uVar3 - 1);
  abStack_3c[0] = param_1._0_1_;
  uStack_4._0_1_ = 0x21;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_envmap_texture_100366e8;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_envmap_texture_100366e8,~uVar3 - 1);
  pbVar1 = (basic_string<> *)(this + 0x58);
  uStack_4._0_1_ = 0x22;
  pbStack_50 = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_4c);
  uStack_4._0_1_ = 0x23;
  pbStack_58 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0x24;
  if (pbStack_58 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_58,abStack_3c,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0x27;
  std::basic_string<>::_Tidy(abStack_3c,true);
  uStack_4._0_1_ = 0x26;
  std::basic_string<>::_Tidy(abStack_4c,true);
  abStack_2c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_2c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_not_defined_100366f8;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_2c,s_not_defined_100366f8,~uVar3 - 1);
  uStack_4._0_1_ = 0x28;
  abStack_1c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_1c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_envmap_tex_format_10036704;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_1c,s_envmap_tex_format_10036704,~uVar3 - 1);
  pbVar1 = (basic_string<> *)(this + 0x68);
  uStack_4._0_1_ = 0x29;
  pbStack_50 = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_2c);
  uStack_4._0_1_ = 0x2a;
  pbStack_58 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0x2b;
  if (pbStack_58 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_58,abStack_1c,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0x2e;
  std::basic_string<>::_Tidy(abStack_1c,true);
  uStack_4._0_1_ = 0x2d;
  std::basic_string<>::_Tidy(abStack_2c,true);
  abStack_3c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_envmap_tex_width_10036718;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_envmap_tex_width_10036718,~uVar3 - 1);
  uStack_4._0_1_ = 0x2f;
  pbStack_58 = (basic_string<> *)0x0;
  FUN_10014500(this + 0x78,(Config_Base *)this,abStack_3c,&pbStack_58);
  uStack_4._0_1_ = 0x2d;
  std::basic_string<>::_Tidy(abStack_3c,true);
  abStack_3c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_envmap_tex_height_1003672c;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_envmap_tex_height_1003672c,~uVar3 - 1);
  uStack_4._0_1_ = 0x30;
  pbStack_58 = (basic_string<> *)0x0;
  FUN_10014500(this + 0x7c,(Config_Base *)this,abStack_3c,&pbStack_58);
  uStack_4._0_1_ = 0x2d;
  std::basic_string<>::_Tidy(abStack_3c,true);
  abStack_3c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_envmap_tex_size_critical_10036740;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_envmap_tex_size_critical_10036740,~uVar3 - 1);
  uStack_4._0_1_ = 0x31;
  aStack_5a = (allocator<char>)0x0;
  FUN_10014460(this + 0x80,(Config_Base *)this,abStack_3c,&aStack_5a);
  uStack_4._0_1_ = 0x2d;
  std::basic_string<>::_Tidy(abStack_3c,true);
  abStack_3c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_overlay_1003675c;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_overlay_1003675c,~uVar3 - 1);
  uStack_4._0_1_ = 0x32;
  aStack_5a = (allocator<char>)0x0;
  FUN_10014460(this + 0x81,(Config_Base *)this,abStack_3c,&aStack_5a);
  uStack_4._0_1_ = 0x2d;
  std::basic_string<>::_Tidy(abStack_3c,true);
  abStack_3c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_alpha_test_10036764;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_alpha_test_10036764,~uVar3 - 1);
  uStack_4._0_1_ = 0x33;
  aStack_5a = (allocator<char>)0x0;
  FUN_10014460(this + 0x82,(Config_Base *)this,abStack_3c,&aStack_5a);
  uStack_4._0_1_ = 0x2d;
  std::basic_string<>::_Tidy(abStack_3c,true);
  abStack_4c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_4c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_not_defined_10036770;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_4c,s_not_defined_10036770,~uVar3 - 1);
  uStack_4._0_1_ = 0x34;
  abStack_3c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_3c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_additive_texture_1003677c;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_3c,s_additive_texture_1003677c,~uVar3 - 1);
  pbVar1 = (basic_string<> *)(this + 0x84);
  uStack_4._0_1_ = 0x35;
  pbStack_50 = pbVar1;
  std::basic_string<>::basic_string<>(pbVar1,abStack_4c);
  uStack_4._0_1_ = 0x36;
  pbStack_58 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 0x37;
  if (pbStack_58 != (basic_string<> *)0x0) {
    FUN_10014030(pbStack_58,abStack_3c,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0x3a;
  std::basic_string<>::_Tidy(abStack_3c,true);
  uStack_4._0_1_ = 0x39;
  std::basic_string<>::_Tidy(abStack_4c,true);
  abStack_1c[0] = param_1._0_1_;
  std::basic_string<>::_Tidy(abStack_1c,false);
  uVar3 = 0xffffffff;
  pcVar4 = s_additive_envmap_10036790;
  do {
    if (uVar3 == 0) break;
    uVar3 = uVar3 - 1;
    cVar2 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar2 != '\0');
  std::basic_string<>::assign(abStack_1c,s_additive_envmap_10036790,~uVar3 - 1);
  uStack_4._0_1_ = 0x3b;
  FUN_10014330(this + 0x94,(Config_Base *)this,abStack_1c,0);
  uStack_4._0_1_ = 0x39;
  std::basic_string<>::_Tidy(abStack_1c,true);
  std::basic_string<>::basic_string<>(abStack_2c,s_not_defined_100367a0,&aStack_5a);
  uStack_4._0_1_ = 0x3c;
  std::basic_string<>::basic_string<>(abStack_1c,s_transparent_texture_100367ac,&aStack_59);
  uStack_4._0_1_ = 0x3d;
  FUN_10014640(this + 0x98,(Config_Base *)this,abStack_1c,abStack_2c);
  uStack_4._0_1_ = 0x40;
  std::basic_string<>::_Tidy(abStack_1c,true);
  uStack_4 = CONCAT31(uStack_4._1_3_,0x3f);
  std::basic_string<>::_Tidy(abStack_2c,true);
  Housemarque::Template_Library::operator>>(param_1,(Config_Base *)this);
  ExceptionList = pvStack_c;
  return this;
}



// public: __thiscall Housemarque::Threedee_Engine::Material_Config::~Material_Config(void)

void __thiscall
Housemarque::Threedee_Engine::Material_Config::~Material_Config(Material_Config *this)

{
  char *pcVar1;
  char cVar2;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0x10850  33  ??1Material_Config@Threedee_Engine@Housemarque@@QAE@XZ
  puStack_8 = &LAB_10022fa2;
  pvStack_c = ExceptionList;
  local_4 = 7;
  ExceptionList = &pvStack_c;
  std::basic_string<>::_Tidy((basic_string<> *)(this + 0x98),true);
  local_4._0_1_ = 6;
  std::basic_string<>::_Tidy((basic_string<> *)(this + 0x84),true);
  local_4 = CONCAT31(local_4._1_3_,5);
  std::basic_string<>::_Tidy((basic_string<> *)(this + 0x68),true);
  if (*(int *)(this + 0x5c) != 0) {
    pcVar1 = (char *)(*(int *)(this + 0x5c) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(this + 0x5c) = 0;
  *(undefined4 *)(this + 0x60) = 0;
  *(undefined4 *)(this + 100) = 0;
  if (*(int *)(this + 0x4c) != 0) {
    pcVar1 = (char *)(*(int *)(this + 0x4c) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(this + 0x4c) = 0;
  *(undefined4 *)(this + 0x50) = 0;
  *(undefined4 *)(this + 0x54) = 0;
  if (*(int *)(this + 0x3c) != 0) {
    pcVar1 = (char *)(*(int *)(this + 0x3c) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(this + 0x3c) = 0;
  *(undefined4 *)(this + 0x40) = 0;
  *(undefined4 *)(this + 0x44) = 0;
  if (*(int *)(this + 0x20) != 0) {
    pcVar1 = (char *)(*(int *)(this + 0x20) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(this + 0x20) = 0;
  *(undefined4 *)(this + 0x24) = 0;
  *(undefined4 *)(this + 0x28) = 0;
  if (*(int *)(this + 0x10) != 0) {
    pcVar1 = (char *)(*(int *)(this + 0x10) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(this + 0x10) = 0;
  *(undefined4 *)(this + 0x14) = 0;
  *(undefined4 *)(this + 0x18) = 0;
  local_4 = 0xffffffff;
  Housemarque::Template_Library::Config_Base::~Config_Base((Config_Base *)this);
  ExceptionList = pvStack_c;
  return;
}



// public: __thiscall Housemarque::Threedee_Engine::Trimesh::Trimesh(void)

Trimesh * __thiscall Housemarque::Threedee_Engine::Trimesh::Trimesh(Trimesh *this)

{
  srRegistry *this_00;
  srRegistry *this_01;
  srRegistry *this_02;
  ClassNode *pCVar1;
  char *pcVar2;
  ulong uVar3;
  int iVar4;
  Trimesh TStack_19;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0x109b0  22  ??0Trimesh@Threedee_Engine@Housemarque@@QAE@XZ
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10022fb5;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  srModelInstance::srModelInstance((srModelInstance *)this,(srNode *)0x0);
  *(undefined ***)this = &PTR_FUN_10025cbc;
  *(undefined ***)(this + 0x138) = &PTR_LAB_10025ca8;
  this_00 = *(srRegistry **)(srCore_exref + 0x2c);
  uStack_4 = 0;
  pCVar1 = srRegistry::getClassNode(this_00,0x38888);
  if (pCVar1 == (ClassNode *)0x0) {
    this_01 = *(srRegistry **)(srCore_exref + 0x2c);
    pCVar1 = srRegistry::getClassNode(this_01,0x1100);
    if (pCVar1 == (ClassNode *)0x0) {
      this_02 = *(srRegistry **)(srCore_exref + 0x2c);
      pCVar1 = srRegistry::getClassNode(this_02,0x1000);
      if (pCVar1 == (ClassNode *)0x0) {
        iVar4 = 1;
        uVar3 = 0x1000;
        pCVar1 = srClass::sGetClassNode();
        pcVar2 = srNode::sGetClassName();
        pCVar1 = srRegistry::registerClass(this_02,pcVar2,pCVar1,uVar3,iVar4);
      }
      pCVar1 = srRegistry::registerClass(this_01,s_srModelInstance_10037024,pCVar1,0x1100,0);
    }
    pCVar1 = srRegistry::registerClass(this_00,s_srModelInstance_10037024,pCVar1,0x38888,0);
  }
  srRegistry::registerInstance(this_00,pCVar1,(srRuntimeClass *)this);
  *(undefined4 *)(this + 0x164) = 0;
  this[0x168] = (Trimesh)0x0;
  this[0x16c] = TStack_19;
  *(undefined4 *)(this + 0x170) = 0;
  *(undefined4 *)(this + 0x174) = 0;
  *(undefined4 *)(this + 0x178) = 0;
  *(undefined4 *)(this + 0x17c) = 0;
  *(undefined ***)this = &PTR_FUN_10025c68;
  *(undefined ***)(this + 0x138) = &PTR_LAB_10025c54;
  ExceptionList = pvStack_c;
  return this;
}



// public: virtual __thiscall Housemarque::Threedee_Engine::Trimesh::~Trimesh(void)

void __thiscall Housemarque::Threedee_Engine::Trimesh::~Trimesh(Trimesh *this)

{
  srRegistry *this_00;
  srRegistry *this_01;
  srRegistry *this_02;
  ClassNode *pCVar1;
  char *pcVar2;
  undefined4 *puVar3;
  ulong uVar4;
  int iVar5;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0x10af0  42  ??1Trimesh@Threedee_Engine@Housemarque@@UAE@XZ
  puStack_8 = &LAB_10022fde;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined ***)this = &PTR_FUN_10025c68;
  *(undefined ***)(this + 0x138) = &PTR_LAB_10025c54;
  local_4 = 1;
  Remove();
  if (this[0x168] != (Trimesh)0x0) {
    srClass::release(*(srClass **)(this + 0x164));
  }
  puVar3 = *(undefined4 **)(this + 0x170);
  if (puVar3 != *(undefined4 **)(this + 0x174)) {
    do {
      srClass::release((srClass *)*puVar3);
      puVar3 = puVar3 + 1;
    } while (puVar3 != *(undefined4 **)(this + 0x174));
  }
  FUN_100212a4(*(void **)(this + 0x170));
  *(undefined4 *)(this + 0x170) = 0;
  *(undefined4 *)(this + 0x174) = 0;
  *(undefined4 *)(this + 0x178) = 0;
  *(undefined ***)this = &PTR_FUN_10025cbc;
  *(undefined ***)(this + 0x138) = &PTR_LAB_10025ca8;
  this_00 = *(srRegistry **)(srCore_exref + 0x2c);
  local_4 = 2;
  pCVar1 = srRegistry::getClassNode(this_00,0x38888);
  if (pCVar1 == (ClassNode *)0x0) {
    this_01 = *(srRegistry **)(srCore_exref + 0x2c);
    pCVar1 = srRegistry::getClassNode(this_01,0x1100);
    if (pCVar1 == (ClassNode *)0x0) {
      this_02 = *(srRegistry **)(srCore_exref + 0x2c);
      pCVar1 = srRegistry::getClassNode(this_02,0x1000);
      if (pCVar1 == (ClassNode *)0x0) {
        iVar5 = 1;
        uVar4 = 0x1000;
        pCVar1 = srClass::sGetClassNode();
        pcVar2 = srNode::sGetClassName();
        pCVar1 = srRegistry::registerClass(this_02,pcVar2,pCVar1,uVar4,iVar5);
      }
      pCVar1 = srRegistry::registerClass(this_01,s_srModelInstance_10037024,pCVar1,0x1100,0);
    }
    pCVar1 = srRegistry::registerClass(this_00,s_srModelInstance_10037024,pCVar1,0x38888,0);
  }
  srRegistry::unregisterInstance(this_00,pCVar1,(srRuntimeClass *)this);
  local_4 = 0xffffffff;
  srModelInstance::~srModelInstance((srModelInstance *)this);
  ExceptionList = pvStack_c;
  return;
}



undefined4 FUN_10010c70(void)

{
  Trimesh *this;
  undefined4 uVar1;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10022ff3;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  this = (Trimesh *)srHeap::allocate((srHeap *)srHeap_exref,0x180);
  uStack_4 = 0;
  if (this != (Trimesh *)0x0) {
    uVar1 = Housemarque::Threedee_Engine::Trimesh::Trimesh(this);
    ExceptionList = pvStack_c;
    return uVar1;
  }
  ExceptionList = pvStack_c;
  return 0;
}



void * __thiscall FUN_10010cd0(void *this,srModelInstance *param_1)

{
  undefined4 *puVar1;
  uint uVar2;
  int iVar3;
  undefined4 *puVar4;
  uint uVar5;
  undefined4 *puVar6;
  
  if ((srModelInstance *)this != param_1) {
    srModelInstance::operator=((srModelInstance *)this,param_1);
    *(undefined4 *)((int)this + 0x160) = *(undefined4 *)(param_1 + 0x160);
    *(undefined4 *)((int)this + 0x164) = *(undefined4 *)(param_1 + 0x164);
    *(srModelInstance *)((int)this + 0x168) = param_1[0x168];
    if ((srModelInstance *)((int)this + 0x16c) != param_1 + 0x16c) {
      puVar1 = *(undefined4 **)(param_1 + 0x170);
      if (puVar1 == (undefined4 *)0x0) {
        uVar5 = 0;
      }
      else {
        uVar5 = *(int *)(param_1 + 0x174) - (int)puVar1 >> 2;
      }
      puVar4 = *(undefined4 **)((int)this + 0x170);
      if (puVar4 == (undefined4 *)0x0) {
        uVar2 = 0;
      }
      else {
        uVar2 = *(int *)((int)this + 0x174) - (int)puVar4 >> 2;
      }
      if (uVar5 <= uVar2) {
        puVar6 = *(undefined4 **)(param_1 + 0x174);
        for (; puVar1 != puVar6; puVar1 = puVar1 + 1) {
          *puVar4 = *puVar1;
          puVar4 = puVar4 + 1;
        }
        if (*(int *)(param_1 + 0x170) == 0) {
          *(undefined4 *)((int)this + 0x174) = *(undefined4 *)((int)this + 0x170);
          return this;
        }
        *(int *)((int)this + 0x174) =
             *(int *)((int)this + 0x170) +
             (*(int *)(param_1 + 0x174) - *(int *)(param_1 + 0x170) >> 2) * 4;
        return this;
      }
      if (puVar1 == (undefined4 *)0x0) {
        uVar5 = 0;
      }
      else {
        uVar5 = *(int *)(param_1 + 0x174) - (int)puVar1 >> 2;
      }
      if (puVar4 == (undefined4 *)0x0) {
        uVar2 = 0;
      }
      else {
        uVar2 = *(int *)((int)this + 0x178) - (int)puVar4 >> 2;
      }
      if (uVar5 <= uVar2) {
        if (puVar4 == (undefined4 *)0x0) {
          iVar3 = 0;
        }
        else {
          iVar3 = *(int *)((int)this + 0x174) - (int)puVar4 >> 2;
        }
        puVar6 = puVar1 + iVar3;
        for (; puVar1 != puVar6; puVar1 = puVar1 + 1) {
          *puVar4 = *puVar1;
          puVar4 = puVar4 + 1;
        }
        puVar1 = *(undefined4 **)(param_1 + 0x174);
        puVar4 = *(undefined4 **)((int)this + 0x174);
        for (; puVar6 != puVar1; puVar6 = puVar6 + 1) {
          FUN_100166e0(puVar4,puVar6);
          puVar4 = puVar4 + 1;
        }
        if (*(int *)(param_1 + 0x170) == 0) {
          *(undefined4 *)((int)this + 0x174) = *(undefined4 *)((int)this + 0x170);
          return this;
        }
        *(int *)((int)this + 0x174) =
             *(int *)((int)this + 0x170) +
             (*(int *)(param_1 + 0x174) - *(int *)(param_1 + 0x170) >> 2) * 4;
        return this;
      }
      FUN_100212a4(puVar4);
      if (*(int *)(param_1 + 0x170) == 0) {
        iVar3 = 0;
      }
      else {
        iVar3 = *(int *)(param_1 + 0x174) - *(int *)(param_1 + 0x170) >> 2;
      }
      if (iVar3 < 0) {
        iVar3 = 0;
      }
      puVar1 = (undefined4 *)operator_new(iVar3 * 4);
      *(undefined4 **)((int)this + 0x170) = puVar1;
      puVar1 = FUN_10015220(*(undefined4 **)(param_1 + 0x170),*(undefined4 **)(param_1 + 0x174),
                            puVar1);
      *(undefined4 **)((int)this + 0x174) = puVar1;
      *(undefined4 **)((int)this + 0x178) = puVar1;
    }
  }
  return this;
}



// public: void __fastcall Housemarque::Threedee_Engine::Trimesh::Remove(void)

void __fastcall Housemarque::Threedee_Engine::Trimesh::Remove(void)

{
  srNode *in_ECX;
  
                    // 0x11260  121  ?Remove@Trimesh@Threedee_Engine@Housemarque@@QAIXXZ
  srNode::setParent(in_ECX,(srNode *)0x0,0);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Trimesh::Add(class
// Housemarque::Threedee_Engine::Scene *)

void __fastcall Housemarque::Threedee_Engine::Trimesh::Add(Scene *param_1)

{
  srScene *psVar1;
  int iVar2;
  
                    // 0x11270  49  ?Add@Trimesh@Threedee_Engine@Housemarque@@QAIXPAVScene@23@@Z
  iVar2 = 0;
  psVar1 = Scene::Get_Sr_Scene();
  srNode::setParent((srNode *)param_1,(srNode *)psVar1,iVar2);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Trimesh::Get_Visibility_Bound_Sphere(class
// Housemarque::Template_Library::Vector_3<float> &,float &)

void __fastcall
Housemarque::Threedee_Engine::Trimesh::Get_Visibility_Bound_Sphere
          (Vector_3<float> *param_1,float *param_2)

{
  srModel *psVar1;
  float fVar2;
  undefined4 *in_stack_00000004;
  float local_18;
  float fStack_14;
  float fStack_10;
  float local_c [2];
  float *local_4;
  
                    // 0x11290  100
                    // ?Get_Visibility_Bound_Sphere@Trimesh@Threedee_Engine@Housemarque@@QAIXAAV?$Vector_3@M@Template_Library@3@AAM@Z
  psVar1 = Get_Sr_Model((int)param_1);
  if (psVar1 == (srModel *)0x0) {
    *param_2 = 0.0;
    param_2[1] = 0.0;
    param_2[2] = 0.0;
    *in_stack_00000004 = 0;
    return;
  }
  (**(code **)(*(int *)psVar1 + 0x2c))(local_c,&local_18);
  fVar2 = Get_Scale();
  local_18 = local_18 * fVar2;
  fStack_10 = fStack_10 * fVar2;
  local_c[0] = local_c[0] * fVar2;
  fStack_14 = fStack_14 * fVar2;
  Housemarque::Template_Library::V3_V3_Add_S_Mul(&fStack_14,(float *)&stack0xffffffe0,param_2,0.5);
  fVar2 = Housemarque::Template_Library::V3_V3_Sub_Norm((float *)&stack0xffffffe0,param_2);
  *local_4 = SQRT(fVar2);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Trimesh::Get_Bound_Sphere(class
// Housemarque::Template_Library::Vector_3<float> &,float &)

void __fastcall
Housemarque::Threedee_Engine::Trimesh::Get_Bound_Sphere(Vector_3<float> *param_1,float *param_2)

{
  float fVar1;
  float *in_stack_00000004;
  float local_18 [3];
  Vector_3<float> local_c [12];
  
                    // 0x11380  73
                    // ?Get_Bound_Sphere@Trimesh@Threedee_Engine@Housemarque@@QAIXAAV?$Vector_3@M@Template_Library@3@AAM@Z
  Get_Bound_Box(param_1,local_c);
  Housemarque::Template_Library::V3_V3_Add_S_Mul((float *)local_c,local_18,param_2,0.5);
  fVar1 = Housemarque::Template_Library::V3_V3_Sub_Norm(local_18,param_2);
  *in_stack_00000004 = SQRT(fVar1);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Trimesh::Get_Bound_Box(class
// Housemarque::Template_Library::Vector_3<float> &,class
// Housemarque::Template_Library::Vector_3<float> &)

void __fastcall
Housemarque::Threedee_Engine::Trimesh::Get_Bound_Box
          (Vector_3<float> *param_1,Vector_3<float> *param_2)

{
  int *piVar1;
  undefined4 unaff_ESI;
  float unaff_EDI;
  float fVar2;
  undefined4 *in_stack_00000004;
  undefined4 local_18;
  float local_14;
  float local_10;
  float local_c [2];
  float *pfStack_4;
  
                    // 0x113e0  72
                    // ?Get_Bound_Box@Trimesh@Threedee_Engine@Housemarque@@QAIXAAV?$Vector_3@M@Template_Library@3@0@Z
  piVar1 = *(int **)(param_1 + 0x164);
  if (piVar1 != (int *)0x0) {
    fVar2 = Get_Scale();
    (**(code **)(*piVar1 + 0x2c))(&local_18,local_c);
    *(undefined4 *)param_2 = unaff_ESI;
    *(float *)(param_2 + 4) = fVar2;
    *(undefined4 *)(param_2 + 8) = local_18;
    *pfStack_4 = local_14;
    pfStack_4[1] = local_10;
    pfStack_4[2] = local_c[0];
    *pfStack_4 = unaff_EDI * *pfStack_4;
    pfStack_4[1] = unaff_EDI * pfStack_4[1];
    pfStack_4[2] = unaff_EDI * pfStack_4[2];
    *(float *)param_2 = unaff_EDI * *(float *)param_2;
    *(float *)(param_2 + 4) = unaff_EDI * *(float *)(param_2 + 4);
    *(float *)(param_2 + 8) = unaff_EDI * *(float *)(param_2 + 8);
    return;
  }
  *(undefined4 *)param_2 = 0;
  *(undefined4 *)(param_2 + 4) = 0;
  *(undefined4 *)(param_2 + 8) = 0;
  *in_stack_00000004 = 0;
  in_stack_00000004[1] = 0;
  in_stack_00000004[2] = 0;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Trimesh::Get_Optimal_Bound_Box(class
// Housemarque::Template_Library::Vector_3<float> &,class
// Housemarque::Template_Library::Vector_3<float> &,class
// Housemarque::Template_Library::Matrix_3x3<float> const &,class
// Housemarque::Template_Library::Vector_3<float> const &)

void __fastcall
Housemarque::Threedee_Engine::Trimesh::Get_Optimal_Bound_Box
          (Vector_3<float> *param_1,Vector_3<float> *param_2,Matrix_3x3<float> *param_3,
          Vector_3<float> *param_4)

{
  srVector3T<float> *psVar1;
  float *pfVar2;
  int iVar3;
  float fVar4;
  float *in_stack_0000000c;
  int iStack_14;
  float afStack_c [3];
  
                    // 0x114c0  82
                    // ?Get_Optimal_Bound_Box@Trimesh@Threedee_Engine@Housemarque@@QAIXAAV?$Vector_3@M@Template_Library@3@0ABV?$Matrix_3x3@M@53@ABV453@@Z
  if (*(int *)(param_1 + 0x164) != 0) {
    fVar4 = Get_Scale();
    psVar1 = srMeshModel::getVertexLoc(*(srMeshModel **)(param_1 + 0x164));
    iStack_14 = *(int *)(*(int *)(param_1 + 0x164) + 0x22c);
    if (iStack_14 != 0) {
      *(undefined4 *)param_2 = 0x7f7fffff;
      *(undefined4 *)(param_2 + 4) = 0x7f7fffff;
      *(undefined4 *)(param_2 + 8) = 0x7f7fffff;
      *(undefined4 *)param_3 = 0xff7fffff;
      *(undefined4 *)(param_3 + 4) = 0xff7fffff;
      *(undefined4 *)(param_3 + 8) = 0xff7fffff;
      psVar1 = psVar1 + 8;
      do {
        afStack_c[1] = *(float *)(psVar1 + -4) * fVar4;
        afStack_c[2] = *(float *)psVar1 * fVar4;
        afStack_c[0] = *(float *)(psVar1 + -8) * fVar4;
        Housemarque::Template_Library::M3x3_V3_Mul_V3_Add
                  ((float *)param_4,afStack_c,in_stack_0000000c,afStack_c);
        pfVar2 = afStack_c;
        iVar3 = 3;
        do {
          if (*pfVar2 <= *(float *)(((int)param_3 - (int)afStack_c) + (int)pfVar2)) {
            if (*pfVar2 < *(float *)(((int)param_2 - (int)afStack_c) + (int)pfVar2)) {
              *(float *)(((int)param_2 - (int)afStack_c) + (int)pfVar2) = *pfVar2;
            }
          }
          else {
            *(float *)(((int)param_3 - (int)afStack_c) + (int)pfVar2) = *pfVar2;
          }
          pfVar2 = pfVar2 + 1;
          iVar3 = iVar3 + -1;
        } while (iVar3 != 0);
        psVar1 = psVar1 + 0xc;
        iStack_14 = iStack_14 + -1;
      } while (iStack_14 != 0);
      *(float *)param_3 = *(float *)param_3 + 0.2;
      *(float *)(param_3 + 4) = *(float *)(param_3 + 4) + 0.2;
      *(float *)(param_3 + 8) = *(float *)(param_3 + 8) + 0.2;
      *(float *)param_2 = *(float *)param_2 - 0.2;
      *(float *)(param_2 + 4) = *(float *)(param_2 + 4) - 0.2;
      *(float *)(param_2 + 8) = *(float *)(param_2 + 8) - 0.2;
      return;
    }
  }
  *(float *)param_2 = *in_stack_0000000c;
  *(float *)(param_2 + 4) = in_stack_0000000c[1];
  *(float *)(param_2 + 8) = in_stack_0000000c[2];
  *(float *)param_3 = *in_stack_0000000c;
  *(float *)(param_3 + 4) = in_stack_0000000c[1];
  *(float *)(param_3 + 8) = in_stack_0000000c[2];
  return;
}



// public: unsigned int __fastcall Housemarque::Threedee_Engine::Trimesh::Get_Poly_Count(void)

uint __fastcall Housemarque::Threedee_Engine::Trimesh::Get_Poly_Count(void)

{
  int in_ECX;
  
                    // 0x11690  85  ?Get_Poly_Count@Trimesh@Threedee_Engine@Housemarque@@QAIIXZ
  if (*(int *)(in_ECX + 0x164) != 0) {
    return *(uint *)(*(int *)(in_ECX + 0x164) + 0x230);
  }
  return 0;
}



// public: void __fastcall Housemarque::Threedee_Engine::Trimesh::Get_Poly(class
// Housemarque::Template_Library::Vector_3<float> &,class
// Housemarque::Template_Library::Vector_3<float> &,class
// Housemarque::Template_Library::Vector_3<float> &,unsigned int)

void __fastcall
Housemarque::Threedee_Engine::Trimesh::Get_Poly
          (Vector_3<float> *param_1,Vector_3<float> *param_2,Vector_3<float> *param_3,uint param_4)

{
  float fVar1;
  float fVar2;
  int iVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  double *pdVar7;
  srVector3i *psVar8;
  srVector3T<float> *psVar9;
  float *unaff_retaddr;
  undefined local_18 [24];
  
                    // 0x116b0  84
                    // ?Get_Poly@Trimesh@Threedee_Engine@Housemarque@@QAIXAAV?$Vector_3@M@Template_Library@3@00I@Z
  pdVar7 = (double *)srNode::getScale((srNode *)param_1,local_18);
  fVar4 = (float)*pdVar7;
  fVar5 = (float)pdVar7[1];
  fVar6 = (float)pdVar7[2];
  psVar8 = srMeshModel::getPolyVertex(*(srMeshModel **)(param_1 + 0x164));
  psVar9 = srMeshModel::getVertexLoc(*(srMeshModel **)(param_1 + 0x164));
  iVar3 = *(int *)(psVar8 + param_4 * 0xc);
  fVar1 = *(float *)(psVar9 + iVar3 * 0xc + 4);
  fVar2 = *(float *)(psVar9 + iVar3 * 0xc + 8);
  *(float *)param_2 = *(float *)(psVar9 + iVar3 * 0xc) * fVar4;
  *(float *)(param_2 + 4) = fVar1 * fVar4;
  *(float *)(param_2 + 8) = fVar4 * fVar2;
  iVar3 = *(int *)(psVar8 + param_4 * 0xc + 4);
  fVar1 = *(float *)(psVar9 + iVar3 * 0xc + 4);
  fVar2 = *(float *)(psVar9 + iVar3 * 0xc + 8);
  *unaff_retaddr = *(float *)(psVar9 + iVar3 * 0xc) * fVar5;
  unaff_retaddr[1] = fVar1 * fVar5;
  unaff_retaddr[2] = fVar5 * fVar2;
  iVar3 = *(int *)(psVar8 + param_4 * 0xc + 8);
  fVar1 = *(float *)(psVar9 + iVar3 * 0xc + 4);
  fVar2 = *(float *)(psVar9 + iVar3 * 0xc + 8);
  *(float *)param_3 = *(float *)(psVar9 + iVar3 * 0xc) * fVar6;
  *(float *)(param_3 + 4) = fVar1 * fVar6;
  *(float *)(param_3 + 8) = fVar6 * fVar2;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Trimesh::Set_Space_Model(class srMeshModel
// *,bool)

void __fastcall
Housemarque::Threedee_Engine::Trimesh::Set_Space_Model(srMeshModel *param_1,bool param_2)

{
  undefined3 in_register_00000009;
  srMeshModel in_stack_00000004;
  
                    // 0x117b0  146
                    // ?Set_Space_Model@Trimesh@Threedee_Engine@Housemarque@@QAIXPAVsrMeshModel@@_N@Z
  if (*(int *)(param_1 + 0x164) != 0) {
    Housemarque::Kernel::Throw_Error
              ((char *)0x0,s__space_model_100367d8,s_E__work_ht_3de_3de_cpp_100367c0,0x580);
    *(uint *)(param_1 + 0x164) = CONCAT31(in_register_00000009,param_2);
    param_1[0x168] = in_stack_00000004;
    return;
  }
  *(uint *)(param_1 + 0x164) = CONCAT31(in_register_00000009,param_2);
  param_1[0x168] = in_stack_00000004;
  return;
}



// public: class srMeshModel * __fastcall Housemarque::Threedee_Engine::Trimesh::Space_Model(void)

srMeshModel * __fastcall Housemarque::Threedee_Engine::Trimesh::Space_Model(void)

{
  int in_ECX;
  
                    // 0x11810  150
                    // ?Space_Model@Trimesh@Threedee_Engine@Housemarque@@QAIPAVsrMeshModel@@XZ
  return *(srMeshModel **)(in_ECX + 0x164);
}



// public: float __fastcall Housemarque::Threedee_Engine::Trimesh::Get_Scale(void)const 

float __fastcall Housemarque::Threedee_Engine::Trimesh::Get_Scale(void)

{
  double *pdVar1;
  srNode *in_ECX;
  undefined local_18 [24];
  
                    // 0x11820  91  ?Get_Scale@Trimesh@Threedee_Engine@Housemarque@@QBIMXZ
  pdVar1 = (double *)srNode::getScale(in_ECX,local_18);
  return (float)*pdVar1;
}



// public: class srModel * __fastcall Housemarque::Threedee_Engine::Trimesh::Get_Sr_Model(int)

srModel * __fastcall Housemarque::Threedee_Engine::Trimesh::Get_Sr_Model(int param_1)

{
  int iVar1;
  srModel *psVar2;
  uint in_EDX;
  
                    // 0x11840  94
                    // ?Get_Sr_Model@Trimesh@Threedee_Engine@Housemarque@@QAIPAVsrModel@@H@Z
  if (((-1 < (int)in_EDX) && (iVar1 = *(int *)(param_1 + 0x170), iVar1 != 0)) &&
     (in_EDX < (uint)(*(int *)(param_1 + 0x174) - iVar1 >> 2))) {
    return *(srModel **)(iVar1 + in_EDX * 4);
  }
                    // WARNING: Could not recover jumptable at 0x1001186f. Too many branches
                    // WARNING: Treating indirect jump as call
  psVar2 = (srModel *)(**(code **)(*(int *)(param_1 + 0x138) + 0xc))();
  return psVar2;
}



// WARNING: Removing unreachable block (ram,0x10011a03)
// WARNING: Removing unreachable block (ram,0x10011a1e)
// WARNING: Removing unreachable block (ram,0x10011a2d)
// WARNING: Removing unreachable block (ram,0x10011a34)
// public: void __fastcall Housemarque::Threedee_Engine::Trimesh::Set_Sr_Model(class
// Emissive_ARGB_Model *,int)

void __fastcall
Housemarque::Threedee_Engine::Trimesh::Set_Sr_Model(Emissive_ARGB_Model *param_1,int param_2)

{
  int iVar1;
  undefined4 *puVar2;
  uint uVar3;
  int *piVar4;
  int *piVar5;
  uint in_stack_00000004;
  int local_c;
  undefined4 *local_8;
  int local_4;
  
                    // 0x11880  147
                    // ?Set_Sr_Model@Trimesh@Threedee_Engine@Housemarque@@QAIXPAVEmissive_ARGB_Model@@H@Z
  if ((*(int *)(param_1 + 0x170) != 0) &&
     (in_stack_00000004 < (uint)(*(int *)(param_1 + 0x174) - *(int *)(param_1 + 0x170) >> 2))) {
    *(int *)(*(int *)(param_1 + 0x170) + in_stack_00000004 * 4) = param_2;
    return;
  }
  if (*(int *)(param_1 + 0x170) == 0) {
    iVar1 = 0;
  }
  else {
    iVar1 = *(int *)(param_1 + 0x174) - *(int *)(param_1 + 0x170) >> 2;
  }
  if (iVar1 <= (int)in_stack_00000004) {
    in_stack_00000004 = (in_stack_00000004 - iVar1) + 1;
    local_c = param_2;
    do {
      piVar5 = *(int **)(param_1 + 0x174);
      if (*(int *)(param_1 + 0x178) - (int)piVar5 >> 2 == 0) {
        iVar1 = *(int *)(param_1 + 0x170);
        if ((iVar1 == 0) || (uVar3 = (int)piVar5 - iVar1 >> 2, uVar3 < 2)) {
          uVar3 = 1;
        }
        if (iVar1 == 0) {
          local_4 = 0;
        }
        else {
          local_4 = (int)piVar5 - iVar1 >> 2;
        }
        local_4 = local_4 + uVar3;
        iVar1 = local_4;
        if (local_4 < 0) {
          iVar1 = 0;
        }
        puVar2 = (undefined4 *)operator_new(iVar1 * 4);
        local_8 = puVar2;
        for (piVar4 = *(int **)(param_1 + 0x170); piVar4 != piVar5; piVar4 = piVar4 + 1) {
          FUN_100166e0(puVar2,piVar4);
          puVar2 = puVar2 + 1;
        }
        FUN_100161e0(puVar2,1,&local_c);
        FUN_10015220(piVar5,*(undefined4 **)(param_1 + 0x174),puVar2 + 1);
        FUN_10015210();
        FUN_100212a4(*(void **)(param_1 + 0x170));
        puVar2 = local_8;
        *(undefined4 **)(param_1 + 0x178) = local_8 + local_4;
        iVar1 = FUN_10014a20((int)(param_1 + 0x16c));
        *(undefined4 **)(param_1 + 0x174) = puVar2 + iVar1 + 1;
        *(undefined4 **)(param_1 + 0x170) = puVar2;
      }
      else {
        FUN_10015220(piVar5,piVar5,piVar5 + 1);
        FUN_100161e0(*(undefined4 **)(param_1 + 0x174),
                     1 - ((int)*(undefined4 **)(param_1 + 0x174) - (int)piVar5 >> 2),&local_c);
        piVar4 = *(int **)(param_1 + 0x174);
        for (; piVar5 != piVar4; piVar5 = piVar5 + 1) {
          *piVar5 = local_c;
        }
        *(int *)(param_1 + 0x174) = *(int *)(param_1 + 0x174) + 4;
      }
      in_stack_00000004 = in_stack_00000004 + -1;
    } while (in_stack_00000004 != 0);
  }
  return;
}



// public: int __fastcall Housemarque::Threedee_Engine::Trimesh::Get_LOD_Count(void)const 

int __fastcall Housemarque::Threedee_Engine::Trimesh::Get_LOD_Count(void)

{
  int in_ECX;
  
                    // 0x11a60  78  ?Get_LOD_Count@Trimesh@Threedee_Engine@Housemarque@@QBIHXZ
  if (*(int *)(in_ECX + 0x170) == 0) {
    return 0;
  }
  return *(int *)(in_ECX + 0x174) - *(int *)(in_ECX + 0x170) >> 2;
}



// public: __thiscall Housemarque::Threedee_Engine::Resource_Handler::Resource_Handler(class
// Housemarque::Threedee_Engine::Engine *,bool,bool,float,class
// Housemarque::Template_Library::Matrix_3x3<float> const &)

Resource_Handler * __thiscall
Housemarque::Threedee_Engine::Resource_Handler::Resource_Handler
          (Resource_Handler *this,Engine *param_1,bool param_2,bool param_3,float param_4,
          Matrix_3x3<float> *param_5)

{
  int iVar1;
  Resource_Handler *pRVar2;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0x11a80  18
                    // ??0Resource_Handler@Threedee_Engine@Housemarque@@QAE@PAVEngine@12@_N1MABV?$Matrix_3x3@M@Template_Library@2@@Z
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10023013;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(Engine **)this = param_1;
  this[0x2a] = (Resource_Handler)0x0;
  Housemarque::Template_Library::DLIL_List_Base::DLIL_List_Base((DLIL_List_Base *)(this + 0x30));
  local_4 = 0;
  Housemarque::Template_Library::DLIL_List_Base::DLIL_List_Base((DLIL_List_Base *)(this + 0x3c));
  local_4 = CONCAT31(local_4._1_3_,1);
  Housemarque::Template_Library::DLIL_List_Base::DLIL_List_Base((DLIL_List_Base *)(this + 0x48));
  pRVar2 = this + 4;
  for (iVar1 = 9; iVar1 != 0; iVar1 = iVar1 + -1) {
    *(undefined4 *)pRVar2 = *(undefined4 *)param_5;
    param_5 = param_5 + 4;
    pRVar2 = pRVar2 + 4;
  }
  this[0x28] = (Resource_Handler)param_2;
  *(float *)(this + 0x2c) = param_4;
  this[0x29] = (Resource_Handler)param_3;
  ExceptionList = local_c;
  return this;
}



// public: __thiscall Housemarque::Threedee_Engine::Resource_Handler::~Resource_Handler(void)

void __thiscall
Housemarque::Threedee_Engine::Resource_Handler::~Resource_Handler(Resource_Handler *this)

{
  int *piVar1;
  bool bVar2;
  bool bVar3;
  bool bVar4;
  DLIL_Node_Base *pDVar5;
  int iVar6;
  basic_ostream<> *pbVar7;
  char *pcVar8;
  DLIL_Node_Base *pDVar9;
  char *pcVar10;
  int iStack_20;
  Error_Log_Inserter aEStack_1c [8];
  Error_Log_Inserter aEStack_14 [8];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0x11b10  38  ??1Resource_Handler@Threedee_Engine@Housemarque@@QAE@XZ
  puStack_8 = &LAB_100230d6;
  pvStack_c = ExceptionList;
  bVar2 = false;
  bVar4 = false;
  local_4 = 2;
  ExceptionList = &pvStack_c;
  pDVar5 = Housemarque::Template_Library::DLIL_List_Base::Head();
  while (pDVar5 != (DLIL_Node_Base *)0x0) {
    pDVar9 = pDVar5 + 0xf8;
    iStack_20 = 8;
    do {
      if (*(srClass **)pDVar9 != (srClass *)0x0) {
        iVar6 = srClass::release(*(srClass **)pDVar9);
        if ((iVar6 == 0) && (bVar3 = Housemarque::Kernel::Logging_Enabled(), bVar3)) {
          bVar4 = true;
          iVar6 = Housemarque::Kernel::Error_Log(aEStack_1c);
          local_4 = CONCAT31(local_4._1_3_,3);
          pbVar7 = std::operator<<(*(basic_ostream<> **)(iVar6 + 4),s_cant_release_model__100367ec);
          pcVar10 = &DAT_100367e8;
          pcVar8 = srRuntimeClass::getName(*(srRuntimeClass **)pDVar9);
          pbVar7 = std::operator<<(pbVar7,pcVar8);
          std::operator<<(pbVar7,pcVar10);
        }
        local_4 = 2;
        if (bVar4) {
          bVar4 = false;
          Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter(aEStack_1c);
        }
      }
      pDVar9 = pDVar9 + 4;
      iStack_20 = iStack_20 + -1;
    } while (iStack_20 != 0);
    if (*(srClass **)(pDVar5 + 0xf4) != (srClass *)0x0) {
      iVar6 = srClass::release(*(srClass **)(pDVar5 + 0xf4));
      if ((iVar6 == 0) && (bVar3 = Housemarque::Kernel::Logging_Enabled(), bVar3)) {
        bVar2 = true;
        iVar6 = Housemarque::Kernel::Error_Log(aEStack_14);
        local_4 = CONCAT31(local_4._1_3_,4);
        pbVar7 = std::operator<<(*(basic_ostream<> **)(iVar6 + 4),s_cant_release_model__10036808);
        pcVar10 = &DAT_10036804;
        pcVar8 = srRuntimeClass::getName(*(srRuntimeClass **)(pDVar5 + 0xf4));
        pbVar7 = std::operator<<(pbVar7,pcVar8);
        std::operator<<(pbVar7,pcVar10);
      }
      local_4 = 2;
      if (bVar2) {
        bVar2 = false;
        Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter(aEStack_14);
      }
    }
    Housemarque::Template_Library::DLIL_Node_Base::Rem();
    if (pDVar5 != (DLIL_Node_Base *)0x0) {
      FUN_100212a4(pDVar5);
    }
    pDVar5 = Housemarque::Template_Library::DLIL_List_Base::Head();
  }
  bVar2 = false;
  bVar4 = false;
  pDVar5 = Housemarque::Template_Library::DLIL_List_Base::Head();
  while (bVar3 = false, pDVar5 != (DLIL_Node_Base *)0x0) {
    if (*(srClass **)(pDVar5 + 0x34) != (srClass *)0x0) {
      iVar6 = srClass::release(*(srClass **)(pDVar5 + 0x34));
      if ((iVar6 == 0) && (bVar3 = Housemarque::Kernel::Logging_Enabled(), bVar3)) {
        bVar4 = true;
        iVar6 = Housemarque::Kernel::Error_Log(aEStack_14);
        local_4 = CONCAT31(local_4._1_3_,5);
        pbVar7 = std::operator<<(*(basic_ostream<> **)(iVar6 + 4),s_cant_release_material__10036824)
        ;
        pcVar10 = &DAT_10036820;
        pcVar8 = srRuntimeClass::getName(*(srRuntimeClass **)(pDVar5 + 0x34));
        pbVar7 = std::operator<<(pbVar7,pcVar8);
        std::operator<<(pbVar7,pcVar10);
      }
      local_4 = 2;
      if (bVar4) {
        bVar4 = false;
        Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter(aEStack_14);
      }
    }
    if (*(srClass **)(pDVar5 + 0x38) != (srClass *)0x0) {
      iVar6 = srClass::release(*(srClass **)(pDVar5 + 0x38));
      if ((iVar6 == 0) && (bVar3 = Housemarque::Kernel::Logging_Enabled(), bVar3)) {
        bVar2 = true;
        iVar6 = Housemarque::Kernel::Error_Log(aEStack_1c);
        local_4 = CONCAT31(local_4._1_3_,6);
        pbVar7 = std::operator<<(*(basic_ostream<> **)(iVar6 + 4),s_cant_release_material__10036840)
        ;
        pcVar10 = &DAT_1003683c;
        pcVar8 = srRuntimeClass::getName(*(srRuntimeClass **)(pDVar5 + 0x38));
        pbVar7 = std::operator<<(pbVar7,pcVar8);
        std::operator<<(pbVar7,pcVar10);
      }
      local_4 = 2;
      if (bVar2) {
        bVar2 = false;
        Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter(aEStack_1c);
      }
    }
    Housemarque::Template_Library::DLIL_Node_Base::Rem();
    if (pDVar5 != (DLIL_Node_Base *)0x0) {
      local_4._0_1_ = 7;
      std::basic_string<>::_Tidy((basic_string<> *)(pDVar5 + 8),true);
      local_4 = CONCAT31(local_4._1_3_,2);
      FUN_100212a4(pDVar5);
    }
    pDVar5 = Housemarque::Template_Library::DLIL_List_Base::Head();
  }
  pDVar5 = Housemarque::Template_Library::DLIL_List_Base::Head();
  while (pDVar5 != (DLIL_Node_Base *)0x0) {
    pcVar8 = srRuntimeClass::getName(*(srRuntimeClass **)(pDVar5 + 0xc));
    iVar6 = srClass::release(*(srClass **)(pDVar5 + 0xc));
    if ((iVar6 == 0) && (bVar4 = Housemarque::Kernel::Logging_Enabled(), bVar4)) {
      bVar3 = true;
      iVar6 = Housemarque::Kernel::Error_Log(aEStack_14);
      local_4 = CONCAT31(local_4._1_3_,8);
      pbVar7 = std::operator<<(*(basic_ostream<> **)(iVar6 + 4),s_cant_release_texture__1003685c);
      pcVar10 = &DAT_10036858;
      pbVar7 = std::operator<<(pbVar7,pcVar8);
      std::operator<<(pbVar7,pcVar10);
    }
    local_4 = 2;
    if (bVar3) {
      bVar3 = false;
      Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter(aEStack_14);
    }
    piVar1 = *(int **)(pDVar5 + 8);
    if (piVar1 != (int *)0x0) {
      (*(code *)**(undefined4 **)(*(int *)(*piVar1 + 4) + (int)piVar1))(1);
    }
    Housemarque::Template_Library::DLIL_Node_Base::Rem();
    if (pDVar5 != (DLIL_Node_Base *)0x0) {
      FUN_100212a4(pDVar5);
    }
    pDVar5 = Housemarque::Template_Library::DLIL_List_Base::Head();
  }
  bVar4 = Housemarque::Kernel::Logging_Enabled();
  if (bVar4) {
    iVar6 = Housemarque::Kernel::Error_Log(aEStack_14);
    local_4 = CONCAT31(local_4._1_3_,9);
    std::operator<<(*(basic_ostream<> **)(iVar6 + 4),s_Resource_handler_destroyed_10036874);
  }
  local_4 = 2;
  if (bVar4) {
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter(aEStack_14);
  }
  ExceptionList = pvStack_c;
  return;
}



// public: class Housemarque::Threedee_Engine::Trimesh * __fastcall
// Housemarque::Threedee_Engine::Resource_Handler::Create_Trimesh_Instance(class
// Housemarque::Threedee_Engine::Scene *,class Housemarque::Threedee_Engine::Mesh_Node *,class
// Housemarque::Threedee_Engine::Material_Node *,bool,bool,bool)

Trimesh * __fastcall
Housemarque::Threedee_Engine::Resource_Handler::Create_Trimesh_Instance
          (Scene *param_1,Mesh_Node *param_2,Material_Node *param_3,bool param_4,bool param_5,
          bool param_6)

{
  bool bVar1;
  Trimesh *this;
  Scene *pSVar2;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0x11f10  62
                    // ?Create_Trimesh_Instance@Resource_Handler@Threedee_Engine@Housemarque@@QAIPAVTrimesh@23@PAVScene@23@PAVMesh_Node@23@PAVMaterial_Node@23@_N33@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_100230eb;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  this = (Trimesh *)srHeap::allocate((srHeap *)srHeap_exref,0x180);
  uStack_4 = 0;
  if (this == (Trimesh *)0x0) {
    pSVar2 = (Scene *)0x0;
  }
  else {
    pSVar2 = (Scene *)Trimesh::Trimesh(this);
  }
  uStack_4 = 0xffffffff;
  bVar1 = Init_Trimesh_Instance((Trimesh *)param_1,pSVar2,param_2,param_3,param_4,param_5,param_6);
  if (bVar1) {
    ExceptionList = pvStack_c;
    return (Trimesh *)pSVar2;
  }
  if (pSVar2 != (Scene *)0x0) {
    (*(code *)**(undefined4 **)pSVar2)(1);
  }
  ExceptionList = pvStack_c;
  return (Trimesh *)0x0;
}



// public: bool __fastcall
// Housemarque::Threedee_Engine::Resource_Handler::Init_Trimesh_Instance(class
// Housemarque::Threedee_Engine::Trimesh &,class Housemarque::Threedee_Engine::Scene *,class
// Housemarque::Threedee_Engine::Mesh_Node *,class Housemarque::Threedee_Engine::Material_Node
// *,bool,bool,bool)

bool __fastcall
Housemarque::Threedee_Engine::Resource_Handler::Init_Trimesh_Instance
          (Trimesh *param_1,Scene *param_2,Mesh_Node *param_3,Material_Node *param_4,bool param_5,
          bool param_6,bool param_7)

{
  uint uVar1;
  int *piVar2;
  srMeshModel *this;
  srModel *psVar3;
  int iVar4;
  undefined3 in_stack_0000000d;
  char in_stack_00000018;
  undefined4 uVar5;
  
                    // 0x11fd0  108
                    // ?Init_Trimesh_Instance@Resource_Handler@Threedee_Engine@Housemarque@@QAI_NAAVTrimesh@23@PAVScene@23@PAVMesh_Node@23@PAVMaterial_Node@23@_N44@Z
  *(Mesh_Node **)(param_2 + 0x160) = param_3;
  *(undefined4 *)(param_2 + 0x17c) = *(undefined4 *)(param_4 + 0x30);
  iVar4 = 0;
  do {
    if (*(int **)(param_4 + iVar4 * 4 + 0xf8) != (int *)0x0) {
      this = (srMeshModel *)(**(code **)(**(int **)(param_4 + iVar4 * 4 + 0xf8) + 0x24))();
      FUN_10012130(this,_param_5);
      if ((in_stack_00000018 != '\0') &&
         (*(uint *)(this + 0x394) = *(uint *)(this + 0x394) | 8, ((byte)this[0x390] & 8) == 0)) {
        uVar1 = *(uint *)(this + 0x390);
        *(uint *)(this + 0x390) = uVar1 | 8;
        *(uint *)(this + 0x390) = uVar1 | 8;
      }
      if (iVar4 == 0) {
        uVar5 = 0x7f7fffff;
      }
      else {
        uVar5 = *(undefined4 *)(param_4 + iVar4 * 4 + 0x10);
      }
      FUN_1001e520(this,uVar5);
      FUN_1001e530((int)this,(char)iVar4 * '\x03',param_4 + iVar4 * 0x18 + 0x34);
      Trimesh::Set_Sr_Model((Emissive_ARGB_Model *)param_2,(int)this);
    }
    iVar4 = iVar4 + 1;
  } while (iVar4 < 8);
  iVar4 = *(int *)(param_2 + 0x138);
  psVar3 = Trimesh::Get_Sr_Model((int)param_2);
  (**(code **)(iVar4 + 4))(psVar3);
  piVar2 = *(int **)(param_4 + 0xf4);
  if (piVar2 == (int *)0x0) {
    *(undefined4 *)(param_2 + 0x164) = 0;
  }
  else if (param_6) {
    uVar5 = (**(code **)(*piVar2 + 0x24))();
    *(undefined4 *)(param_2 + 0x164) = uVar5;
    param_2[0x168] = (Scene)0x1;
  }
  else {
    *(int **)(param_2 + 0x164) = piVar2;
    param_2[0x168] = (Scene)0x0;
  }
  Trimesh::Add(param_2);
  if (param_4[8] != (Material_Node)0x0) {
    *(uint *)(param_2 + 0x148) = *(uint *)(param_2 + 0x148) | 1;
    return true;
  }
  *(uint *)(param_2 + 0x148) = *(uint *)(param_2 + 0x148) & 0xfffffffe;
  return true;
}



void __cdecl FUN_10012130(srMeshModel *param_1,int param_2)

{
  int iVar1;
  undefined4 uStack_1c;
  
  if (param_2 != 0) {
    if (*(srMaterialIFace **)(param_2 + 0x18) != (srMaterialIFace *)0x0) {
      uStack_1c = 0x10012159;
      srMeshModel::setMaterial(param_1,*(srMaterialIFace **)(param_2 + 0x18),0,0);
    }
    if (*(srMaterialIFace **)(param_2 + 0x1c) != (srMaterialIFace *)0x0) {
      uStack_1c = 0x10012169;
      srMeshModel::setMaterial(param_1,*(srMaterialIFace **)(param_2 + 0x1c),1,0);
    }
    uStack_1c = 0x10012177;
    FUN_10005ed0(&stack0xffffffec,(undefined4 *)(param_2 + 0x28));
    srMeshModel::setShader(param_1);
    FUN_10005ed0(&uStack_1c,(undefined4 *)(param_2 + 0x2c));
    srMeshModel::setShader(param_1);
    if (*(srTextureIFace **)(param_2 + 0x20) != (srTextureIFace *)0x0) {
      srMeshModel::setTexture(param_1,*(srTextureIFace **)(param_2 + 0x20),0,0);
    }
    if (*(srTextureIFace **)(param_2 + 0x24) != (srTextureIFace *)0x0) {
      srMeshModel::setTexture(param_1,*(srTextureIFace **)(param_2 + 0x24),1,0);
    }
    iVar1 = *(int *)(param_2 + 0x30);
    *(int *)(param_1 + 0x228) = iVar1;
    if (iVar1 < 1) {
      *(undefined4 *)(param_1 + 0x228) = 1;
      return;
    }
    if (4 < iVar1) {
      *(undefined4 *)(param_1 + 0x228) = 4;
    }
  }
  return;
}



// public: class srTextureIFace * __fastcall
// Housemarque::Threedee_Engine::Resource_Handler::Get_Texture(class std::basic_string<char,struct
// std::char_traits<char>,class std::allocator<char> > const &,class std::basic_string<char,struct
// std::char_traits<char>,class std::allocator<char> > const &,int,int,bool,bool)

srTextureIFace * __fastcall
Housemarque::Threedee_Engine::Resource_Handler::Get_Texture
          (basic_string<> *param_1,basic_string<> *param_2,int param_3,int param_4,bool param_5,
          bool param_6)

{
  ios_base *this;
  code cVar1;
  srRegistry *this_00;
  basic_stringstream<> *pbVar2;
  srPalette *psVar3;
  bool bVar4;
  undefined4 *puVar5;
  basic_ostream<> *pbVar6;
  char *pcVar7;
  ClassNode *pCVar8;
  ClassNode *pCVar9;
  srRuntimeClass *this_01;
  srPalette *this_02;
  srTextureFile *this_03;
  code *pcVar10;
  void *pvVar11;
  code **ppcVar12;
  code **ppcVar13;
  undefined3 in_stack_0000000d;
  char in_stack_00000014;
  basic_string<> *pbVar14;
  char *pcVar15;
  ulong uVar16;
  e_surfaceType eVar17;
  char cVar18;
  int iVar19;
  uint uVar20;
  e_mipmap eVar21;
  basic_iostream<> *local_f8;
  code **local_f4;
  basic_string<> *local_f0;
  basic_stringstream<> *local_ec;
  int iStack_e8;
  uint uStack_e4;
  srPalette *psStack_e0;
  PixelFormat aPStack_dc [6];
  char cStack_d6;
  int iStack_d4;
  int iStack_d0;
  undefined4 uStack_cc;
  uint uStack_c8;
  undefined4 uStack_c4;
  srFilter *psStack_c0;
  basic_string<> local_bc [4];
  code *pcStack_b8;
  basic_string<> abStack_ac [4];
  int iStack_a8;
  code *pcStack_9c;
  uint uStack_98;
  basic_streambuf<> abStack_94 [84];
  code *apcStack_40 [13];
  void *local_c;
  undefined *puStack_8;
  uint local_4;
  
                    // 0x121f0  96
                    // ?Get_Texture@Resource_Handler@Threedee_Engine@Housemarque@@QAIPAVsrTextureIFace@@ABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@0HH_N1@Z
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10023241;
  local_c = ExceptionList;
  ppcVar12 = (code **)0x0;
  local_f4 = (code **)0x0;
  if (param_1[0x2a] != (basic_string<>)0x0) {
    return (srTextureIFace *)0x0;
  }
  ExceptionList = &local_c;
  local_f0 = param_1;
  FUN_1000c6f0(*(void **)param_1,local_bc,param_2);
  local_4 = 0;
  local_ec = (basic_stringstream<> *)operator_new(0x8c);
  local_4._0_1_ = 1;
  local_f8 = (basic_iostream<> *)local_ec;
  if (local_ec == (basic_stringstream<> *)0x0) {
    local_ec = (basic_stringstream<> *)0x0;
  }
  else {
    *(code **)local_ec =
         _vbtable__for__std__basic_istream<char,struct_std::char_traits<char>_>___exref;
    this = (ios_base *)((basic_iostream<> *)local_ec + 0x58);
    *(code **)((basic_iostream<> *)local_ec + 8) =
         _vbtable__for__std__basic_ostream<char,struct_std::char_traits<char>_>___exref;
    std::ios_base::ios_base(this);
    ppcVar12 = (code **)0x10;
    *(code **)this = _vftable__exref;
    local_f4 = (code **)0x10;
    local_4 = CONCAT31(local_4._1_3_,2);
    std::basic_iostream<>::basic_iostream<>
              ((basic_iostream<> *)local_ec,
               (basic_streambuf<> *)((basic_iostream<> *)local_ec + 0xc));
    local_4._0_1_ = 3;
    local_4._1_3_ = 0;
    std::basic_stringbuf<>::basic_stringbuf<>
              ((basic_stringbuf<> *)((basic_iostream<> *)local_ec + 0xc),3);
    *(code **)((basic_iostream<> *)local_ec + *(int *)(*(int *)local_ec + 4)) = _vftable__exref;
    *(undefined ***)this = &PTR_LAB_10025d54;
  }
  pbVar2 = local_ec;
  local_4 = (uint)local_4._1_3_ << 8;
  bVar4 = FUN_1001ae30(_param_5);
  if ((!bVar4) || (bVar4 = FUN_1001ae30(param_4), !bVar4)) {
    ppcVar12 = (code **)((uint)ppcVar12 | 1);
    local_f4 = ppcVar12;
    puVar5 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&local_f8);
    local_4 = CONCAT31(local_4._1_3_,4);
    pbVar6 = std::operator<<((basic_ostream<> *)*puVar5,s_Texture_size_is_very_huono____100368a8);
    cVar18 = '\0';
    pbVar6 = std::operator<<(pbVar6,local_bc);
    std::operator<<(pbVar6,cVar18);
    iVar19 = 0x69e;
    pcVar15 = s_E__work_ht_3de_3de_cpp_10036890;
    pcVar7 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error
              (s_Power_Of_Two_h_____Power_Of_Two__100368c8,pcVar7,pcVar15,iVar19);
  }
  local_4 = 0;
  if (((uint)ppcVar12 & 1) != 0) {
    ppcVar12 = (code **)((uint)ppcVar12 & 0xfffffffe);
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&local_f8);
  }
  if (pbVar2 == (basic_stringstream<> *)0x0) {
    pbVar6 = (basic_ostream<> *)0x0;
  }
  else {
    pbVar6 = (basic_ostream<> *)(pbVar2 + 8);
  }
  pbVar14 = (basic_string<> *)param_3;
  iVar19 = param_4;
  uVar20 = _param_5;
  pbVar6 = std::operator<<(pbVar6,local_bc);
  pbVar6 = std::operator<<(pbVar6,pbVar14);
  pbVar6 = std::basic_ostream<>::operator<<(pbVar6,iVar19);
  std::basic_ostream<>::operator<<(pbVar6,uVar20);
  if (in_stack_00000014 != '\0') {
    if (pbVar2 == (basic_stringstream<> *)0x0) {
      pbVar6 = (basic_ostream<> *)0x0;
    }
    else {
      pbVar6 = (basic_ostream<> *)(pbVar2 + 8);
    }
    std::operator<<(pbVar6,s_mipmap_100368ec);
  }
  std::basic_ostream<>::operator<<((basic_ostream<> *)(pbVar2 + 8),local_f0);
  std::ends((basic_ostream<> *)(pbVar2 + 8));
  iVar19 = std::basic_stringstream<>::str(pbVar2,abStack_ac);
  local_4 = CONCAT31(local_4._1_3_,5);
  pcVar10 = _C_exref;
  if (*(code **)(iVar19 + 4) != (code *)0x0) {
    pcVar10 = *(code **)(iVar19 + 4);
  }
  this_00 = *(srRegistry **)(srCore_exref + 0x2c);
  pCVar8 = srRegistry::getClassNode(this_00,0x2112);
  if (pCVar8 == (ClassNode *)0x0) {
    uVar16 = 0x2112;
    pCVar9 = (ClassNode *)FUN_10015250();
    pCVar8 = srRegistry::registerClass(this_00,s_srTextureFile_100372d0,pCVar9,uVar16,(int)pCVar8);
  }
  this_01 = srRegistry::find(*(srRegistry **)(srCore_exref + 0x2c),pCVar8,(char *)pcVar10,
                             (srRuntimeClass *)0x0);
  local_4 = local_4 & 0xffffff00;
  std::basic_string<>::_Tidy(abStack_ac,true);
  if (this_01 != (srRuntimeClass *)0x0) {
    if (pbVar2 != (basic_stringstream<> *)0x0) {
      (*(code *)**(undefined4 **)(pbVar2 + *(int *)(*(int *)pbVar2 + 4)))(1);
    }
    goto LAB_100129df;
  }
  bVar4 = std::operator!=(local_bc,s_not_defined_100368f4);
  if (!bVar4) {
    ppcVar12 = (code **)((uint)ppcVar12 | 2);
    local_f4 = ppcVar12;
    puVar5 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&local_f8);
    local_4 = CONCAT31(local_4._1_3_,6);
    pbVar6 = std::operator<<((basic_ostream<> *)*puVar5,s_3DE__invalid_texture_name__10036918);
    std::operator<<(pbVar6,'\0');
    iVar19 = 0x6ab;
    pcVar15 = s_E__work_ht_3de_3de_cpp_10036900;
    pcVar7 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(s_fname_____not_defined__10036934,pcVar7,pcVar15,iVar19);
  }
  local_4 = 0;
  if (((uint)ppcVar12 & 2) != 0) {
    ppcVar12 = (code **)((uint)ppcVar12 & 0xfffffffd);
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&local_f8);
  }
  pcVar10 = pcStack_b8;
  if (pcStack_b8 == (code *)0x0) {
    pcVar10 = _C_exref;
  }
  pcStack_9c = _vbtable__exref;
  std::ios_base::ios_base((ios_base *)apcStack_40);
  ppcVar13 = (code **)((uint)ppcVar12 | 0x20);
  apcStack_40[0] = _vftable__exref;
  local_4 = CONCAT31(local_4._1_3_,7);
  local_f4 = ppcVar13;
  std::basic_istream<>::basic_istream<>((basic_istream<> *)&pcStack_9c,abStack_94,false);
  local_4 = 8;
  std::basic_filebuf<>::basic_filebuf<>((basic_filebuf<> *)abStack_94,(_iobuf *)0x0);
  local_4 = CONCAT31(local_4._1_3_,9);
  *(code **)((int)&pcStack_9c + *(int *)(pcStack_9c + 4)) = _vftable__exref;
  iVar19 = std::basic_filebuf<>::open((char *)pcVar10,1);
  if (iVar19 == 0) {
    std::basic_ios<>::setstate((basic_ios<> *)((int)&pcStack_9c + *(int *)(pcStack_9c + 4)),2,false)
    ;
  }
  local_4._0_1_ = 10;
  if ((*(uint *)((int)&uStack_98 + *(int *)(pcStack_9c + 4)) & 6) != 0) {
    ppcVar13 = (code **)((uint)ppcVar12 | 0x24);
    local_f4 = ppcVar13;
    puVar5 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&local_f8);
    local_4 = CONCAT31(local_4._1_3_,0xb);
    pbVar6 = std::operator<<((basic_ostream<> *)*puVar5,s_Error_while_opening_10036964);
    cVar18 = '\0';
    pbVar6 = std::operator<<(pbVar6,local_bc);
    std::operator<<(pbVar6,cVar18);
    iVar19 = 0x6ad;
    pcVar15 = s_E__work_ht_3de_3de_cpp_1003694c;
    pcVar7 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error
              (s__std__ifstream_fname_c_str____fa_1003697c,pcVar7,pcVar15,iVar19);
  }
  local_4 = 10;
  if (((uint)ppcVar13 & 4) != 0) {
    ppcVar13 = (code **)((uint)ppcVar13 & 0xfffffffb);
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&local_f8);
  }
  local_f4 = apcStack_40;
  *(code **)((int)&pcStack_9c + *(int *)(pcStack_9c + 4)) = _vftable__exref;
  local_4._0_1_ = 0xc;
  std::basic_filebuf<>::~basic_filebuf<>((basic_filebuf<> *)abStack_94);
  *(code **)((int)&pcStack_9c + *(int *)(pcStack_9c + 4)) = _vftable__exref;
  local_4._0_1_ = 0;
  apcStack_40[0] = _vftable__exref;
  std::ios_base::~ios_base((ios_base *)apcStack_40);
  psStack_e0 = (srPalette *)0x0;
  local_4 = CONCAT31(local_4._1_3_,0xd);
  uStack_cc = 0;
  uStack_c8 = 0;
  iStack_e8 = 0x40;
  uStack_e4 = 0x40;
  this_02 = srCore::getPalette((srCore *)srCore_exref);
  psVar3 = psStack_e0;
  if (this_02 != psStack_e0) {
    if (this_02 != (srPalette *)0x0) {
      srClass::addReference((srClass *)this_02);
    }
    psVar3 = this_02;
    if (psStack_e0 != (srPalette *)0x0) {
      srClass::release((srClass *)psStack_e0);
    }
  }
  psStack_e0 = psVar3;
  psStack_c0 = srCore::getFilter((srCore *)srCore_exref);
  uStack_c8 = 0;
  uStack_c4 = 4;
  srPixelConvert::mapPixelFormat(0xb,aPStack_dc);
  local_4._0_1_ = 0xe;
  this_03 = (srTextureFile *)srHeap::allocate((srHeap *)srHeap_exref,100);
  local_4._0_1_ = 0xf;
  if (this_03 == (srTextureFile *)0x0) {
    this_03 = (srTextureFile *)0x0;
  }
  else {
    pcVar10 = pcStack_b8;
    if (pcStack_b8 == (code *)0x0) {
      pcVar10 = _C_exref;
    }
    local_f8 = (basic_iostream<> *)this_03;
    srTextureFile::srTextureFile(this_03,(char *)pcVar10,0);
    *(undefined ***)this_03 = &PTR_FUN_10025d04;
  }
  local_4._0_1_ = 0xe;
  local_f8 = (basic_iostream<> *)this_03;
  (**(code **)(*(int *)this_03 + 0x30))(&iStack_e8);
  bVar4 = std::operator==((basic_string<> *)param_3,s_not_defined_100369a4);
  if (((bVar4) && (param_4 == 0)) && (_param_5 == 0)) {
    if (((*(char *)(*(int *)local_f0 + 0x50) != '\0') && (2 < iStack_d0 + 1U)) && (iStack_d4 == 0))
    {
      if (cStack_d6 == '\0') {
        eVar17 = 7;
      }
      else {
        eVar17 = 0xb;
      }
      srPixelConvert::mapPixelFormat(eVar17,aPStack_dc);
      uStack_c8 = uStack_c8 | 1;
      goto LAB_10012944;
    }
  }
  else {
    bVar4 = std::operator==((basic_string<> *)param_3,&DAT_100369b0);
    if (bVar4) {
      eVar17 = 7;
    }
    else {
      bVar4 = std::operator==((basic_string<> *)param_3,&DAT_100369b4);
      if (bVar4) {
        eVar17 = 0xb;
      }
      else {
        bVar4 = std::operator==((basic_string<> *)param_3,&DAT_100369bc);
        if (bVar4) {
          eVar17 = 9;
        }
        else {
          bVar4 = std::operator==((basic_string<> *)param_3,&DAT_100369c4);
          if (!bVar4) {
            bVar4 = Housemarque::Kernel::Logging_Enabled();
            if (bVar4) {
              ppcVar13 = (code **)((uint)ppcVar13 | 8);
              local_f4 = ppcVar13;
              iVar19 = Housemarque::Kernel::Error_Log(abStack_ac);
              local_4 = CONCAT31(local_4._1_3_,0x10);
              pbVar6 = std::operator<<(*(basic_ostream<> **)(iVar19 + 4),
                                       s_3DE__Texture_import__Invalid_tex_100369e4);
              pcVar7 = s___Converting_to_RGB565_100369cc;
              pbVar6 = std::operator<<(pbVar6,(basic_string<> *)param_3);
              std::operator<<(pbVar6,pcVar7);
            }
            local_4._0_1_ = 0xe;
            local_4._1_3_ = 0;
            if (((uint)ppcVar13 & 8) != 0) {
              Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
                        ((Error_Log_Inserter *)abStack_ac);
            }
          }
          eVar17 = 0xe;
        }
      }
    }
    srPixelConvert::mapPixelFormat(eVar17,aPStack_dc);
    psStack_c0 = (srFilter *)srBSplineFilter_exref;
    if (param_4 != 0) {
      iStack_e8 = param_4;
    }
    if (_param_5 != 0) {
      uStack_e4 = _param_5;
    }
LAB_10012944:
    srTexture::setDimensions((srTexture *)this_03,(Dimensions *)&iStack_e8);
  }
  pbVar2 = local_ec;
  iVar19 = std::basic_stringstream<>::str(local_ec,abStack_ac);
  local_4._0_1_ = 0x11;
  pcVar10 = *(code **)(iVar19 + 4);
  if (*(code **)(iVar19 + 4) == (code *)0x0) {
    pcVar10 = _C_exref;
  }
  srRuntimeClass::setName((srRuntimeClass *)this_03,(char *)pcVar10);
  local_4._0_1_ = 0xe;
  if (iStack_a8 != 0) {
    pcVar7 = (char *)(iStack_a8 + -1);
    cVar18 = *pcVar7;
    if ((cVar18 == '\0') || (cVar18 == -1)) {
      FUN_100212a4(pcVar7);
    }
    else {
      *pcVar7 = cVar18 + -1;
    }
  }
  pvVar11 = operator_new(0x10);
  *(basic_stringstream<> **)((int)pvVar11 + 8) = pbVar2;
  *(srTextureFile **)((int)pvVar11 + 0xc) = this_03;
  Housemarque::Template_Library::DLIL_List_Base::Add_Head((DLIL_Node_Base *)(local_f0 + 0x30));
  local_4 = (uint)local_4._1_3_ << 8;
  this_01 = (srRuntimeClass *)local_f8;
  if (psStack_e0 != (srPalette *)0x0) {
    srClass::release((srClass *)psStack_e0);
    this_01 = (srRuntimeClass *)local_f8;
  }
LAB_100129df:
  if ((local_f0[0x29] == (basic_string<>)0x0) || (in_stack_00000014 == '\0')) {
    eVar21 = 0;
  }
  else {
    eVar21 = 1;
  }
  srTexture::setMipmap((srTexture *)this_01,eVar21);
  if (pcStack_b8 != (code *)0x0) {
    cVar1 = pcStack_b8[-1];
    if ((cVar1 == (code)0x0) || (cVar1 == (code)0xff)) {
      FUN_100212a4(pcStack_b8 + -1);
    }
    else {
      pcStack_b8[-1] = (code)((char)cVar1 + -1);
    }
  }
  ExceptionList = local_c;
  return (srTextureIFace *)this_01;
}



// public: void __fastcall Housemarque::Threedee_Engine::Resource_Handler::Pre_Render_Textures(void)

void __fastcall Housemarque::Threedee_Engine::Resource_Handler::Pre_Render_Textures(void)

{
  srGERD *this;
  bool bVar1;
  DLIL_Node_Base *pDVar2;
  int iVar3;
  int *in_ECX;
  uint uStack_108;
  uint local_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  undefined4 uStack_f8;
  undefined4 uStack_f4;
  undefined4 uStack_f0;
  undefined4 uStack_ec;
  undefined4 uStack_e8;
  undefined4 uStack_e4;
  undefined4 uStack_e0;
  undefined4 uStack_dc;
  undefined4 uStack_d8;
  undefined4 uStack_d4;
  undefined4 uStack_d0;
  undefined4 uStack_cc;
  undefined4 uStack_c8;
  undefined4 uStack_c4;
  undefined4 uStack_c0;
  undefined4 uStack_bc;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  undefined4 uStack_ac;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined4 uStack_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  undefined4 uStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined *puStack_18;
  Error_Log_Inserter aEStack_14 [8];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0x12a60  118
                    // ?Pre_Render_Textures@Resource_Handler@Threedee_Engine@Housemarque@@QAIXXZ
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10023266;
  pvStack_c = ExceptionList;
  local_104 = 0;
  ExceptionList = &pvStack_c;
  pDVar2 = Housemarque::Template_Library::DLIL_List_Base::Head();
  this = *(srGERD **)(*in_ECX + 0x44);
  srGERD::beginFrame(this);
  uStack_e8 = 0;
  uStack_e4 = 0;
  uStack_e0 = 0;
  uStack_dc = 0x3f800000;
  uStack_d8 = 0x3f800000;
  uStack_d4 = 0x3f800000;
  uStack_d0 = 0x3f800000;
  uStack_cc = 0;
  uStack_c8 = 0xbf800000;
  uStack_c4 = 0xbf800000;
  uStack_c0 = 0x3f800000;
  uStack_bc = 0xbf800000;
  uStack_b8 = 0x3f800000;
  uStack_b4 = 0x3f800000;
  uStack_b0 = 0x3f800000;
  uStack_ac = 0x3f800000;
  uStack_a8 = 0x3f800000;
  uStack_a4 = 0x3f800000;
  uStack_a0 = 0xbf800000;
  uStack_9c = 0x3f800000;
  uStack_98 = 0;
  uStack_94 = 0;
  uStack_90 = 0;
  uStack_8c = 0;
  uStack_88 = 0;
  uStack_84 = 0;
  uStack_80 = 0;
  uStack_7c = 0;
  uStack_78 = 0;
  uStack_74 = 0;
  uStack_70 = 0;
  uStack_6c = 0;
  uStack_68 = 0;
  uStack_64 = 0;
  uStack_60 = 0;
  uStack_5c = 0;
  uStack_100 = 0;
  uStack_fc = 3;
  uStack_f8 = 1;
  uStack_f4 = 1;
  uStack_f0 = 3;
  uStack_ec = 2;
  FUN_10014400(&uStack_108);
  uStack_108 = uStack_108 & 0xfffffff7 | 0x8007;
  if (uStack_108 != *(uint *)(this + 0x1ff8)) {
    *(uint *)(this + 0x1ff8) = uStack_108;
    *(uint *)(this + 0x24) = *(uint *)(this + 0x24) | 0x1000;
  }
  uStack_58 = 0x3f800000;
  uStack_54 = 0;
  uStack_50 = 0;
  uStack_4c = 0;
  uStack_48 = 0;
  uStack_44 = 0x3f800000;
  uStack_40 = 0;
  uStack_3c = 0;
  uStack_38 = 0;
  uStack_34 = 0;
  uStack_30 = 0x3f800000;
  uStack_2c = 0;
  uStack_28 = 0;
  uStack_24 = 0;
  uStack_20 = 0;
  uStack_1c = 0x3f800000;
  srGERD::matrixMode(this,1);
  srGERD::loadMatrix(this,(srMatrix4T<float> *)&uStack_58);
  while (pDVar2 != (DLIL_Node_Base *)0x0) {
    *(undefined4 *)(this + 0x21c4) = 0x12;
    *(uint *)(this + 0x21c0) = *(uint *)(this + 0x21c0) | 1;
    *(undefined4 *)(this + 0x21c8) = 4;
    *(undefined4 *)(this + 0x21d0) = 3;
    *(undefined4 *)(this + 0x21e8) = 1;
    *(undefined4 *)(this + 0x2200) = 0xc;
    *(undefined4 **)(this + 0x2218) = &uStack_c8;
    *(uint *)(this + 0x21c0) = *(uint *)(this + 0x21c0) | 1;
    *(undefined4 *)(this + 0x21e0) = 2;
    *(undefined4 *)(this + 0x21f8) = 1;
    *(undefined4 *)(this + 0x2210) = 8;
    *(undefined4 **)(this + 0x2228) = &uStack_e8;
    *(uint *)(this + 0x21c0) = *(uint *)(this + 0x21c0) | 1;
    *(undefined4 *)(this + 0x21d4) = 4;
    *(undefined4 *)(this + 0x21ec) = 1;
    *(undefined4 *)(this + 0x2204) = 0x10;
    *(undefined4 **)(this + 0x221c) = &uStack_98;
    puStack_18 = &stack0xfffffee4;
    *(uint *)(this + 0x21c0) = *(uint *)(this + 0x21c0) | 1;
    srGERD::setClipState(this);
    srGERD::setTexture(this,*(srTextureIFace **)(pDVar2 + 0xc),0);
    srGERD::drawElements(this,5,6,2,&uStack_100);
    pDVar2 = Housemarque::Template_Library::DLIL_Node_Base::Succ();
  }
  bVar1 = Housemarque::Kernel::Logging_Enabled();
  if (bVar1) {
    iVar3 = Housemarque::Kernel::Error_Log(aEStack_14);
    local_104 = 1;
    uStack_4 = 0;
    std::operator<<(*(basic_ostream<> **)(iVar3 + 4),s_All_textures_rendered_10036a10);
  }
  uStack_4 = 0xffffffff;
  if ((local_104 & 1) != 0) {
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter(aEStack_14);
  }
  srGERD::endFrame(this);
  ExceptionList = pvStack_c;
  return;
}



// public: void __fastcall
// Housemarque::Threedee_Engine::Resource_Handler::Import_Material_List(class std::list<class
// Housemarque::Threedee_Engine::Material_Node *,class std::allocator<class
// Housemarque::Threedee_Engine::Material_Node *> > &,char const *,bool)

void __fastcall
Housemarque::Threedee_Engine::Resource_Handler::Import_Material_List
          (list<> *param_1,char *param_2,bool param_3)

{
  _iobuf *p_Var1;
  undefined4 *puVar2;
  basic_ostream<> *pbVar3;
  char *pcVar4;
  uint uVar5;
  undefined3 in_stack_00000005;
  char *pcVar6;
  char cVar7;
  int iVar8;
  Inserter aIStack_ac [4];
  uint local_a8;
  basic_istream<> *local_a4;
  list<> *local_a0;
  code *local_9c;
  uint uStack_98;
  code *apcStack_94 [17];
  void *pvStack_50;
  char cStack_4c;
  locale alStack_48 [4];
  FILE *pFStack_44;
  code *local_40 [13];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0x12e30  104
                    // ?Import_Material_List@Resource_Handler@Threedee_Engine@Housemarque@@QAIXAAV?$list@PAVMaterial_Node@Threedee_Engine@Housemarque@@V?$allocator@PAVMaterial_Node@Threedee_Engine@Housemarque@@@std@@@std@@PBD_N@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10023315;
  pvStack_c = ExceptionList;
  local_a8 = 0;
  local_9c = _vbtable__exref;
  ExceptionList = &pvStack_c;
  local_a4 = (basic_istream<> *)param_2;
  local_a0 = param_1;
  std::ios_base::ios_base((ios_base *)local_40);
  uVar5 = 4;
  local_40[0] = _vftable__exref;
  local_a8 = 4;
  uStack_4 = 0;
  std::basic_istream<>::basic_istream<>
            ((basic_istream<> *)&local_9c,(basic_streambuf<> *)apcStack_94,false);
  uStack_4 = 1;
  std::basic_filebuf<>::basic_filebuf<>((basic_filebuf<> *)apcStack_94,(_iobuf *)0x0);
  *(code **)((int)&local_9c + *(int *)(local_9c + 4)) = _vftable__exref;
  uStack_4 = CONCAT31(uStack_4._1_3_,2);
  if (pFStack_44 == (FILE *)0x0) {
    p_Var1 = std::__Fiopen(_param_3,1);
    if (p_Var1 != (_iobuf *)0x0) {
      std::basic_filebuf<>::_Init((basic_filebuf<> *)apcStack_94,p_Var1,1);
      std::basic_filebuf<>::_Initcvt((basic_filebuf<> *)apcStack_94);
      if (&stack0x00000000 != (undefined *)0x94) goto LAB_10012f1a;
    }
  }
  std::basic_ios<>::setstate((basic_ios<> *)((int)&local_9c + *(int *)(local_9c + 4)),2,false);
LAB_10012f1a:
  uStack_4 = 3;
  if ((*(uint *)((int)&uStack_98 + *(int *)(local_9c + 4)) & 6) != 0) {
    uVar5 = 5;
    local_a8 = 5;
    puVar2 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(aIStack_ac);
    uStack_4 = CONCAT31(uStack_4._1_3_,4);
    pbVar3 = std::operator<<((basic_ostream<> *)*puVar2,s_3DE__Can_t_open_material_list_fi_10036a40)
    ;
    cVar7 = '\0';
    pbVar3 = std::operator<<(pbVar3,_param_3);
    std::operator<<(pbVar3,cVar7);
    iVar8 = 0x750;
    pcVar6 = s_E__work_ht_3de_3de_cpp_10036a28;
    pcVar4 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(s__data_fail___10036a68,pcVar4,pcVar6,iVar8);
  }
  uStack_4 = 3;
  if ((uVar5 & 1) != 0) {
    uVar5 = uVar5 & 0xfffffffe;
    Housemarque::Kernel::Inserter::~Inserter(aIStack_ac);
  }
  Import_Material_List(local_a0,local_a4,SUB41(&local_9c,0));
  if (*(int *)(local_a4 + 8) == 0) {
    uVar5 = uVar5 | 2;
    local_a8 = uVar5;
    puVar2 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(aIStack_ac);
    uStack_4 = CONCAT31(uStack_4._1_3_,5);
    pbVar3 = std::operator<<((basic_ostream<> *)*puVar2,s_3DE__Material_list_file_empty__10036a90);
    cVar7 = '\0';
    pbVar3 = std::operator<<(pbVar3,_param_3);
    std::operator<<(pbVar3,cVar7);
    iVar8 = 0x754;
    pcVar6 = s_E__work_ht_3de_3de_cpp_10036a78;
    pcVar4 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(s_mat_list_size__>0_10036ab0,pcVar4,pcVar6,iVar8);
  }
  uStack_4 = 3;
  if ((uVar5 & 2) != 0) {
    Housemarque::Kernel::Inserter::~Inserter(aIStack_ac);
  }
  local_a0 = (list<> *)local_40;
  *(code **)((int)&local_9c + *(int *)(local_9c + 4)) = _vftable__exref;
  local_a4 = (basic_istream<> *)apcStack_94;
  apcStack_94[0] = _vftable__exref;
  uStack_4 = 8;
  if ((cStack_4c != '\0') && (pFStack_44 != (FILE *)0x0)) {
    iVar8 = fclose(pFStack_44);
    if (iVar8 == 0) {
      std::basic_filebuf<>::_Init((basic_filebuf<> *)apcStack_94,(_iobuf *)0x0,2);
    }
  }
  if (pvStack_50 != (void *)0x0) {
    FUN_10015120(pvStack_50,1);
  }
  std::locale::~locale(alStack_48);
  uStack_4 = CONCAT31(uStack_4._1_3_,6);
  std::basic_streambuf<>::~basic_streambuf<>((basic_streambuf<> *)apcStack_94);
  *(code **)((int)&local_9c + *(int *)(local_9c + 4)) = _vftable__exref;
  uStack_4 = 0xffffffff;
  local_40[0] = _vftable__exref;
  std::ios_base::~ios_base((ios_base *)local_40);
  ExceptionList = pvStack_c;
  return;
}



// public: void __fastcall
// Housemarque::Threedee_Engine::Resource_Handler::Import_Material_List(class std::list<class
// Housemarque::Threedee_Engine::Material_Node *,class std::allocator<class
// Housemarque::Threedee_Engine::Material_Node *> > &,class std::basic_istream<char,struct
// std::char_traits<char> > &,bool)

void __fastcall
Housemarque::Threedee_Engine::Resource_Handler::Import_Material_List
          (list<> *param_1,basic_istream<> *param_2,bool param_3)

{
  int *piVar1;
  bool bVar2;
  basic_stringstream<> *pbVar3;
  Material_Node *pMVar4;
  int *piVar5;
  int *piVar6;
  int *piVar7;
  undefined3 in_stack_00000005;
  Segment local_154 [4];
  undefined auStack_150 [156];
  Material_Config aMStack_b4 [168];
  void *local_c;
  undefined *puStack_8;
  int iStack_4;
  
                    // 0x13130  103
                    // ?Import_Material_List@Resource_Handler@Threedee_Engine@Housemarque@@QAIXAAV?$list@PAVMaterial_Node@Threedee_Engine@Housemarque@@V?$allocator@PAVMaterial_Node@Threedee_Engine@Housemarque@@@std@@@std@@AAV?$basic_istream@DU?$char_traits@D@std@@@5@_N@Z
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_10023336;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if ((*(uint *)(_param_3 + *(int *)(*(int *)_param_3 + 4) + 4) & 1) == 0) {
    do {
      Housemarque::Legacy::Segment::Segment(local_154,_param_3,'{','}',';');
      iStack_4 = 0;
      bVar2 = std::operator==((basic_string<> *)
                              (-(uint)(&stack0x00000000 != (undefined *)0x154) & (uint)auStack_150),
                              s_material_10036ac4);
      if (bVar2) {
        pbVar3 = Housemarque::Legacy::Segment::Stream();
        Material_Config::Material_Config(aMStack_b4,(basic_istream<> *)pbVar3);
        iStack_4._0_1_ = 1;
        pMVar4 = Import_Material((Material_Config *)param_1,SUB41(aMStack_b4,0));
        piVar1 = (int *)**(undefined4 **)(param_2 + 4);
        piVar7 = (int *)piVar1[1];
        piVar5 = (int *)operator_new(0xc);
        piVar6 = piVar1;
        if (piVar1 == (int *)0x0) {
          piVar6 = piVar5;
        }
        *piVar5 = (int)piVar6;
        if (piVar7 == (int *)0x0) {
          piVar7 = piVar5;
        }
        piVar5[1] = (int)piVar7;
        piVar1[1] = (int)piVar5;
        *(int **)piVar5[1] = piVar5;
        if (piVar5 + 2 != (int *)0x0) {
          piVar5[2] = (int)pMVar4;
        }
        *(int *)(param_2 + 8) = *(int *)(param_2 + 8) + 1;
        iStack_4 = (uint)iStack_4._1_3_ << 8;
        Material_Config::~Material_Config(aMStack_b4);
      }
      iStack_4 = 0xffffffff;
      Housemarque::Legacy::Segment::~Segment(local_154);
    } while (((byte)_param_3[*(int *)(*(int *)_param_3 + 4) + 4] & 1) == 0);
  }
  ExceptionList = local_c;
  return;
}



// public: class Housemarque::Threedee_Engine::Material_Node * __fastcall
// Housemarque::Threedee_Engine::Resource_Handler::Import_Material(class
// Housemarque::Threedee_Engine::Material_Config &,bool)

Material_Node * __fastcall
Housemarque::Threedee_Engine::Resource_Handler::Import_Material
          (Material_Config *param_1,bool param_2)

{
  basic_string<> *pbVar1;
  char cVar2;
  uint uVar3;
  bool bVar4;
  srClass *psVar5;
  int iVar6;
  basic_ostream<> *pbVar7;
  srClass *this;
  Material_Node *pMVar8;
  code *pcVar9;
  uint uVar10;
  uint uVar11;
  undefined3 in_register_00000009;
  int iVar12;
  Material_Node *pMVar13;
  Material_Node *pMVar14;
  code *pcVar15;
  char *pcVar16;
  void *pvVar17;
  char *pcVar18;
  Material_Node MStack_51;
  uint local_50;
  uint local_4c;
  uint local_48;
  Material_Config *local_44;
  srTextureIFace *local_40;
  basic_string<> *pbStack_3c;
  srMaterial *local_38;
  srTextureIFace *local_34;
  undefined4 local_30;
  srClass *psStack_2c;
  undefined4 uStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined4 uStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  iVar12 = CONCAT31(in_register_00000009,param_2);
                    // 0x13280  102
                    // ?Import_Material@Resource_Handler@Threedee_Engine@Housemarque@@QAIPAVMaterial_Node@23@AAVMaterial_Config@23@_N@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10023396;
  pvStack_c = ExceptionList;
  local_4c = 0;
  local_38 = (srMaterial *)0x0;
  local_40 = (srTextureIFace *)0x0;
  local_34 = (srTextureIFace *)0x0;
  ExceptionList = &pvStack_c;
  local_44 = param_1;
  FUN_10014400(&local_50);
  FUN_10014400(&local_48);
  local_30 = 1;
  psVar5 = (srClass *)srHeap::allocate((srHeap *)srHeap_exref,0x78);
  uStack_4 = 0;
  psStack_2c = psVar5;
  if (psVar5 == (srClass *)0x0) {
    psVar5 = (srClass *)0x0;
  }
  else {
    FUN_10014b40(psVar5);
    uStack_4 = CONCAT31(uStack_4._1_3_,1);
    FUN_10014440((int)(psVar5 + 0x18));
    *(undefined4 *)(psVar5 + 0x6c) = 0;
    *(code **)psVar5 = _vftable__exref;
    srMaterial::reset((srMaterial *)psVar5);
    *(undefined ***)psVar5 = &PTR_FUN_10025d5c;
  }
  pbStack_3c = (basic_string<> *)(iVar12 + 0xc);
  uStack_4 = 0xffffffff;
  pcVar9 = *(code **)(iVar12 + 0x10);
  if (*(code **)(iVar12 + 0x10) == (code *)0x0) {
    pcVar9 = _C_exref;
  }
  srRuntimeClass::setName((srRuntimeClass *)psVar5,(char *)pcVar9);
  psStack_2c = (srClass *)0x0;
  uStack_28 = 0;
  uStack_24 = 0;
  uStack_20 = 0;
  srMaterial::setVector
            ((srMaterial *)psVar5,(srVector4T<float> *)((srRuntimeClass *)psVar5 + 0x28),
             (srVector4T<float> *)&psStack_2c);
  psStack_2c = (srClass *)0x0;
  uStack_28 = 0;
  uStack_24 = 0;
  uStack_20 = 0;
  srMaterial::setVector
            ((srMaterial *)psVar5,(srVector4T<float> *)((srRuntimeClass *)psVar5 + 0x54),
             (srVector4T<float> *)&psStack_2c);
  psStack_2c = (srClass *)0x0;
  uStack_28 = 0;
  uStack_24 = 0;
  uStack_20 = 0;
  srMaterial::setVector
            ((srMaterial *)psVar5,(srVector4T<float> *)((srRuntimeClass *)psVar5 + 0x18),
             (srVector4T<float> *)&psStack_2c);
  psStack_2c._0_1_ = (srVector4T<float>)0x0;
  psStack_2c._1_3_ = 0;
  uStack_28 = 0;
  uStack_24 = 0;
  uStack_20 = 0;
  srMaterial::setVector
            ((srMaterial *)psVar5,(srVector4T<float> *)((srRuntimeClass *)psVar5 + 0x38),
             (srVector4T<float> *)&psStack_2c);
  pbVar1 = (basic_string<> *)(iVar12 + 0x38);
  bVar4 = std::operator==(pbVar1,s_additive_10036ad0);
  if (bVar4) {
    psStack_2c._0_1_ = *(srVector4T<float> *)(iVar12 + 0x34);
    local_40 = (srTextureIFace *)CONCAT31(local_40._1_3_,*(undefined *)(iVar12 + 0x35));
    local_40 = Get_Texture((basic_string<> *)local_44,(basic_string<> *)(iVar12 + 0x84),
                           iVar12 + 0x1c,*(int *)(iVar12 + 0x2c),
                           SUB41(*(undefined4 *)(iVar12 + 0x30),0),(bool)psStack_2c._0_1_);
    if (*(char *)(iVar12 + 0x94) != '\0') {
      bVar4 = Housemarque::Kernel::Logging_Enabled();
      if (bVar4) {
        iVar6 = Housemarque::Kernel::Error_Log(&psStack_2c);
        local_4c = 1;
        uStack_4 = 2;
        std::operator<<(*(basic_ostream<> **)(iVar6 + 4),s_additive_envmap_10036adc);
      }
      uStack_4 = 0xffffffff;
      if ((local_4c & 1) != 0) {
        local_4c = local_4c & 0xfffffffe;
        Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
                  ((Error_Log_Inserter *)&psStack_2c);
      }
      *(code **)((srRuntimeClass *)psVar5 + 0x70) = srEnvironmentMapper_exref;
      bVar4 = Housemarque::Kernel::Logging_Enabled();
      if (bVar4) {
        pvVar17 = *(void **)((srRuntimeClass *)psVar5 + 0x70);
        local_4c = local_4c | 2;
        iVar6 = Housemarque::Kernel::Error_Log(&psStack_2c);
        uStack_4 = 3;
        pbVar7 = std::operator<<(*(basic_ostream<> **)(iVar6 + 4),s__srEnvironmentMapper__10036b0c);
        pcVar18 = &DAT_10036af0;
        pcVar16 = s_mat0_>getMapper____10036af4;
        pbVar7 = std::basic_ostream<>::operator<<(pbVar7,srEnvironmentMapper_exref);
        pbVar7 = std::operator<<(pbVar7,pcVar16);
        pbVar7 = std::basic_ostream<>::operator<<(pbVar7,pvVar17);
        std::operator<<(pbVar7,pcVar18);
      }
      uStack_4 = 0xffffffff;
      if ((local_4c & 2) != 0) {
        Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
                  ((Error_Log_Inserter *)&psStack_2c);
      }
    }
    local_50 = local_50 & 0xffffb337 | 0xa020;
  }
  else {
    bVar4 = std::operator==(pbVar1,s_normal_10036b24);
    if (bVar4) {
      psStack_2c._0_1_ = *(srVector4T<float> *)(iVar12 + 0x35);
      local_40 = (srTextureIFace *)CONCAT31(local_40._1_3_,*(bool *)(iVar12 + 0x34));
      local_40 = Get_Texture((basic_string<> *)local_44,(basic_string<> *)(iVar12 + 0x48),
                             iVar12 + 0x1c,*(int *)(iVar12 + 0x2c),
                             SUB41(*(undefined4 *)(iVar12 + 0x30),0),*(bool *)(iVar12 + 0x34));
      local_50 = local_50 | 0x8000;
      uVar10 = 0xffffffff;
      pcVar16 = s_not_defined_10036b2c;
      do {
        if (uVar10 == 0) break;
        uVar10 = uVar10 - 1;
        cVar2 = *pcVar16;
        pcVar16 = pcVar16 + 1;
      } while (cVar2 != '\0');
      iVar6 = std::basic_string<>::compare
                        ((basic_string<> *)(iVar12 + 0x58),0,*(uint *)(iVar12 + 0x60),
                         s_not_defined_10036b2c,~uVar10 - 1);
      if (iVar6 != 0) {
        psStack_2c = (srClass *)CONCAT31(psStack_2c._1_3_,*(undefined *)(iVar12 + 0x35));
        local_34 = (srTextureIFace *)CONCAT31(local_34._1_3_,*(bool *)(iVar12 + 0x80));
        local_30 = 2;
        local_34 = Get_Texture((basic_string<> *)local_44,(basic_string<> *)(iVar12 + 0x58),
                               iVar12 + 0x68,*(int *)(iVar12 + 0x78),
                               SUB41(*(undefined4 *)(iVar12 + 0x7c),0),*(bool *)(iVar12 + 0x80));
        this = (srClass *)srHeap::allocate((srHeap *)srHeap_exref,0x78);
        uStack_4 = 4;
        if (this == (srClass *)0x0) {
          this = (srClass *)0x0;
        }
        else {
          psStack_2c = this;
          FUN_10014b40(this);
          uStack_4 = CONCAT31(uStack_4._1_3_,5);
          FUN_10014440((int)(this + 0x18));
          *(undefined4 *)(this + 0x6c) = 0;
          *(code **)this = _vftable__exref;
          srMaterial::reset((srMaterial *)this);
          *(undefined ***)this = &PTR_FUN_10025d5c;
        }
        uStack_4 = 0xffffffff;
        *(code **)((srMaterial *)this + 0x70) = srEnvironmentMapper_exref;
        psStack_2c = (srClass *)0x0;
        uStack_28 = 0;
        uStack_24 = 0;
        uStack_20 = 0;
        local_38 = (srMaterial *)this;
        srMaterial::setVector
                  ((srMaterial *)this,(srVector4T<float> *)((srMaterial *)this + 0x28),
                   (srVector4T<float> *)&psStack_2c);
        psStack_2c = (srClass *)0x0;
        uStack_28 = 0;
        uStack_24 = 0;
        uStack_20 = 0;
        srMaterial::setVector
                  ((srMaterial *)this,(srVector4T<float> *)((srMaterial *)this + 0x54),
                   (srVector4T<float> *)&psStack_2c);
        psStack_2c._0_1_ = (srVector4T<float>)0x0;
        psStack_2c._1_3_ = 0;
        uStack_28 = 0;
        uStack_24 = 0;
        uStack_20 = 0;
        srMaterial::setVector
                  ((srMaterial *)this,(srVector4T<float> *)((srMaterial *)this + 0x18),
                   (srVector4T<float> *)&psStack_2c);
        uStack_1c = 0;
        uStack_18 = 0;
        uStack_14 = 0;
        uStack_10 = 0;
        srMaterial::setVector
                  ((srMaterial *)this,(srVector4T<float> *)((srMaterial *)this + 0x38),
                   (srVector4T<float> *)&uStack_1c);
        local_48 = local_48 & 0xffffb337 | 0xa020;
      }
    }
    else {
      bVar4 = std::operator==(pbVar1,s_transparent_10036b38);
      if (bVar4) {
        psStack_2c._0_1_ = *(srVector4T<float> *)(iVar12 + 0x35);
        local_40 = (srTextureIFace *)CONCAT31(local_40._1_3_,*(bool *)(iVar12 + 0x34));
        local_40 = Get_Texture((basic_string<> *)local_44,(basic_string<> *)(iVar12 + 0x48),
                               iVar12 + 0x1c,*(int *)(iVar12 + 0x2c),
                               SUB41(*(undefined4 *)(iVar12 + 0x30),0),*(bool *)(iVar12 + 0x34));
        *(undefined4 *)((srRuntimeClass *)psVar5 + 0x24) = 0x3f800000;
        *(undefined4 *)((srRuntimeClass *)psVar5 + 0x74) = 1;
        local_50 = local_50 & 0xffffdfb7 | 0xc0a0;
      }
    }
  }
  if (*(char *)(iVar12 + 0x81) != '\0') {
    local_50 = local_50 | 7;
    local_48 = local_48 | 7;
  }
  if (*(char *)(iVar12 + 0x82) != '\0') {
    local_50 = local_50 | 0x800008;
    local_48 = local_48 | 0x800008;
  }
  if (*(basic_string<> *)(local_44 + 0x2a) != (basic_string<>)0x0) {
    local_50 = local_50 & 0xffff7fff;
  }
  pMVar8 = (Material_Node *)operator_new(0x3c);
  pMVar13 = (Material_Node *)0x0;
  if (pMVar8 != (Material_Node *)0x0) {
    *(undefined4 *)(pMVar8 + 0xc) = 0;
    *(undefined4 *)(pMVar8 + 0x10) = 0;
    *(undefined4 *)(pMVar8 + 0x14) = 0;
    pMVar8[8] = MStack_51;
    pMVar14 = pMVar8 + 0x28;
    iVar12 = 2;
    do {
      FUN_10014400((undefined4 *)pMVar14);
      pMVar14 = pMVar14 + 4;
      iVar12 = iVar12 + -1;
      pMVar13 = pMVar8;
    } while (iVar12 != 0);
  }
  uVar10 = *(uint *)(pbStack_3c + 8);
  uVar3 = *(uint *)npos_exref;
  pbVar1 = (basic_string<> *)(pMVar13 + 8);
  uVar11 = uVar10;
  if (uVar3 < uVar10) {
    uVar11 = uVar3;
  }
  local_4c = uVar11;
  if (pbVar1 == pbStack_3c) {
    std::basic_string<>::erase(pbVar1,uVar11,uVar3);
    std::basic_string<>::erase(pbVar1,0,0);
  }
  else {
    if ((uVar11 != 0) && (uVar11 == uVar10)) {
      pcVar9 = *(code **)(pbStack_3c + 4);
      if (*(code **)(pbStack_3c + 4) == (code *)0x0) {
        pcVar9 = _C_exref;
      }
      if ((byte)pcVar9[-1] < 0xfe) {
        if (*(int *)(pMVar13 + 0xc) != 0) {
          pcVar16 = (char *)(*(int *)(pMVar13 + 0xc) + -1);
          cVar2 = *pcVar16;
          if ((cVar2 == '\0') || (cVar2 == -1)) {
            FUN_100212a4(pcVar16);
          }
          else {
            *pcVar16 = cVar2 + -1;
          }
        }
        *(undefined4 *)(pMVar13 + 0xc) = 0;
        *(undefined4 *)(pMVar13 + 0x10) = 0;
        *(undefined4 *)(pMVar13 + 0x14) = 0;
        pcVar9 = *(code **)(pbStack_3c + 4);
        if (*(code **)(pbStack_3c + 4) == (code *)0x0) {
          pcVar9 = _C_exref;
        }
        *(code **)(pMVar13 + 0xc) = pcVar9;
        *(undefined4 *)(pMVar13 + 0x10) = *(undefined4 *)(pbStack_3c + 8);
        *(undefined4 *)(pMVar13 + 0x14) = *(undefined4 *)(pbStack_3c + 0xc);
        pcVar9[-1] = (code)((char)pcVar9[-1] + '\x01');
        goto LAB_10013946;
      }
    }
    bVar4 = std::basic_string<>::_Grow(pbVar1,uVar11,true);
    if (bVar4) {
      pcVar9 = *(code **)(pbStack_3c + 4);
      if (*(code **)(pbStack_3c + 4) == (code *)0x0) {
        pcVar9 = _C_exref;
      }
      pcVar15 = *(code **)(pMVar13 + 0xc);
      for (uVar10 = uVar11 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
        *(undefined4 *)pcVar15 = *(undefined4 *)pcVar9;
        pcVar9 = pcVar9 + 4;
        pcVar15 = pcVar15 + 4;
      }
      for (uVar11 = uVar11 & 3; uVar11 != 0; uVar11 = uVar11 - 1) {
        *pcVar15 = *pcVar9;
        pcVar9 = pcVar9 + 1;
        pcVar15 = pcVar15 + 1;
      }
      std::basic_string<>::_Eos((basic_string<> *)(pMVar13 + 8),local_4c);
    }
  }
LAB_10013946:
  *(srTextureIFace **)(pMVar13 + 0x24) = local_34;
  *(srMaterial **)(pMVar13 + 0x1c) = local_38;
  *(srMaterial **)(pMVar13 + 0x38) = local_38;
  *(srTextureIFace **)(pMVar13 + 0x20) = local_40;
  *(srClass **)(pMVar13 + 0x18) = psVar5;
  *(srClass **)(pMVar13 + 0x34) = psVar5;
  *(uint *)(pMVar13 + 0x28) = local_50;
  *(uint *)(pMVar13 + 0x2c) = local_48;
  *(undefined4 *)(pMVar13 + 0x30) = local_30;
  Housemarque::Template_Library::DLIL_List_Base::Add_Head((DLIL_Node_Base *)(local_44 + 0x48));
  ExceptionList = pvStack_c;
  return pMVar13;
}



// public: class Housemarque::Threedee_Engine::Mesh_Node * __fastcall
// Housemarque::Threedee_Engine::Resource_Handler::Import_Trimesh(class
// Housemarque::Threedee_Engine::Trimesh_Config &,class std::list<class
// Housemarque::Threedee_Engine::Material_Node *,class std::allocator<class
// Housemarque::Threedee_Engine::Material_Node *> > const &)

Mesh_Node * __fastcall
Housemarque::Threedee_Engine::Resource_Handler::Import_Trimesh
          (Trimesh_Config *param_1,list<> *param_2)

{
  bool bVar1;
  srGFInterface *psVar2;
  undefined4 *puVar3;
  basic_ostream<> *pbVar4;
  char *pcVar5;
  list<> *plVar6;
  srMeshModel *psVar7;
  code *pcVar8;
  srGFInterface *psVar9;
  list<> *plVar10;
  undefined uStack00000004;
  char *pcVar11;
  char cVar12;
  int iVar13;
  srGFInterface *local_50;
  uint local_4c;
  Trimesh_Config *local_48;
  srGFInterface *local_44;
  list<> *local_40;
  float fStack_3c;
  srGFInterface *psStack_38;
  undefined4 uStack_34;
  float fStack_30;
  undefined4 uStack_2c;
  undefined4 uStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined uStack_1c;
  undefined4 uStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0x139a0  106
                    // ?Import_Trimesh@Resource_Handler@Threedee_Engine@Housemarque@@QAIPAVMesh_Node@23@AAVTrimesh_Config@23@ABV?$list@PAVMaterial_Node@Threedee_Engine@Housemarque@@V?$allocator@PAVMaterial_Node@Threedee_Engine@Housemarque@@@std@@@std@@@Z
  local_4 = 0xffffffff;
  puStack_8 = &LAB_100233f6;
  pvStack_c = ExceptionList;
  local_4c = 0;
  ExceptionList = &pvStack_c;
  local_48 = param_1;
  local_40 = param_2;
  FUN_1000eed0();
  local_50 = (srGFInterface *)operator_new(0x118);
  local_44 = local_50;
  if (local_50 == (srGFInterface *)0x0) {
    local_50 = (srGFInterface *)0x0;
  }
  else {
    local_4 = 1;
    FUN_10005d90(local_50 + 0x34,0x18,8,Coeff_Info::Coeff_Info);
  }
  local_4 = 0xffffffff;
  bVar1 = std::operator!=((basic_string<> *)(param_2 + 0xc),s_not_defined_10036b44);
  if (bVar1) {
    local_44 = (srGFInterface *)operator_new(0x28);
    local_4 = 2;
    if (local_44 == (srGFInterface *)0x0) {
      psVar2 = (srGFInterface *)0x0;
    }
    else {
      pcVar8 = *(code **)(param_2 + 0x10);
      if (*(code **)(param_2 + 0x10) == (code *)0x0) {
        pcVar8 = _C_exref;
      }
      psVar2 = (srGFInterface *)srGFInterface::srGFInterface(local_44,(char *)pcVar8);
    }
    local_4 = 0xffffffff;
    if (psVar2 == (srGFInterface *)0x0) {
      pcVar8 = _C_exref;
      if (*(code **)(param_2 + 0x10) != (code *)0x0) {
        pcVar8 = *(code **)(param_2 + 0x10);
      }
      local_4c = 1;
      puVar3 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&local_44);
      local_4 = 3;
      pbVar4 = std::operator<<((basic_ostream<> *)*puVar3,
                               s_Import_error__cant_find_hx_file__10036b68);
      cVar12 = '\0';
      pbVar4 = std::operator<<(pbVar4,(char *)pcVar8);
      std::operator<<(pbVar4,cVar12);
      iVar13 = 0x7fa;
      pcVar11 = s_E__work_ht_3de_3de_cpp_10036b50;
      pcVar5 = Housemarque::Kernel::Error::Tmp_Str();
      Housemarque::Kernel::Throw_Error(s_mesh_file_10036b8c,pcVar5,pcVar11,iVar13);
    }
    local_4 = 0xffffffff;
    if ((local_4c & 1) != 0) {
      Housemarque::Kernel::Inserter::~Inserter((Inserter *)&local_44);
    }
  }
  else {
    local_4c = 2;
    puVar3 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&local_44);
    local_4 = 4;
    pbVar4 = std::operator<<((basic_ostream<> *)*puVar3,s_3de__Import_error__hx_file_not_d_10036bb0)
    ;
    std::operator<<(pbVar4,'\0');
    iVar13 = 0x7fc;
    pcVar11 = s_E__work_ht_3de_3de_cpp_10036b98;
    pcVar5 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(&DAT_10036bdc,pcVar5,pcVar11,iVar13);
    local_4 = 0xffffffff;
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&local_44);
    psVar2 = _uStack00000004;
  }
  srGFInterface::loadMaterials();
  psVar9 = local_50 + 0xf8;
  plVar10 = param_2 + 0x1c;
  local_4c = 8;
  do {
    bVar1 = std::operator!=(*(basic_string<> **)plVar10,s_not_defined_10036be0);
    if (bVar1) {
      fStack_3c = 0.0;
      psStack_38 = (srGFInterface *)0x0;
      uStack_34 = 0;
      plVar6 = *(list<> **)(*(int *)plVar10 + 4);
      if (*(list<> **)(*(int *)plVar10 + 4) == (list<> *)0x0) {
        plVar6 = (list<> *)_C_exref;
      }
      psVar7 = FUN_1001ae40((char *)psVar2,(reOrderData *)_uStack00000004,plVar6,
                            (float *)(local_48 + 4),&fStack_3c,(float *)(param_2 + 0xf4),0);
      *(srMeshModel **)psVar9 = psVar7;
      FUN_1001b140(psVar7,'\0');
    }
    else {
      *(undefined4 *)psVar9 = 0;
    }
    plVar10 = plVar10 + 4;
    psVar9 = psVar9 + 4;
    local_4c = local_4c - 1;
  } while (local_4c != 0);
  bVar1 = std::operator!=((basic_string<> *)(param_2 + 0x3c),s_not_defined_10036bec);
  if (bVar1) {
    fStack_30 = 0.0;
    uStack_2c = 0;
    uStack_28 = 0;
    plVar10 = *(list<> **)(param_2 + 0x40);
    if (*(list<> **)(param_2 + 0x40) == (list<> *)0x0) {
      plVar10 = (list<> *)_C_exref;
    }
    fStack_3c = (float)CONCAT31(fStack_3c._1_3_,uStack00000004);
    psStack_38 = (srGFInterface *)operator_new(0xc);
    *(srGFInterface **)psStack_38 = psStack_38;
    *(srGFInterface **)(psStack_38 + 4) = psStack_38;
    uStack_34 = 0;
    local_4 = 5;
    psVar7 = FUN_1001ae40((char *)psVar2,(reOrderData *)&fStack_3c,plVar10,(float *)(local_48 + 4),
                          &fStack_30,(float *)(param_2 + 0xf4),1);
    psVar9 = psStack_38;
    *(srMeshModel **)(local_50 + 0xf4) = psVar7;
    _uStack00000004 = *(srGFInterface **)psStack_38;
    local_4 = 0xffffffff;
    while (_uStack00000004 != psVar9) {
      puVar3 = (undefined4 *)FUN_10015b00(&stack0x00000004,&local_44);
      FUN_10015ac0(&fStack_3c,(int *)&local_4c,(int *)*puVar3);
    }
    FUN_100212a4(psStack_38);
  }
  else {
    *(undefined4 *)(local_50 + 0xf4) = 0;
  }
  if (psVar2 != (srGFInterface *)0x0) {
    srGFInterface::~srGFInterface(psVar2);
    FUN_100212a4(psVar2);
  }
  *(undefined4 *)(local_50 + 0xc) = 0;
  plVar10 = param_2 + 0x4c;
  psVar9 = local_50 + 0xf8;
  psVar2 = local_50 + 0x34;
  iVar13 = 8;
  do {
    *(int *)(psVar9 + -0xe8) = **(int **)(plVar10 + 0x80);
    Coeff_Info::Coeff_Info((Coeff_Info *)&uStack_24);
    uStack_24 = **(undefined4 **)plVar10;
    uStack_20 = **(undefined4 **)(plVar10 + 0x20);
    puVar3 = *(undefined4 **)(plVar10 + 0x60);
    uStack_1c = **(undefined **)(plVar10 + 0x40);
    uStack_18 = *puVar3;
    uStack_14 = puVar3[1];
    uStack_10 = puVar3[2];
    Coeff_Info::operator=((Coeff_Info *)psVar2);
    if (*(int *)psVar9 != 0) {
      *(int *)(local_50 + 0xc) = *(int *)(local_50 + 0xc) + 1;
    }
    plVar10 = plVar10 + 4;
    psVar9 = psVar9 + 4;
    psVar2 = (srGFInterface *)((Coeff_Info *)psVar2 + 0x18);
    iVar13 = iVar13 + -1;
  } while (iVar13 != 0);
  *(float *)(local_50 + 0x30) = (float)*(int *)(local_40 + 0xec);
  local_50[8] = *(srGFInterface *)(local_40 + 0xf0);
  Housemarque::Template_Library::DLIL_List_Base::Add_Head((DLIL_Node_Base *)(local_48 + 0x3c));
  ExceptionList = pvStack_c;
  return (Mesh_Node *)local_50;
}



// public: __thiscall Housemarque::Threedee_Engine::Engine::Buffer::Buffer(class srGERD *)

Buffer * __thiscall
Housemarque::Threedee_Engine::Engine::Buffer::Buffer(Buffer *this,srGERD *param_1)

{
  srColorSurfaceIFace *psVar1;
  srColorSurface *this_00;
  srColorSurface *psVar2;
  void *unaff_EBX;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0x13dd0  1  ??0Buffer@Engine@Threedee_Engine@Housemarque@@QAE@PAVsrGERD@@@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_1002340b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  psVar1 = srGERD::lockBuffer(param_1);
  this_00 = (srColorSurface *)srHeap::allocate((srHeap *)srHeap_exref,0x5c);
  uStack_4 = 0;
  psVar2 = (srColorSurface *)0x0;
  if (this_00 != (srColorSurface *)0x0) {
    srColorSurface::srColorSurface(this_00,0x18,*(ulong *)(psVar1 + 0x1c),*(ulong *)(psVar1 + 0x20))
    ;
    *(undefined ***)this_00 = &PTR_FUN_10025890;
    psVar2 = this_00;
  }
  *(srColorSurface **)this = psVar2;
  uStack_4 = 0xffffffff;
  (**(code **)(*(int *)psVar2 + 0x88))(psVar1);
  srGERD::unlockBuffer(param_1);
  ExceptionList = unaff_EBX;
  return this;
}



// public: __thiscall Housemarque::Threedee_Engine::Engine::Buffer::~Buffer(void)

void __thiscall Housemarque::Threedee_Engine::Engine::Buffer::~Buffer(Buffer *this)

{
                    // 0x13e70  27  ??1Buffer@Engine@Threedee_Engine@Housemarque@@QAE@XZ
                    // WARNING: Could not recover jumptable at 0x10013e72. Too many branches
                    // WARNING: Treating indirect jump as call
  srClass::release(*(srClass **)this);
  return;
}



// public: void * __fastcall Housemarque::Threedee_Engine::Engine::Buffer::Addr(void)

void * __fastcall Housemarque::Threedee_Engine::Engine::Buffer::Addr(void)

{
  void *pvVar1;
  undefined4 *in_ECX;
  
                    // 0x13e80  52  ?Addr@Buffer@Engine@Threedee_Engine@Housemarque@@QAIPAXXZ
                    // WARNING: Could not recover jumptable at 0x10013e84. Too many branches
                    // WARNING: Treating indirect jump as call
  pvVar1 = (void *)(**(code **)(*(int *)*in_ECX + 0x58))();
  return pvVar1;
}



// public: int __fastcall Housemarque::Threedee_Engine::Engine::Buffer::Pitch(void)const 

int __fastcall Housemarque::Threedee_Engine::Engine::Buffer::Pitch(void)

{
  int *in_ECX;
  
                    // 0x13e90  114  ?Pitch@Buffer@Engine@Threedee_Engine@Housemarque@@QBIHXZ
  return *(int *)(*in_ECX + 0x24);
}



// public: int __fastcall Housemarque::Threedee_Engine::Engine::Buffer::Width(void)const 

int __fastcall Housemarque::Threedee_Engine::Engine::Buffer::Width(void)

{
  int *in_ECX;
  
                    // 0x13ea0  155  ?Width@Buffer@Engine@Threedee_Engine@Housemarque@@QBIHXZ
  return *(int *)(*in_ECX + 0x1c);
}



// public: int __fastcall Housemarque::Threedee_Engine::Engine::Buffer::Height(void)const 

int __fastcall Housemarque::Threedee_Engine::Engine::Buffer::Height(void)

{
  int *in_ECX;
  
                    // 0x13eb0  101  ?Height@Buffer@Engine@Threedee_Engine@Housemarque@@QBIHXZ
  return *(int *)(*in_ECX + 0x20);
}



// public: unsigned int __fastcall
// Housemarque::Threedee_Engine::Engine::Buffer::Bytes_Per_Pixel(void)const 

uint __fastcall Housemarque::Threedee_Engine::Engine::Buffer::Bytes_Per_Pixel(void)

{
  int *in_ECX;
  int iVar1;
  int *piVar2;
  int *piVar3;
  int local_14 [5];
  
                    // 0x13ec0  58
                    // ?Bytes_Per_Pixel@Buffer@Engine@Threedee_Engine@Housemarque@@QBIIXZ
  piVar2 = (int *)(*in_ECX + 0x30);
  piVar3 = local_14;
  for (iVar1 = 5; iVar1 != 0; iVar1 = iVar1 + -1) {
    *piVar3 = *piVar2;
    piVar2 = piVar2 + 1;
    piVar3 = piVar3 + 1;
  }
  return local_14[3] + 1;
}



// public: __thiscall Housemarque::Threedee_Engine::Coeff_Info::Coeff_Info(void)

void __thiscall Housemarque::Threedee_Engine::Coeff_Info::Coeff_Info(Coeff_Info *this)

{
                    // 0x13ee0  6  ??0Coeff_Info@Threedee_Engine@Housemarque@@QAE@XZ
  *(undefined4 *)this = 0x3f800000;
  *(undefined4 *)(this + 4) = 0x3f800000;
  this[8] = (Coeff_Info)0x0;
  *(undefined4 *)(this + 0xc) = 0;
  *(undefined4 *)(this + 0x10) = 0;
  *(undefined4 *)(this + 0x14) = 0;
  return;
}



// public: __thiscall Housemarque::Threedee_Engine::Coeff_Info::Coeff_Info(struct
// Housemarque::Threedee_Engine::Coeff_Info const &)

void __thiscall
Housemarque::Threedee_Engine::Coeff_Info::Coeff_Info(Coeff_Info *this,Coeff_Info *param_1)

{
                    // 0x13f00  5  ??0Coeff_Info@Threedee_Engine@Housemarque@@QAE@ABU012@@Z
  *(undefined4 *)this = *(undefined4 *)param_1;
  *(undefined4 *)(this + 4) = *(undefined4 *)(param_1 + 4);
  this[8] = param_1[8];
  *(undefined4 *)(this + 0xc) = *(undefined4 *)(param_1 + 0xc);
  *(undefined4 *)(this + 0x10) = *(undefined4 *)(param_1 + 0x10);
  *(undefined4 *)(this + 0x14) = *(undefined4 *)(param_1 + 0x14);
  return;
}



// public: struct Housemarque::Threedee_Engine::Coeff_Info & __fastcall
// Housemarque::Threedee_Engine::Coeff_Info::operator=(struct
// Housemarque::Threedee_Engine::Coeff_Info const &)

Coeff_Info * __fastcall Housemarque::Threedee_Engine::Coeff_Info::operator=(Coeff_Info *param_1)

{
  undefined4 *in_EDX;
  
                    // 0x13f40  45  ??4Coeff_Info@Threedee_Engine@Housemarque@@QAIAAU012@ABU012@@Z
  *(undefined4 *)param_1 = *in_EDX;
  *(undefined4 *)(param_1 + 4) = in_EDX[1];
  param_1[8] = *(Coeff_Info *)(in_EDX + 2);
  *(undefined4 *)(param_1 + 0xc) = in_EDX[3];
  *(undefined4 *)(param_1 + 0x10) = in_EDX[4];
  *(undefined4 *)(param_1 + 0x14) = in_EDX[5];
  return param_1;
}



void * __thiscall FUN_10013f70(void *this,byte param_1)

{
  Housemarque::Threedee_Engine::Engine::~Engine((Engine *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void __fastcall FUN_10013f90(int param_1)

{
  char *pcVar1;
  char cVar2;
  
  if (*(int *)(param_1 + 8) != 0) {
    pcVar1 = (char *)(*(int *)(param_1 + 8) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(param_1 + 8) = 0;
  *(undefined4 *)(param_1 + 0xc) = 0;
  *(undefined4 *)(param_1 + 0x10) = 0;
  return;
}



void * __thiscall FUN_10013fd0(void *this,byte param_1)

{
  srCamera::~srCamera((srCamera *)this);
  if ((param_1 & 1) != 0) {
    srHeap::free(this);
  }
  return this;
}



void * __thiscall FUN_10014000(void *this,byte param_1)

{
  srScene::~srScene((srScene *)this);
  if ((param_1 & 1) != 0) {
    srHeap::free(this);
  }
  return this;
}



void * __thiscall
FUN_10014030(void *this,basic_string<> *param_1,Config_Base *param_2,undefined4 param_3)

{
  CPP_Identifier local_1c [16];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10023431;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  Housemarque::Template_Library::CPP_Identifier::CPP_Identifier(local_1c,param_1);
  uStack_4 = 0;
  Housemarque::Template_Library::Item_Ref_Base::Item_Ref_Base
            ((Item_Ref_Base *)this,local_1c,param_2);
  uStack_4 = CONCAT31(uStack_4._1_3_,2);
  Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier(local_1c);
  *(undefined4 *)((int)this + 0x1c) = param_3;
  *(undefined ***)this = &PTR_FUN_10025c24;
  ExceptionList = pvStack_c;
  return this;
}



void __fastcall FUN_100140b0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_10023467;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_10025c34;
  local_4 = 0;
  Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier((CPP_Identifier *)(param_1 + 3));
  ExceptionList = pvStack_c;
  return;
}



void * __thiscall FUN_100140f0(void *this,byte param_1)

{
  FUN_100140b0((undefined4 *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



basic_istream<> * __thiscall FUN_10014120(void *this,basic_istream<> *param_1)

{
  code cVar1;
  basic_string<> *this_00;
  bool bVar2;
  basic_istream<> *pbVar3;
  code *pcVar4;
  uint uVar5;
  uint uVar6;
  code *pcVar7;
  basic_string<> local_1c [4];
  code *local_18;
  uint local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_10023489;
  pvStack_c = ExceptionList;
  local_1c[0] = param_1._0_1_;
  local_18 = (code *)0x0;
  local_14 = 0;
  local_10 = 0;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  pbVar3 = Housemarque::Template_Library::Read_Literal(param_1,local_1c);
  if (pbVar3 == (basic_istream<> *)0x0) {
    pbVar3 = (basic_istream<> *)0x0;
  }
  else {
    pbVar3 = pbVar3 + *(int *)(*(int *)pbVar3 + 4);
  }
  if (((uint)pbVar3 & ~-(uint)((*(uint *)(pbVar3 + 4) & 6) != 0)) != 0) {
    uVar5 = *(uint *)npos_exref;
    this_00 = *(basic_string<> **)((int)this + 0x1c);
    uVar6 = local_14;
    if (uVar5 < local_14) {
      uVar6 = uVar5;
    }
    if (this_00 == local_1c) {
      std::basic_string<>::erase(this_00,uVar6,uVar5);
      std::basic_string<>::erase(this_00,0,0);
    }
    else {
      if ((uVar6 != 0) && (uVar6 == local_14)) {
        pcVar4 = local_18;
        if (local_18 == (code *)0x0) {
          pcVar4 = _C_exref;
        }
        if ((byte)pcVar4[-1] < 0xfe) {
          std::basic_string<>::_Tidy(this_00,true);
          pcVar4 = local_18;
          if (local_18 == (code *)0x0) {
            pcVar4 = _C_exref;
          }
          *(code **)(this_00 + 4) = pcVar4;
          *(uint *)(this_00 + 8) = local_14;
          *(undefined4 *)(this_00 + 0xc) = local_10;
          pcVar4[-1] = (code)((char)pcVar4[-1] + '\x01');
          goto LAB_10014244;
        }
      }
      bVar2 = std::basic_string<>::_Grow(this_00,uVar6,true);
      if (bVar2) {
        pcVar4 = local_18;
        if (local_18 == (code *)0x0) {
          pcVar4 = _C_exref;
        }
        pcVar7 = *(code **)(this_00 + 4);
        for (uVar5 = uVar6 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
          *(undefined4 *)pcVar7 = *(undefined4 *)pcVar4;
          pcVar4 = pcVar4 + 4;
          pcVar7 = pcVar7 + 4;
        }
        for (uVar5 = uVar6 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
          *pcVar7 = *pcVar4;
          pcVar4 = pcVar4 + 1;
          pcVar7 = pcVar7 + 1;
        }
        *(uint *)(this_00 + 8) = uVar6;
        *(undefined *)(*(int *)(this_00 + 4) + uVar6) = 0;
      }
    }
  }
LAB_10014244:
  if (local_18 != (code *)0x0) {
    cVar1 = local_18[-1];
    if ((cVar1 == (code)0x0) || (cVar1 == (code)0xff)) {
      FUN_100212a4(local_18 + -1);
    }
    else {
      local_18[-1] = (code)((char)cVar1 + -1);
    }
  }
  ExceptionList = pvStack_c;
  return param_1;
}



void * __thiscall FUN_10014290(void *this,byte param_1)

{
  FUN_100142b0((undefined4 *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void __fastcall FUN_100142b0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_100234c7;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_10025c34;
  local_4 = 0;
  Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier((CPP_Identifier *)(param_1 + 3));
  ExceptionList = pvStack_c;
  return;
}



void __fastcall FUN_100142f0(int param_1)

{
  char *pcVar1;
  char cVar2;
  
  if (*(int *)(param_1 + 4) != 0) {
    pcVar1 = (char *)(*(int *)(param_1 + 4) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(param_1 + 4) = 0;
  *(undefined4 *)(param_1 + 8) = 0;
  *(undefined4 *)(param_1 + 0xc) = 0;
  return;
}



void * __thiscall
FUN_10014330(void *this,Config_Base *param_1,basic_string<> *param_2,undefined param_3)

{
  FUN_10014460(this,param_1,param_2,&param_3);
  return this;
}



void * __thiscall FUN_10014360(void *this,byte param_1)

{
  Housemarque::Threedee_Engine::Trimesh::~Trimesh((Trimesh *)this);
  if ((param_1 & 1) != 0) {
    srHeap::free(this);
  }
  return this;
}



void * __thiscall FUN_10014390(void *this,byte param_1)

{
  srTextureFile::~srTextureFile((srTextureFile *)this);
  if ((param_1 & 1) != 0) {
    srHeap::free(this);
  }
  return this;
}



void __fastcall FUN_100143f0(int param_1)

{
  if (*(srClass **)(param_1 + 8) != (srClass *)0x0) {
                    // WARNING: Could not recover jumptable at 0x100143f7. Too many branches
                    // WARNING: Treating indirect jump as call
    srClass::release(*(srClass **)(param_1 + 8));
    return;
  }
  return;
}



void __fastcall FUN_10014400(undefined4 *param_1)

{
  *param_1 = 0x100241b;
  return;
}



void * __thiscall FUN_10014410(void *this,byte param_1)

{
  srMaterial::~srMaterial((srMaterial *)this);
  if ((param_1 & 1) != 0) {
    srHeap::free(this);
  }
  return this;
}



void __fastcall FUN_10014440(int param_1)

{
  *(undefined4 *)(param_1 + 0x50) = 0;
  return;
}



void * __thiscall
FUN_10014460(void *this,Config_Base *param_1,basic_string<> *param_2,undefined *param_3)

{
  Item_Ref_Base *this_00;
  CPP_Identifier local_1c [16];
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_100234fc;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(undefined *)this = *param_3;
  this_00 = (Item_Ref_Base *)operator_new(0x20);
  local_4 = 0;
  if (this_00 != (Item_Ref_Base *)0x0) {
    Housemarque::Template_Library::CPP_Identifier::CPP_Identifier(local_1c,param_2);
    local_4._0_1_ = 1;
    Housemarque::Template_Library::Item_Ref_Base::Item_Ref_Base(this_00,local_1c,param_1);
    local_4 = CONCAT31(local_4._1_3_,3);
    Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier(local_1c);
    *(void **)(this_00 + 0x1c) = this;
    *(undefined ***)this_00 = &PTR_FUN_10025d9c;
  }
  ExceptionList = local_c;
  return this;
}



void * __thiscall
FUN_10014500(void *this,Config_Base *param_1,basic_string<> *param_2,undefined4 *param_3)

{
  Item_Ref_Base *this_00;
  CPP_Identifier local_1c [16];
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_1002352c;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(undefined4 *)this = *param_3;
  this_00 = (Item_Ref_Base *)operator_new(0x20);
  local_4 = 0;
  if (this_00 != (Item_Ref_Base *)0x0) {
    Housemarque::Template_Library::CPP_Identifier::CPP_Identifier(local_1c,param_2);
    local_4._0_1_ = 1;
    Housemarque::Template_Library::Item_Ref_Base::Item_Ref_Base(this_00,local_1c,param_1);
    local_4 = CONCAT31(local_4._1_3_,3);
    Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier(local_1c);
    *(void **)(this_00 + 0x1c) = this;
    *(undefined ***)this_00 = &PTR_FUN_10025dac;
  }
  ExceptionList = local_c;
  return this;
}



void * __thiscall
FUN_100145a0(void *this,Config_Base *param_1,basic_string<> *param_2,undefined4 *param_3)

{
  Item_Ref_Base *this_00;
  CPP_Identifier local_1c [16];
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_1002355c;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(undefined4 *)this = *param_3;
  this_00 = (Item_Ref_Base *)operator_new(0x20);
  local_4 = 0;
  if (this_00 != (Item_Ref_Base *)0x0) {
    Housemarque::Template_Library::CPP_Identifier::CPP_Identifier(local_1c,param_2);
    local_4._0_1_ = 1;
    Housemarque::Template_Library::Item_Ref_Base::Item_Ref_Base(this_00,local_1c,param_1);
    local_4 = CONCAT31(local_4._1_3_,3);
    Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier(local_1c);
    *(void **)(this_00 + 0x1c) = this;
    *(undefined ***)this_00 = &PTR_FUN_10025dbc;
  }
  ExceptionList = local_c;
  return this;
}



void * __thiscall
FUN_10014640(void *this,Config_Base *param_1,basic_string<> *param_2,undefined *param_3)

{
  uint uVar1;
  bool bVar2;
  code *pcVar3;
  Item_Ref_Base *this_00;
  uint uVar4;
  uint uVar5;
  code *pcVar6;
  CPP_Identifier aCStack_1c [16];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10023595;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined *)this = *param_3;
  *(undefined4 *)((int)this + 4) = 0;
  *(undefined4 *)((int)this + 8) = 0;
  *(undefined4 *)((int)this + 0xc) = 0;
  uVar4 = *(uint *)npos_exref;
  uVar1 = *(uint *)(param_3 + 8);
  uVar5 = uVar1;
  if (uVar4 < uVar1) {
    uVar5 = uVar4;
  }
  if ((undefined *)this == param_3) {
    std::basic_string<>::erase((basic_string<> *)this,uVar5,uVar4);
    std::basic_string<>::erase((basic_string<> *)this,0,0);
  }
  else {
    if ((uVar5 != 0) && (uVar5 == uVar1)) {
      pcVar3 = *(code **)(param_3 + 4);
      if (*(code **)(param_3 + 4) == (code *)0x0) {
        pcVar3 = _C_exref;
      }
      if ((byte)pcVar3[-1] < 0xfe) {
        std::basic_string<>::_Tidy((basic_string<> *)this,true);
        pcVar3 = *(code **)(param_3 + 4);
        if (*(code **)(param_3 + 4) == (code *)0x0) {
          pcVar3 = _C_exref;
        }
        *(code **)((int)this + 4) = pcVar3;
        *(undefined4 *)((int)this + 8) = *(undefined4 *)(param_3 + 8);
        *(undefined4 *)((int)this + 0xc) = *(undefined4 *)(param_3 + 0xc);
        pcVar3[-1] = (code)((char)pcVar3[-1] + '\x01');
        goto LAB_10014720;
      }
    }
    bVar2 = std::basic_string<>::_Grow((basic_string<> *)this,uVar5,true);
    if (bVar2) {
      pcVar3 = *(code **)(param_3 + 4);
      if (*(code **)(param_3 + 4) == (code *)0x0) {
        pcVar3 = _C_exref;
      }
      pcVar6 = *(code **)((int)this + 4);
      for (uVar4 = uVar5 >> 2; uVar4 != 0; uVar4 = uVar4 - 1) {
        *(undefined4 *)pcVar6 = *(undefined4 *)pcVar3;
        pcVar3 = pcVar3 + 4;
        pcVar6 = pcVar6 + 4;
      }
      for (uVar4 = uVar5 & 3; uVar4 != 0; uVar4 = uVar4 - 1) {
        *pcVar6 = *pcVar3;
        pcVar3 = pcVar3 + 1;
        pcVar6 = pcVar6 + 1;
      }
      std::basic_string<>::_Eos((basic_string<> *)this,uVar5);
    }
  }
LAB_10014720:
  uStack_4 = 0;
  this_00 = (Item_Ref_Base *)operator_new(0x20);
  uStack_4._0_1_ = 1;
  if (this_00 != (Item_Ref_Base *)0x0) {
    Housemarque::Template_Library::CPP_Identifier::CPP_Identifier(aCStack_1c,param_2);
    uStack_4._0_1_ = 2;
    Housemarque::Template_Library::Item_Ref_Base::Item_Ref_Base(this_00,aCStack_1c,param_1);
    uStack_4 = CONCAT31(uStack_4._1_3_,4);
    Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier(aCStack_1c);
    *(void **)(this_00 + 0x1c) = this;
    *(undefined ***)this_00 = &PTR_FUN_10025c24;
  }
  ExceptionList = pvStack_c;
  return this;
}



void __fastcall FUN_100147a0(int param_1)

{
  char *pcVar1;
  char cVar2;
  int iVar3;
  int iVar4;
  
  iVar3 = *(int *)(param_1 + 8);
  for (iVar4 = *(int *)(param_1 + 4); iVar4 != iVar3; iVar4 = iVar4 + 0x10) {
    if (*(int *)(iVar4 + 4) != 0) {
      pcVar1 = (char *)(*(int *)(iVar4 + 4) + -1);
      cVar2 = *pcVar1;
      if ((cVar2 == '\0') || (cVar2 == -1)) {
        FUN_100212a4(pcVar1);
      }
      else {
        *pcVar1 = cVar2 + -1;
      }
    }
    *(undefined4 *)(iVar4 + 4) = 0;
    *(undefined4 *)(iVar4 + 8) = 0;
    *(undefined4 *)(iVar4 + 0xc) = 0;
  }
  FUN_100212a4(*(void **)(param_1 + 4));
  *(undefined4 *)(param_1 + 4) = 0;
  *(undefined4 *)(param_1 + 8) = 0;
  *(undefined4 *)(param_1 + 0xc) = 0;
  return;
}



void __thiscall Housemarque::Template_Library::Config_Base::~Config_Base(Config_Base *this)

{
                    // WARNING: Could not recover jumptable at 0x10014800. Too many branches
                    // WARNING: Treating indirect jump as call
  ~Config_Base(this);
  return;
}



void * __fastcall FUN_100148d0(srModelInstance *param_1)

{
  void *this;
  
  this = (void *)(**(code **)(*(int *)param_1 + 0x20))();
  FUN_10010cd0(this,param_1);
  return this;
}



void __fastcall FUN_100148f0(srModelInstance *param_1)

{
  srRegistry *this;
  srRegistry *this_00;
  srRegistry *this_01;
  ClassNode *pCVar1;
  char *pcVar2;
  ulong uVar3;
  int iVar4;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_100235a9;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined ***)param_1 = &PTR_FUN_10025cbc;
  *(undefined ***)(param_1 + 0x138) = &PTR_LAB_10025ca8;
  this = *(srRegistry **)(srCore_exref + 0x2c);
  local_4 = 0;
  pCVar1 = srRegistry::getClassNode(this,0x38888);
  if (pCVar1 == (ClassNode *)0x0) {
    this_00 = *(srRegistry **)(srCore_exref + 0x2c);
    pCVar1 = srRegistry::getClassNode(this_00,0x1100);
    if (pCVar1 == (ClassNode *)0x0) {
      this_01 = *(srRegistry **)(srCore_exref + 0x2c);
      pCVar1 = srRegistry::getClassNode(this_01,0x1000);
      if (pCVar1 == (ClassNode *)0x0) {
        iVar4 = 1;
        uVar3 = 0x1000;
        pCVar1 = srClass::sGetClassNode();
        pcVar2 = srNode::sGetClassName();
        pCVar1 = srRegistry::registerClass(this_01,pcVar2,pCVar1,uVar3,iVar4);
      }
      pCVar1 = srRegistry::registerClass(this_00,s_srModelInstance_10037024,pCVar1,0x1100,0);
    }
    pCVar1 = srRegistry::registerClass(this,s_srModelInstance_10037024,pCVar1,0x38888,0);
  }
  srRegistry::unregisterInstance(this,pCVar1,(srRuntimeClass *)param_1);
  local_4 = 0xffffffff;
  srModelInstance::~srModelInstance(param_1);
  ExceptionList = pvStack_c;
  return;
}



void __fastcall FUN_100149f0(int param_1)

{
  FUN_100212a4(*(void **)(param_1 + 4));
  *(undefined4 *)(param_1 + 4) = 0;
  *(undefined4 *)(param_1 + 8) = 0;
  *(undefined4 *)(param_1 + 0xc) = 0;
  return;
}



int __fastcall FUN_10014a20(int param_1)

{
  if (*(int *)(param_1 + 4) == 0) {
    return 0;
  }
  return *(int *)(param_1 + 8) - *(int *)(param_1 + 4) >> 2;
}



void __fastcall FUN_10014a80(undefined4 *param_1)

{
  if ((srClass *)*param_1 != (srClass *)0x0) {
                    // WARNING: Could not recover jumptable at 0x10014a86. Too many branches
                    // WARNING: Treating indirect jump as call
    srClass::release((srClass *)*param_1);
    return;
  }
  return;
}



srMaterial * __fastcall FUN_10014b20(srMaterial *param_1)

{
  srMaterial *this;
  
  this = (srMaterial *)(**(code **)(*(int *)param_1 + 0x20))();
  srMaterial::operator=(this,param_1);
  return this;
}



srClass * __fastcall FUN_10014b40(srClass *param_1)

{
  srRegistry *psVar1;
  srRegistry *this;
  ClassNode *pCVar2;
  char *pcVar3;
  ulong uVar4;
  int iVar5;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_100235d2;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  srClass::srClass(param_1);
  *(undefined ***)param_1 = &PTR_FUN_10025e04;
  psVar1 = *(srRegistry **)(srCore_exref + 0x2c);
  uStack_4 = 0;
  pCVar2 = srRegistry::getClassNode(psVar1,0x2200);
  if (pCVar2 == (ClassNode *)0x0) {
    iVar5 = 1;
    uVar4 = 0x2200;
    pCVar2 = srClass::sGetClassNode();
    pcVar3 = srMaterialIFace::sGetClassName();
    pCVar2 = srRegistry::registerClass(psVar1,pcVar3,pCVar2,uVar4,iVar5);
  }
  srRegistry::registerInstance(psVar1,pCVar2,(srRuntimeClass *)param_1);
  *(undefined ***)param_1 = &PTR_FUN_10025dcc;
  psVar1 = *(srRegistry **)(srCore_exref + 0x2c);
  uStack_4 = 1;
  pCVar2 = srRegistry::getClassNode(psVar1,0x2210);
  if (pCVar2 == (ClassNode *)0x0) {
    this = *(srRegistry **)(srCore_exref + 0x2c);
    pCVar2 = srRegistry::getClassNode(this,0x2200);
    if (pCVar2 == (ClassNode *)0x0) {
      iVar5 = 1;
      uVar4 = 0x2200;
      pCVar2 = srClass::sGetClassNode();
      pcVar3 = srMaterialIFace::sGetClassName();
      pCVar2 = srRegistry::registerClass(this,pcVar3,pCVar2,uVar4,iVar5);
    }
    iVar5 = 0;
    uVar4 = 0x2210;
    pcVar3 = srMaterial::sGetClassName();
    pCVar2 = srRegistry::registerClass(psVar1,pcVar3,pCVar2,uVar4,iVar5);
  }
  srRegistry::registerInstance(psVar1,pCVar2,(srRuntimeClass *)param_1);
  ExceptionList = pvStack_c;
  return param_1;
}



srMaterialIFace * __fastcall FUN_10014cb0(srMaterialIFace *param_1)

{
  srMaterialIFace *this;
  
  this = (srMaterialIFace *)(**(code **)(*(int *)param_1 + 0x20))();
  srMaterialIFace::operator=(this,param_1);
  return this;
}



void __fastcall FUN_10014cd0(srClass *param_1)

{
  srRegistry *psVar1;
  srRegistry *this;
  ClassNode *pCVar2;
  char *pcVar3;
  ulong uVar4;
  int iVar5;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_100235f2;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined ***)param_1 = &PTR_FUN_10025dcc;
  psVar1 = *(srRegistry **)(srCore_exref + 0x2c);
  local_4 = 0;
  pCVar2 = srRegistry::getClassNode(psVar1,0x2210);
  if (pCVar2 == (ClassNode *)0x0) {
    this = *(srRegistry **)(srCore_exref + 0x2c);
    pCVar2 = srRegistry::getClassNode(this,0x2200);
    if (pCVar2 == (ClassNode *)0x0) {
      iVar5 = 1;
      uVar4 = 0x2200;
      pCVar2 = srClass::sGetClassNode();
      pcVar3 = srMaterialIFace::sGetClassName();
      pCVar2 = srRegistry::registerClass(this,pcVar3,pCVar2,uVar4,iVar5);
    }
    iVar5 = 0;
    uVar4 = 0x2210;
    pcVar3 = srMaterial::sGetClassName();
    pCVar2 = srRegistry::registerClass(psVar1,pcVar3,pCVar2,uVar4,iVar5);
  }
  srRegistry::unregisterInstance(psVar1,pCVar2,(srRuntimeClass *)param_1);
  *(undefined ***)param_1 = &PTR_FUN_10025e04;
  psVar1 = *(srRegistry **)(srCore_exref + 0x2c);
  local_4 = 1;
  pCVar2 = srRegistry::getClassNode(psVar1,0x2200);
  if (pCVar2 == (ClassNode *)0x0) {
    iVar5 = 1;
    uVar4 = 0x2200;
    pCVar2 = srClass::sGetClassNode();
    pcVar3 = srMaterialIFace::sGetClassName();
    pCVar2 = srRegistry::registerClass(psVar1,pcVar3,pCVar2,uVar4,iVar5);
  }
  srRegistry::unregisterInstance(psVar1,pCVar2,(srRuntimeClass *)param_1);
  local_4 = 0xffffffff;
  srClass::~srClass(param_1);
  ExceptionList = pvStack_c;
  return;
}



srCamera * __fastcall FUN_10014e70(srCamera *param_1)

{
  srCamera *this;
  
  this = (srCamera *)(**(code **)(*(int *)param_1 + 0x20))();
  srCamera::operator=(this,param_1);
  return this;
}



srScene * __fastcall FUN_10014f20(srScene *param_1)

{
  srScene *this;
  
  this = (srScene *)(**(code **)(*(int *)param_1 + 0x20))();
  srScene::operator=(this,param_1);
  return this;
}



srTextureFile * __fastcall FUN_10015000(srTextureFile *param_1)

{
  srTextureFile *this;
  
  this = (srTextureFile *)(**(code **)(*(int *)param_1 + 0x20))();
  srTextureFile::operator=(this,param_1);
  return this;
}



void * __thiscall FUN_10015030(void *this,byte param_1)

{
  FUN_100148f0((srModelInstance *)this);
  if ((param_1 & 1) != 0) {
    srHeap::free(this);
  }
  return this;
}



void * __thiscall FUN_10015060(void *this,byte param_1)

{
  FUN_10015090((srClass *)this);
  if ((param_1 & 1) != 0) {
    srHeap::free(this);
  }
  return this;
}



void __fastcall FUN_10015090(srClass *param_1)

{
  srRegistry *this;
  ClassNode *pCVar1;
  char *pcVar2;
  ulong uVar3;
  int iVar4;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_10023609;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined ***)param_1 = &PTR_FUN_10025e04;
  this = *(srRegistry **)(srCore_exref + 0x2c);
  local_4 = 0;
  pCVar1 = srRegistry::getClassNode(this,0x2200);
  if (pCVar1 == (ClassNode *)0x0) {
    iVar4 = 1;
    uVar3 = 0x2200;
    pCVar1 = srClass::sGetClassNode();
    pcVar2 = srMaterialIFace::sGetClassName();
    pCVar1 = srRegistry::registerClass(this,pcVar2,pCVar1,uVar3,iVar4);
  }
  srRegistry::unregisterInstance(this,pCVar1,(srRuntimeClass *)param_1);
  local_4 = 0xffffffff;
  srClass::~srClass(param_1);
  ExceptionList = pvStack_c;
  return;
}



void * __thiscall FUN_10015120(void *this,byte param_1)

{
  char *pcVar1;
  char cVar2;
  
  if (*(int *)((int)this + 4) != 0) {
    pcVar1 = (char *)(*(int *)((int)this + 4) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)((int)this + 4) = 0;
  *(undefined4 *)((int)this + 8) = 0;
  *(undefined4 *)((int)this + 0xc) = 0;
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void * __thiscall FUN_10015180(void *this,byte param_1)

{
  FUN_10014cd0((srClass *)this);
  if ((param_1 & 1) != 0) {
    srHeap::free(this);
  }
  return this;
}



void FUN_100151b0(void *param_1)

{
  FUN_100212a4(param_1);
  return;
}



void FUN_100151c0(int param_1,int param_2)

{
  char *pcVar1;
  char cVar2;
  
  if (param_1 != param_2) {
    do {
      if (*(int *)(param_1 + 4) != 0) {
        pcVar1 = (char *)(*(int *)(param_1 + 4) + -1);
        cVar2 = *pcVar1;
        if ((cVar2 == '\0') || (cVar2 == -1)) {
          FUN_100212a4(pcVar1);
        }
        else {
          *pcVar1 = cVar2 + -1;
        }
      }
      *(undefined4 *)(param_1 + 4) = 0;
      *(undefined4 *)(param_1 + 8) = 0;
      *(undefined4 *)(param_1 + 0xc) = 0;
      param_1 = param_1 + 0x10;
    } while (param_1 != param_2);
  }
  return;
}



void FUN_10015210(void)

{
  return;
}



undefined4 * FUN_10015220(undefined4 *param_1,undefined4 *param_2,undefined4 *param_3)

{
  if (param_1 == param_2) {
    return param_3;
  }
  do {
    if (param_3 != (undefined4 *)0x0) {
      *param_3 = *param_1;
    }
    param_1 = param_1 + 1;
    param_3 = param_3 + 1;
  } while (param_1 != param_2);
  return param_3;
}



void FUN_10015250(void)

{
  srRegistry *this;
  srRegistry *this_00;
  ClassNode *pCVar1;
  char *pcVar2;
  ulong uVar3;
  int iVar4;
  
  this = *(srRegistry **)(srCore_exref + 0x2c);
  pCVar1 = srRegistry::getClassNode(this,0x2110);
  if (pCVar1 == (ClassNode *)0x0) {
    this_00 = *(srRegistry **)(srCore_exref + 0x2c);
    pCVar1 = srRegistry::getClassNode(this_00,0x2100);
    if (pCVar1 == (ClassNode *)0x0) {
      iVar4 = 1;
      uVar3 = 0x2100;
      pCVar1 = srClass::sGetClassNode();
      pCVar1 = srRegistry::registerClass(this_00,s_srTextureIFace_10037530,pCVar1,uVar3,iVar4);
    }
    iVar4 = 0;
    uVar3 = 0x2110;
    pcVar2 = srTexture::sGetClassName();
    srRegistry::registerClass(this,pcVar2,pCVar1,uVar3,iVar4);
  }
  return;
}



int __thiscall AddResolutionPairToGui16Bit(void *this,undefined4 *param_1,undefined4 *param_2)

{
  int iVar1;
  undefined4 *puVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  undefined4 *puVar6;
  uint uVar7;
  undefined4 *puVar8;
  
  iVar5 = *(int *)((int)this + 4);
  iVar3 = (int)param_1 - iVar5;
  puVar6 = *(undefined4 **)((int)this + 8);
  iVar4 = *(int *)((int)this + 0xc) - (int)puVar6;
  iVar1 = iVar4 >> 0x1f;
  if (iVar4 / 0xc + iVar1 == iVar1) {
    if ((iVar5 == 0) || (uVar7 = ((int)puVar6 - iVar5) / 0xc, uVar7 < 2)) {
      uVar7 = 1;
    }
    if (iVar5 == 0) {
      iVar5 = 0;
    }
    else {
      iVar5 = ((int)puVar6 - iVar5) / 0xc;
    }
    iVar5 = iVar5 + uVar7;
    iVar1 = iVar5;
    if (iVar5 < 0) {
      iVar1 = 0;
    }
    puVar2 = (undefined4 *)operator_new(iVar1 * 0xc);
    puVar8 = puVar2;
    for (puVar6 = *(undefined4 **)((int)this + 4); puVar6 != param_1; puVar6 = puVar6 + 3) {
      FUN_10016880(puVar8,puVar6);
      puVar8 = puVar8 + 3;
    }
    FUN_10016880(puVar8,param_2);
    FUN_10016240(param_1,*(undefined4 **)((int)this + 8),puVar8 + 3);
    FUN_10016230();
    FUN_100212a4(*(void **)((int)this + 4));
    *(undefined4 **)((int)this + 0xc) = puVar2 + iVar5 * 3;
    iVar5 = FUN_10016210((int)this);
    *(undefined4 **)((int)this + 8) = puVar2 + (iVar5 + 1) * 3;
    *(undefined4 **)((int)this + 4) = puVar2;
  }
  else {
    iVar5 = (int)puVar6 - (int)param_1 >> 0x1f;
    if (((int)puVar6 - (int)param_1) / 0xc + iVar5 == iVar5) {
      FUN_10016240(param_1,puVar6,param_1 + 3);
      FUN_10016280(*(undefined4 **)((int)this + 8),
                   1 - ((int)*(undefined4 **)((int)this + 8) - (int)param_1) / 0xc,param_2);
      puVar6 = *(undefined4 **)((int)this + 8);
      if (param_1 != puVar6) {
        do {
          *param_1 = *param_2;
          param_1[1] = param_2[1];
          puVar8 = param_1 + 3;
          param_1[2] = param_2[2];
          param_1 = puVar8;
        } while (puVar8 != puVar6);
      }
    }
    else {
      FUN_10016240(puVar6 + -3,puVar6,puVar6);
      puVar6 = *(undefined4 **)((int)this + 8);
      puVar8 = *(undefined4 **)((int)this + 8) + -3;
      while (param_1 != puVar8) {
        puVar6[-3] = puVar8[-3];
        puVar6[-2] = puVar8[-2];
        puVar6[-1] = puVar8[-1];
        puVar6 = puVar6 + -3;
        puVar8 = puVar8 + -3;
      }
      puVar6 = param_1 + 3;
      if (param_1 != puVar6) {
        do {
          *param_1 = *param_2;
          param_1[1] = param_2[1];
          puVar8 = param_1 + 3;
          param_1[2] = param_2[2];
          param_1 = puVar8;
        } while (puVar8 != puVar6);
      }
    }
    *(int *)((int)this + 8) = *(int *)((int)this + 8) + 0xc;
  }
  return *(int *)((int)this + 4) + (iVar3 / 0xc) * 0xc;
}



void * __thiscall
FUN_100155a0(void *this,basic_string<> *param_1,Config_Base *param_2,undefined4 param_3)

{
  CPP_Identifier local_1c [16];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10023631;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  Housemarque::Template_Library::CPP_Identifier::CPP_Identifier(local_1c,param_1);
  uStack_4 = 0;
  Housemarque::Template_Library::Item_Ref_Base::Item_Ref_Base
            ((Item_Ref_Base *)this,local_1c,param_2);
  uStack_4 = CONCAT31(uStack_4._1_3_,2);
  Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier(local_1c);
  *(undefined4 *)((int)this + 0x1c) = param_3;
  *(undefined ***)this = &PTR_FUN_10025c44;
  ExceptionList = pvStack_c;
  return this;
}



void * __thiscall FUN_10015640(void *this,byte param_1)

{
  FUN_10015660((undefined4 *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void __fastcall FUN_10015660(undefined4 *param_1)

{
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_10023667;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_10025c34;
  local_4 = 0;
  Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier((CPP_Identifier *)(param_1 + 3));
  ExceptionList = pvStack_c;
  return;
}



void * __thiscall FUN_100156b0(void *this,byte param_1)

{
  FUN_100156d0((undefined4 *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void __fastcall FUN_100156d0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_100236a7;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_10025c34;
  local_4 = 0;
  Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier((CPP_Identifier *)(param_1 + 3));
  ExceptionList = pvStack_c;
  return;
}



void * __thiscall FUN_10015720(void *this,byte param_1)

{
  FUN_10015740((undefined4 *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void __fastcall FUN_10015740(undefined4 *param_1)

{
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_100236e7;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_10025c34;
  local_4 = 0;
  Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier((CPP_Identifier *)(param_1 + 3));
  ExceptionList = pvStack_c;
  return;
}



void * __thiscall FUN_10015790(void *this,byte param_1)

{
  FUN_100157b0((undefined4 *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void __fastcall FUN_100157b0(undefined4 *param_1)

{
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_10023727;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *param_1 = &PTR_FUN_10025c34;
  local_4 = 0;
  Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier((CPP_Identifier *)(param_1 + 3));
  ExceptionList = pvStack_c;
  return;
}



void __thiscall
FUN_10015800(void *this,basic_string<> *param_1,basic_string<> *param_2,basic_string<> *param_3)

{
  basic_string<> *pbVar1;
  basic_string<> *pbVar2;
  basic_string<> *pbVar3;
  basic_string<> *pbVar4;
  basic_string<> *pbVar5;
  int iVar6;
  basic_string<> *pbVar7;
  int iVar8;
  bool bVar9;
  
  pbVar1 = param_2;
  pbVar3 = *(basic_string<> **)((int)this + 8);
  if (param_2 <= (basic_string<> *)(*(int *)((int)this + 0xc) - (int)pbVar3 >> 4)) {
    if ((basic_string<> *)((int)pbVar3 - (int)param_1 >> 4) < param_2) {
      iVar6 = (int)param_2 * 0x10;
      param_2 = param_1 + iVar6;
      pbVar2 = param_1;
      if (param_1 != pbVar3) {
        do {
          FUN_10016560(param_2,pbVar2);
          pbVar2 = pbVar2 + 0x10;
          param_2 = param_2 + 0x10;
        } while (pbVar2 != pbVar3);
      }
      pbVar3 = *(basic_string<> **)((int)this + 8);
      for (iVar8 = (int)pbVar1 - ((int)pbVar3 - (int)param_1 >> 4); iVar8 != 0; iVar8 = iVar8 + -1)
      {
        FUN_10016560(pbVar3,param_3);
        pbVar3 = pbVar3 + 0x10;
      }
      pbVar3 = *(basic_string<> **)((int)this + 8);
      if (param_1 != pbVar3) {
        do {
          std::basic_string<>::assign(param_1,param_3,0,*(uint *)npos_exref);
          param_1 = param_1 + 0x10;
        } while (param_1 != pbVar3);
        *(int *)((int)this + 8) = *(int *)((int)this + 8) + iVar6;
        return;
      }
    }
    else {
      if (param_2 == (basic_string<> *)0x0) {
        return;
      }
      pbVar2 = pbVar3;
      for (pbVar5 = pbVar3 + (int)param_2 * -0x10; pbVar5 != pbVar3; pbVar5 = pbVar5 + 0x10) {
        FUN_10016560(pbVar2,pbVar5);
        pbVar2 = pbVar2 + 0x10;
      }
      pbVar3 = *(basic_string<> **)((int)this + 8);
      pbVar2 = pbVar3 + (int)param_2 * -0x10;
      while (param_1 != pbVar2) {
        pbVar2 = pbVar2 + -0x10;
        pbVar3 = pbVar3 + -0x10;
        std::basic_string<>::assign(pbVar3,pbVar2,0,*(uint *)npos_exref);
      }
      pbVar3 = param_1 + (int)param_2 * 0x10;
      if (param_1 != pbVar3) {
        do {
          std::basic_string<>::assign(param_1,param_3,0,*(uint *)npos_exref);
          param_1 = param_1 + 0x10;
        } while (param_1 != pbVar3);
      }
    }
    *(int *)((int)this + 8) = *(int *)((int)this + 8) + (int)pbVar1 * 0x10;
    return;
  }
  iVar6 = *(int *)((int)this + 4);
  if ((iVar6 == 0) || (pbVar2 = (basic_string<> *)((int)pbVar3 - iVar6 >> 4), pbVar2 <= param_2)) {
    pbVar2 = param_2;
  }
  if (iVar6 == 0) {
    iVar6 = 0;
  }
  else {
    iVar6 = (int)pbVar3 - iVar6 >> 4;
  }
  pbVar2 = pbVar2 + iVar6;
  pbVar3 = pbVar2;
  if ((int)pbVar2 < 0) {
    pbVar3 = (basic_string<> *)0x0;
  }
  pbVar4 = (basic_string<> *)operator_new((int)pbVar3 << 4);
  pbVar5 = pbVar4;
  for (pbVar3 = *(basic_string<> **)((int)this + 4); pbVar3 != param_1; pbVar3 = pbVar3 + 0x10) {
    FUN_10016560(pbVar5,pbVar3);
    pbVar5 = pbVar5 + 0x10;
  }
  pbVar3 = pbVar5;
  if (param_2 != (basic_string<> *)0x0) {
    do {
      FUN_10016560(pbVar3,param_3);
      param_2 = param_2 + -1;
      pbVar3 = pbVar3 + 0x10;
    } while (param_2 != (basic_string<> *)0x0);
  }
  pbVar3 = *(basic_string<> **)((int)this + 8);
  bVar9 = param_1 != pbVar3;
  pbVar7 = param_1;
  param_1 = pbVar5 + (int)pbVar1 * 0x10;
  if (bVar9) {
    do {
      FUN_10016560(param_1,pbVar7);
      pbVar7 = pbVar7 + 0x10;
      param_1 = param_1 + 0x10;
    } while (pbVar7 != pbVar3);
  }
  pbVar3 = *(basic_string<> **)((int)this + 8);
  for (pbVar5 = *(basic_string<> **)((int)this + 4); pbVar5 != pbVar3; pbVar5 = pbVar5 + 0x10) {
    std::basic_string<>::_Tidy(pbVar5,true);
  }
  FUN_100212a4(*(void **)((int)this + 4));
  *(basic_string<> **)((int)this + 0xc) = pbVar4 + (int)pbVar2 * 0x10;
  if (*(int *)((int)this + 4) != 0) {
    *(basic_string<> **)((int)this + 8) =
         pbVar4 + (int)(pbVar1 + (*(int *)((int)this + 8) - *(int *)((int)this + 4) >> 4)) * 0x10;
    *(basic_string<> **)((int)this + 4) = pbVar4;
    return;
  }
  *(basic_string<> **)((int)this + 8) = pbVar4 + (int)pbVar1 * 0x10;
  *(basic_string<> **)((int)this + 4) = pbVar4;
  return;
}



void __thiscall FUN_10015ac0(void *this,int *param_1,int *param_2)

{
  int iVar1;
  
  iVar1 = *param_2;
  *(int *)param_2[1] = *param_2;
  *(int *)(*param_2 + 4) = param_2[1];
  FUN_100212a4(param_2);
  *(int *)((int)this + 8) = *(int *)((int)this + 8) + -1;
  *param_1 = iVar1;
  return;
}



void __thiscall FUN_10015b00(void *this,undefined4 *param_1)

{
  undefined4 *puVar1;
  
                    // WARNING: Load size is inaccurate
  puVar1 = *this;
  *(undefined4 *)this = *puVar1;
  *param_1 = puVar1;
  return;
}



void __thiscall FUN_10015b10(void *this,undefined4 *param_1,undefined4 *param_2,undefined4 *param_3)

{
  int iVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  basic_string<> *this_00;
  int iVar4;
  basic_string<> *pbVar5;
  undefined4 *puVar6;
  undefined4 *puVar7;
  int iVar8;
  undefined4 *puVar9;
  bool bVar10;
  
  puVar9 = param_2;
  puVar6 = *(undefined4 **)((int)this + 8);
  if (param_2 <= (undefined4 *)((*(int *)((int)this + 0xc) - (int)puVar6) / 0x14)) {
    if (param_2 <= (undefined4 *)(((int)puVar6 - (int)param_1) / 0x14)) {
      if (param_2 != (undefined4 *)0x0) {
        puVar9 = puVar6;
        for (puVar3 = puVar6 + (int)param_2 * -5; puVar3 != puVar6; puVar3 = puVar3 + 5) {
          FUN_10016700(puVar9,puVar3);
          puVar9 = puVar9 + 5;
        }
        puVar6 = *(undefined4 **)((int)this + 8);
        puVar9 = puVar6 + (int)param_2 * -5;
        if (param_1 != puVar9) {
          this_00 = (basic_string<> *)(puVar6 + 1);
          pbVar5 = (basic_string<> *)(puVar9 + 1);
          do {
            puVar3 = puVar9 + -5;
            puVar9 = puVar9 + -5;
            puVar6 = puVar6 + -5;
            *puVar6 = *puVar3;
            pbVar5 = pbVar5 + -0x14;
            this_00 = this_00 + -0x14;
            std::basic_string<>::assign(this_00,pbVar5,0,*(uint *)npos_exref);
          } while (puVar9 != param_1);
        }
        puVar6 = param_1 + (int)param_2 * 5;
        if (param_1 != puVar6) {
          do {
            *param_1 = *param_3;
            std::basic_string<>::assign
                      ((basic_string<> *)(param_1 + 1),(basic_string<> *)(param_3 + 1),0,
                       *(uint *)npos_exref);
            param_1 = param_1 + 5;
          } while (param_1 != puVar6);
        }
        *(int *)((int)this + 8) = *(int *)((int)this + 8) + (int)param_2 * 0x14;
      }
      return;
    }
    iVar4 = (int)param_2 * 0x14;
    param_2 = param_1 + (int)param_2 * 5;
    puVar3 = param_1;
    if (param_1 != puVar6) {
      do {
        FUN_10016700(param_2,puVar3);
        puVar3 = puVar3 + 5;
        param_2 = param_2 + 5;
      } while (puVar3 != puVar6);
    }
    puVar6 = *(undefined4 **)((int)this + 8);
    for (iVar1 = (int)puVar9 - ((int)puVar6 - (int)param_1) / 0x14; iVar1 != 0; iVar1 = iVar1 + -1)
    {
      FUN_10016700(puVar6,param_3);
      puVar6 = puVar6 + 5;
    }
    puVar6 = *(undefined4 **)((int)this + 8);
    if (param_1 != puVar6) {
      do {
        *param_1 = *param_3;
        std::basic_string<>::assign
                  ((basic_string<> *)(param_1 + 1),(basic_string<> *)(param_3 + 1),0,
                   *(uint *)npos_exref);
        param_1 = param_1 + 5;
      } while (param_1 != puVar6);
    }
    *(int *)((int)this + 8) = *(int *)((int)this + 8) + iVar4;
    return;
  }
  iVar4 = *(int *)((int)this + 4);
  if ((iVar4 == 0) || (puVar3 = (undefined4 *)(((int)puVar6 - iVar4) / 0x14), puVar3 <= param_2)) {
    puVar3 = param_2;
  }
  if (iVar4 == 0) {
    iVar4 = 0;
  }
  else {
    iVar4 = ((int)puVar6 - iVar4) / 0x14;
  }
  iVar4 = iVar4 + (int)puVar3;
  iVar1 = iVar4;
  if (iVar4 < 0) {
    iVar1 = 0;
  }
  puVar2 = (undefined4 *)operator_new(iVar1 * 0x14);
  puVar3 = puVar2;
  for (puVar6 = *(undefined4 **)((int)this + 4); puVar6 != param_1; puVar6 = puVar6 + 5) {
    FUN_10016700(puVar3,puVar6);
    puVar3 = puVar3 + 5;
  }
  puVar6 = puVar3;
  if (param_2 != (undefined4 *)0x0) {
    do {
      FUN_10016700(puVar6,param_3);
      param_2 = (undefined4 *)((int)param_2 + -1);
      puVar6 = puVar6 + 5;
    } while (param_2 != (undefined4 *)0x0);
  }
  puVar6 = *(undefined4 **)((int)this + 8);
  bVar10 = param_1 != puVar6;
  puVar7 = param_1;
  param_1 = puVar3 + (int)puVar9 * 5;
  if (bVar10) {
    do {
      FUN_10016700(param_1,puVar7);
      puVar7 = puVar7 + 5;
      param_1 = param_1 + 5;
    } while (puVar7 != puVar6);
  }
  iVar1 = *(int *)((int)this + 8);
  for (iVar8 = *(int *)((int)this + 4); iVar8 != iVar1; iVar8 = iVar8 + 0x14) {
    FUN_10013f90(iVar8);
  }
  FUN_100212a4(*(void **)((int)this + 4));
  iVar1 = *(int *)((int)this + 4);
  *(undefined4 **)((int)this + 0xc) = puVar2 + iVar4 * 5;
  if (iVar1 != 0) {
    *(undefined4 **)((int)this + 4) = puVar2;
    *(undefined4 **)((int)this + 8) =
         puVar2 + ((int)puVar9 + (*(int *)((int)this + 8) - iVar1) / 0x14) * 5;
    return;
  }
  *(undefined4 **)((int)this + 4) = puVar2;
  *(undefined4 **)((int)this + 8) = puVar2 + (int)puVar9 * 5;
  return;
}



void __thiscall
AddResolutionPairToGui32Bit(void *this,undefined4 *param_1,uint param_2,undefined4 *param_3)

{
  uint uVar1;
  int iVar2;
  undefined4 *puVar3;
  uint uVar4;
  int iVar5;
  undefined4 *puVar6;
  undefined4 *puVar7;
  undefined4 *puVar8;
  bool bVar9;
  
  uVar1 = param_2;
  puVar6 = param_1;
  puVar7 = *(undefined4 **)((int)this + 8);
  if (param_2 <= (uint)((*(int *)((int)this + 0xc) - (int)puVar7) / 0xc)) {
    if (param_2 <= (uint)(((int)puVar7 - (int)param_1) / 0xc)) {
      if (param_2 != 0) {
        puVar6 = puVar7;
        for (puVar3 = puVar7 + param_2 * -3; puVar3 != puVar7; puVar3 = puVar3 + 3) {
          FUN_10016880(puVar6,puVar3);
          puVar6 = puVar6 + 3;
        }
        puVar7 = *(undefined4 **)((int)this + 8);
        puVar6 = *(undefined4 **)((int)this + 8) + param_2 * -3;
        while (param_1 != puVar6) {
          puVar7[-3] = puVar6[-3];
          puVar7[-2] = puVar6[-2];
          puVar7[-1] = puVar6[-1];
          puVar7 = puVar7 + -3;
          puVar6 = puVar6 + -3;
        }
        puVar7 = param_1;
        if (param_1 != param_1 + param_2 * 3) {
          do {
            *puVar7 = *param_3;
            puVar7[1] = param_3[1];
            puVar6 = puVar7 + 3;
            puVar7[2] = param_3[2];
            puVar7 = puVar6;
          } while (puVar6 != param_1 + param_2 * 3);
        }
        *(int *)((int)this + 8) = *(int *)((int)this + 8) + param_2 * 0xc;
      }
      return;
    }
    bVar9 = param_1 != puVar7;
    puVar3 = param_1;
    param_1 = param_1 + param_2 * 3;
    if (bVar9) {
      do {
        FUN_10016880(param_1,puVar3);
        puVar3 = puVar3 + 3;
        param_1 = param_1 + 3;
      } while (puVar3 != puVar7);
    }
    puVar7 = *(undefined4 **)((int)this + 8);
    for (iVar5 = param_2 - ((int)puVar7 - (int)puVar6) / 0xc; iVar5 != 0; iVar5 = iVar5 + -1) {
      FUN_10016880(puVar7,param_3);
      puVar7 = puVar7 + 3;
    }
    puVar7 = *(undefined4 **)((int)this + 8);
    if (puVar6 != puVar7) {
      do {
        *puVar6 = *param_3;
        puVar6[1] = param_3[1];
        puVar3 = puVar6 + 3;
        puVar6[2] = param_3[2];
        puVar6 = puVar3;
      } while (puVar3 != puVar7);
    }
    *(uint *)((int)this + 8) = *(int *)((int)this + 8) + param_2 * 0xc;
    return;
  }
  iVar5 = *(int *)((int)this + 4);
  if ((iVar5 == 0) || (uVar4 = ((int)puVar7 - iVar5) / 0xc, uVar4 <= param_2)) {
    uVar4 = param_2;
  }
  if (iVar5 == 0) {
    iVar5 = 0;
  }
  else {
    iVar5 = ((int)puVar7 - iVar5) / 0xc;
  }
  iVar5 = iVar5 + uVar4;
  iVar2 = iVar5;
  if (iVar5 < 0) {
    iVar2 = 0;
  }
  puVar3 = (undefined4 *)operator_new(iVar2 * 0xc);
  puVar6 = puVar3;
  for (puVar7 = *(undefined4 **)((int)this + 4); puVar7 != param_1; puVar7 = puVar7 + 3) {
    FUN_10016880(puVar6,puVar7);
    puVar6 = puVar6 + 3;
  }
  puVar7 = puVar6;
  if (param_2 != 0) {
    do {
      FUN_10016880(puVar7,param_3);
      param_2 = param_2 - 1;
      puVar7 = puVar7 + 3;
    } while (param_2 != 0);
  }
  puVar7 = *(undefined4 **)((int)this + 8);
  bVar9 = param_1 != puVar7;
  puVar8 = param_1;
  param_1 = puVar6 + uVar1 * 3;
  if (bVar9) {
    do {
      FUN_10016880(param_1,puVar8);
      puVar8 = puVar8 + 3;
      param_1 = param_1 + 3;
    } while (puVar8 != puVar7);
  }
  FUN_100212a4(*(void **)((int)this + 4));
  *(undefined4 **)((int)this + 0xc) = puVar3 + iVar5 * 3;
  iVar5 = *(int *)((int)this + 4);
  if (iVar5 != 0) {
    *(undefined4 **)((int)this + 4) = puVar3;
    *(undefined4 **)((int)this + 8) = puVar3 + (uVar1 + (*(int *)((int)this + 8) - iVar5) / 0xc) * 3
    ;
    return;
  }
  *(undefined4 **)((int)this + 4) = puVar3;
  *(undefined4 **)((int)this + 8) = puVar3 + uVar1 * 3;
  return;
}



void FUN_10016140(undefined4 *param_1,int param_2)

{
  undefined4 *puVar1;
  
  puVar1 = (undefined4 *)operator_new(0x18);
  if (param_1 == (undefined4 *)0x0) {
    param_1 = puVar1;
  }
  *puVar1 = param_1;
  if (param_2 != 0) {
    puVar1[1] = param_2;
    return;
  }
  puVar1[1] = puVar1;
  return;
}



basic_string<> *
FUN_10016170(basic_string<> *param_1,basic_string<> *param_2,basic_string<> *param_3)

{
  if (param_1 == param_2) {
    return param_3;
  }
  do {
    FUN_10016560(param_3,param_1);
    param_1 = param_1 + 0x10;
    param_3 = param_3 + 0x10;
  } while (param_1 != param_2);
  return param_3;
}



void FUN_100161b0(basic_string<> *param_1,int param_2,basic_string<> *param_3)

{
  if (param_2 != 0) {
    do {
      FUN_10016560(param_1,param_3);
      param_1 = param_1 + 0x10;
      param_2 = param_2 + -1;
    } while (param_2 != 0);
  }
  return;
}



void FUN_100161e0(undefined4 *param_1,int param_2,undefined4 *param_3)

{
  if (param_2 != 0) {
    do {
      if (param_1 != (undefined4 *)0x0) {
        *param_1 = *param_3;
      }
      param_1 = param_1 + 1;
      param_2 = param_2 + -1;
    } while (param_2 != 0);
  }
  return;
}



int __fastcall FUN_10016210(int param_1)

{
  int iVar1;
  
  iVar1 = *(int *)(param_1 + 4);
  if (iVar1 == 0) {
    return iVar1;
  }
  return (*(int *)(param_1 + 8) - iVar1) / 0xc;
}



void FUN_10016230(void)

{
  return;
}



undefined4 * FUN_10016240(undefined4 *param_1,undefined4 *param_2,undefined4 *param_3)

{
  if (param_1 == param_2) {
    return param_3;
  }
  do {
    FUN_10016880(param_3,param_1);
    param_1 = param_1 + 3;
    param_3 = param_3 + 3;
  } while (param_1 != param_2);
  return param_3;
}



void FUN_10016280(undefined4 *param_1,int param_2,undefined4 *param_3)

{
  if (param_2 != 0) {
    do {
      FUN_10016880(param_1,param_3);
      param_1 = param_1 + 3;
      param_2 = param_2 + -1;
    } while (param_2 != 0);
  }
  return;
}



void __cdecl FUN_100162b0(basic_string<> *param_1,basic_string<> *param_2,basic_string<> *param_3)

{
  char *pcVar1;
  char cVar2;
  int iVar3;
  bool bVar4;
  uint uVar5;
  code *pcVar6;
  uint uVar7;
  uint uVar8;
  code *pcVar9;
  
  if (param_1 != param_2) {
    do {
      uVar7 = *(uint *)npos_exref;
      uVar5 = *(uint *)(param_3 + 8);
      uVar8 = uVar5;
      if (uVar7 < uVar5) {
        uVar8 = uVar7;
      }
      if (param_1 == param_3) {
        if (*(uint *)(param_1 + 8) < uVar8) {
          std::_Xran();
        }
        std::basic_string<>::_Split(param_1);
        uVar5 = *(int *)(param_1 + 8) - uVar8;
        if (uVar5 < uVar7) {
          uVar7 = uVar5;
        }
        if (uVar7 != 0) {
          memmove((void *)(uVar8 + *(int *)(param_1 + 4)),
                  (void *)((int)(uVar8 + *(int *)(param_1 + 4)) + uVar7),uVar5 - uVar7);
          iVar3 = *(int *)(param_1 + 8);
          bVar4 = std::basic_string<>::_Grow(param_1,iVar3 - uVar7,false);
          if (bVar4) {
            std::basic_string<>::_Eos(param_1,iVar3 - uVar7);
          }
        }
        std::basic_string<>::_Split(param_1);
      }
      else {
        if ((uVar8 != 0) && (uVar8 == uVar5)) {
          pcVar6 = *(code **)(param_3 + 4);
          if (*(code **)(param_3 + 4) == (code *)0x0) {
            pcVar6 = _C_exref;
          }
          if ((byte)pcVar6[-1] < 0xfe) {
            if (*(int *)(param_1 + 4) != 0) {
              pcVar1 = (char *)(*(int *)(param_1 + 4) + -1);
              cVar2 = *pcVar1;
              if ((cVar2 == '\0') || (cVar2 == -1)) {
                FUN_100212a4(pcVar1);
              }
              else {
                *pcVar1 = cVar2 + -1;
              }
            }
            *(undefined4 *)(param_1 + 4) = 0;
            *(undefined4 *)(param_1 + 8) = 0;
            *(undefined4 *)(param_1 + 0xc) = 0;
            pcVar6 = *(code **)(param_3 + 4);
            if (*(code **)(param_3 + 4) == (code *)0x0) {
              pcVar6 = _C_exref;
            }
            *(code **)(param_1 + 4) = pcVar6;
            *(undefined4 *)(param_1 + 8) = *(undefined4 *)(param_3 + 8);
            *(undefined4 *)(param_1 + 0xc) = *(undefined4 *)(param_3 + 0xc);
            pcVar6[-1] = (code)((char)pcVar6[-1] + '\x01');
            goto LAB_100163e1;
          }
        }
        bVar4 = std::basic_string<>::_Grow(param_1,uVar8,true);
        if (bVar4) {
          pcVar6 = *(code **)(param_3 + 4);
          if (*(code **)(param_3 + 4) == (code *)0x0) {
            pcVar6 = _C_exref;
          }
          pcVar9 = *(code **)(param_1 + 4);
          for (uVar7 = uVar8 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
            *(undefined4 *)pcVar9 = *(undefined4 *)pcVar6;
            pcVar6 = pcVar6 + 4;
            pcVar9 = pcVar9 + 4;
          }
          for (uVar7 = uVar8 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
            *pcVar9 = *pcVar6;
            pcVar6 = pcVar6 + 1;
            pcVar9 = pcVar9 + 1;
          }
          *(uint *)(param_1 + 8) = uVar8;
          *(undefined *)(*(int *)(param_1 + 4) + uVar8) = 0;
        }
      }
LAB_100163e1:
      param_1 = param_1 + 0x10;
    } while (param_1 != param_2);
  }
  return;
}



basic_string<> * __cdecl
FUN_10016400(basic_string<> *param_1,basic_string<> *param_2,basic_string<> *param_3)

{
  char *pcVar1;
  char cVar2;
  int iVar3;
  bool bVar4;
  uint uVar5;
  code *pcVar6;
  uint uVar7;
  uint uVar8;
  basic_string<> *this;
  basic_string<> *pbVar9;
  code *pcVar10;
  
  if (param_1 == param_2) {
    return param_3;
  }
  do {
    uVar7 = *(uint *)npos_exref;
    uVar5 = *(uint *)(param_2 + -8);
    pbVar9 = param_2 + -0x10;
    this = param_3 + -0x10;
    uVar8 = uVar5;
    if (uVar7 < uVar5) {
      uVar8 = uVar7;
    }
    if (this == pbVar9) {
      if (*(uint *)(param_3 + -8) < uVar8) {
        std::_Xran();
      }
      std::basic_string<>::_Split(this);
      uVar5 = *(int *)(param_3 + -8) - uVar8;
      if (uVar5 < uVar7) {
        uVar7 = uVar5;
      }
      if (uVar7 != 0) {
        memmove((void *)(uVar8 + *(int *)(param_3 + -0xc)),
                (void *)((int)(uVar8 + *(int *)(param_3 + -0xc)) + uVar7),uVar5 - uVar7);
        iVar3 = *(int *)(param_3 + -8);
        bVar4 = std::basic_string<>::_Grow(this,iVar3 - uVar7,false);
        if (bVar4) {
          std::basic_string<>::_Eos(this,iVar3 - uVar7);
        }
      }
      std::basic_string<>::_Split(this);
    }
    else {
      if ((uVar8 != 0) && (uVar8 == uVar5)) {
        pcVar6 = *(code **)(param_2 + -0xc);
        if (*(code **)(param_2 + -0xc) == (code *)0x0) {
          pcVar6 = _C_exref;
        }
        if ((byte)pcVar6[-1] < 0xfe) {
          if (*(int *)(param_3 + -0xc) != 0) {
            pcVar1 = (char *)(*(int *)(param_3 + -0xc) + -1);
            cVar2 = *pcVar1;
            if ((cVar2 == '\0') || (cVar2 == -1)) {
              FUN_100212a4(pcVar1);
            }
            else {
              *pcVar1 = cVar2 + -1;
            }
          }
          *(undefined4 *)(param_3 + -0xc) = 0;
          *(undefined4 *)(param_3 + -8) = 0;
          *(undefined4 *)(param_3 + -4) = 0;
          pcVar6 = *(code **)(param_2 + -0xc);
          if (*(code **)(param_2 + -0xc) == (code *)0x0) {
            pcVar6 = _C_exref;
          }
          *(code **)(param_3 + -0xc) = pcVar6;
          *(undefined4 *)(param_3 + -8) = *(undefined4 *)(param_2 + -8);
          *(undefined4 *)(param_3 + -4) = *(undefined4 *)(param_2 + -4);
          pcVar6[-1] = (code)((char)pcVar6[-1] + '\x01');
          goto LAB_1001653f;
        }
      }
      bVar4 = std::basic_string<>::_Grow(this,uVar8,true);
      if (bVar4) {
        pcVar6 = *(code **)(param_2 + -0xc);
        if (*(code **)(param_2 + -0xc) == (code *)0x0) {
          pcVar6 = _C_exref;
        }
        pcVar10 = *(code **)(param_3 + -0xc);
        for (uVar7 = uVar8 >> 2; uVar7 != 0; uVar7 = uVar7 - 1) {
          *(undefined4 *)pcVar10 = *(undefined4 *)pcVar6;
          pcVar6 = pcVar6 + 4;
          pcVar10 = pcVar10 + 4;
        }
        for (uVar7 = uVar8 & 3; uVar7 != 0; uVar7 = uVar7 - 1) {
          *pcVar10 = *pcVar6;
          pcVar6 = pcVar6 + 1;
          pcVar10 = pcVar10 + 1;
        }
        *(uint *)(param_3 + -8) = uVar8;
        *(undefined *)(*(int *)(param_3 + -0xc) + uVar8) = 0;
      }
    }
LAB_1001653f:
    param_3 = this;
    param_2 = pbVar9;
    if (pbVar9 == param_1) {
      return this;
    }
  } while( true );
}



void __cdecl FUN_10016560(basic_string<> *param_1,basic_string<> *param_2)

{
  int iVar1;
  bool bVar2;
  uint uVar3;
  code *pcVar4;
  uint uVar5;
  uint uVar6;
  code *pcVar7;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_10023751;
  local_c = ExceptionList;
  local_4 = 0;
  if (param_1 != (basic_string<> *)0x0) {
    ExceptionList = &local_c;
    *param_1 = *param_2;
    *(undefined4 *)(param_1 + 4) = 0;
    *(undefined4 *)(param_1 + 8) = 0;
    *(undefined4 *)(param_1 + 0xc) = 0;
    uVar5 = *(uint *)npos_exref;
    uVar3 = *(uint *)(param_2 + 8);
    uVar6 = uVar3;
    if (uVar5 < uVar3) {
      uVar6 = uVar5;
    }
    if (param_1 == param_2) {
      if (uVar6 != 0) {
        std::_Xran();
      }
      std::basic_string<>::_Split(param_1);
      uVar3 = *(int *)(param_1 + 8) - uVar6;
      if (uVar3 < uVar5) {
        uVar5 = uVar3;
      }
      if (uVar5 != 0) {
        memmove((void *)(uVar6 + *(int *)(param_1 + 4)),
                (void *)((int)(uVar6 + *(int *)(param_1 + 4)) + uVar5),uVar3 - uVar5);
        iVar1 = *(int *)(param_1 + 8);
        bVar2 = std::basic_string<>::_Grow(param_1,iVar1 - uVar5,false);
        if (bVar2) {
          std::basic_string<>::_Eos(param_1,iVar1 - uVar5);
        }
      }
      std::basic_string<>::_Split(param_1);
      ExceptionList = local_c;
      return;
    }
    if ((uVar6 != 0) && (uVar6 == uVar3)) {
      pcVar4 = *(code **)(param_2 + 4);
      if (*(code **)(param_2 + 4) == (code *)0x0) {
        pcVar4 = _C_exref;
      }
      if ((byte)pcVar4[-1] < 0xfe) {
        std::basic_string<>::_Tidy(param_1,true);
        pcVar4 = *(code **)(param_2 + 4);
        if (*(code **)(param_2 + 4) == (code *)0x0) {
          pcVar4 = _C_exref;
        }
        *(code **)(param_1 + 4) = pcVar4;
        *(undefined4 *)(param_1 + 8) = *(undefined4 *)(param_2 + 8);
        *(undefined4 *)(param_1 + 0xc) = *(undefined4 *)(param_2 + 0xc);
        pcVar4[-1] = (code)((char)pcVar4[-1] + '\x01');
        ExceptionList = local_c;
        return;
      }
    }
    bVar2 = std::basic_string<>::_Grow(param_1,uVar6,true);
    if (bVar2) {
      pcVar4 = *(code **)(param_2 + 4);
      if (*(code **)(param_2 + 4) == (code *)0x0) {
        pcVar4 = _C_exref;
      }
      pcVar7 = *(code **)(param_1 + 4);
      for (uVar5 = uVar6 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        *(undefined4 *)pcVar7 = *(undefined4 *)pcVar4;
        pcVar4 = pcVar4 + 4;
        pcVar7 = pcVar7 + 4;
      }
      for (uVar5 = uVar6 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *pcVar7 = *pcVar4;
        pcVar4 = pcVar4 + 1;
        pcVar7 = pcVar7 + 1;
      }
      *(uint *)(param_1 + 8) = uVar6;
      *(undefined *)(*(int *)(param_1 + 4) + uVar6) = 0;
    }
  }
  ExceptionList = local_c;
  return;
}



void FUN_100166d0(void)

{
  return;
}



void __cdecl FUN_100166e0(undefined4 *param_1,undefined4 *param_2)

{
  if (param_1 != (undefined4 *)0x0) {
    *param_1 = *param_2;
  }
  return;
}



void __cdecl FUN_10016700(undefined4 *param_1,undefined4 *param_2)

{
  int iVar1;
  bool bVar2;
  uint uVar3;
  code *pcVar4;
  uint uVar5;
  basic_string<> *this;
  uint uVar6;
  code *pcVar7;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_10023771;
  local_c = ExceptionList;
  local_4 = 0;
  if (param_1 != (undefined4 *)0x0) {
    ExceptionList = &local_c;
    *param_1 = *param_2;
    this = (basic_string<> *)(param_1 + 1);
    *this = *(basic_string<> *)(param_2 + 1);
    param_1[2] = 0;
    param_1[3] = 0;
    param_1[4] = 0;
    uVar3 = param_2[3];
    uVar6 = *(uint *)npos_exref;
    uVar5 = uVar3;
    if (uVar6 < uVar3) {
      uVar5 = uVar6;
    }
    if (this == (basic_string<> *)(param_2 + 1)) {
      if (uVar5 != 0) {
        std::_Xran();
      }
      std::basic_string<>::_Split(this);
      uVar3 = param_1[3] - uVar5;
      if (uVar3 < uVar6) {
        uVar6 = uVar3;
      }
      if (uVar6 != 0) {
        memmove((void *)(uVar5 + param_1[2]),(void *)((int)(uVar5 + param_1[2]) + uVar6),
                uVar3 - uVar6);
        iVar1 = param_1[3];
        bVar2 = std::basic_string<>::_Grow(this,iVar1 - uVar6,false);
        if (bVar2) {
          std::basic_string<>::_Eos(this,iVar1 - uVar6);
        }
      }
      std::basic_string<>::_Split(this);
      ExceptionList = local_c;
      return;
    }
    if ((uVar5 != 0) && (uVar5 == uVar3)) {
      pcVar4 = (code *)param_2[2];
      if ((code *)param_2[2] == (code *)0x0) {
        pcVar4 = _C_exref;
      }
      if ((byte)pcVar4[-1] < 0xfe) {
        std::basic_string<>::_Tidy(this,true);
        pcVar4 = (code *)param_2[2];
        if ((code *)param_2[2] == (code *)0x0) {
          pcVar4 = _C_exref;
        }
        param_1[2] = pcVar4;
        param_1[3] = param_2[3];
        param_1[4] = param_2[4];
        pcVar4[-1] = (code)((char)pcVar4[-1] + '\x01');
        ExceptionList = local_c;
        return;
      }
    }
    bVar2 = std::basic_string<>::_Grow(this,uVar5,true);
    if (bVar2) {
      pcVar4 = (code *)param_2[2];
      if ((code *)param_2[2] == (code *)0x0) {
        pcVar4 = _C_exref;
      }
      pcVar7 = (code *)param_1[2];
      for (uVar3 = uVar5 >> 2; uVar3 != 0; uVar3 = uVar3 - 1) {
        *(undefined4 *)pcVar7 = *(undefined4 *)pcVar4;
        pcVar4 = pcVar4 + 4;
        pcVar7 = pcVar7 + 4;
      }
      for (uVar3 = uVar5 & 3; uVar3 != 0; uVar3 = uVar3 - 1) {
        *pcVar7 = *pcVar4;
        pcVar4 = pcVar4 + 1;
        pcVar7 = pcVar7 + 1;
      }
      param_1[3] = uVar5;
      *(undefined *)(param_1[2] + uVar5) = 0;
    }
  }
  ExceptionList = local_c;
  return;
}



void __cdecl FUN_10016880(undefined4 *param_1,undefined4 *param_2)

{
  if (param_1 != (undefined4 *)0x0) {
    *param_1 = *param_2;
    param_1[1] = param_2[1];
    param_1[2] = param_2[2];
  }
  return;
}



void FUN_100168d0(void)

{
                    // WARNING: Could not recover jumptable at 0x100168d5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::Init((Init *)&DAT_100384f5);
  return;
}



void FUN_100168e0(void)

{
  FUN_100214c2(FUN_100168f0);
  return;
}



void FUN_100168f0(void)

{
                    // WARNING: Could not recover jumptable at 0x100168f5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::~Init((Init *)&DAT_100384f5);
  return;
}



void FUN_10016910(void)

{
                    // WARNING: Could not recover jumptable at 0x10016915. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::_Winit((_Winit *)&DAT_100384f4);
  return;
}



void FUN_10016920(void)

{
  FUN_100214c2(FUN_10016930);
  return;
}



void FUN_10016930(void)

{
                    // WARNING: Could not recover jumptable at 0x10016935. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::~_Winit((_Winit *)&DAT_100384f4);
  return;
}



// public: __thiscall Housemarque::Threedee_Engine::Morphing_Mesh::Morphing_Mesh(char const *,class
// Housemarque::Threedee_Engine::Scene *)

Morphing_Mesh * __thiscall
Housemarque::Threedee_Engine::Morphing_Mesh::Morphing_Mesh
          (Morphing_Mesh *this,char *param_1,Scene *param_2)

{
  srModelInstance *this_00;
  srScene *psVar1;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0x16940  17
                    // ??0Morphing_Mesh@Threedee_Engine@Housemarque@@QAE@PBDPAVScene@12@@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_1002378b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined4 *)this = 0;
  this_00 = (srModelInstance *)srHeap::allocate((srHeap *)srHeap_exref,0x160);
  uStack_4 = 0;
  if (this_00 == (srModelInstance *)0x0) {
    this_00 = (srModelInstance *)0x0;
  }
  else {
    psVar1 = Scene::Get_Sr_Scene();
    srModelInstance::srModelInstance(this_00,(srNode *)psVar1);
    *(undefined ***)this_00 = &PTR_FUN_10025e58;
    *(undefined ***)(this_00 + 0x138) = &PTR_LAB_10025e44;
  }
  uStack_4 = 0xffffffff;
  *(srModelInstance **)(this + 4) = this_00;
  srRuntimeClass::setName((srRuntimeClass *)this_00,param_1);
  ExceptionList = pvStack_c;
  return this;
}



// public: __thiscall Housemarque::Threedee_Engine::Morphing_Mesh::~Morphing_Mesh(void)

void __thiscall Housemarque::Threedee_Engine::Morphing_Mesh::~Morphing_Mesh(Morphing_Mesh *this)

{
  bool bVar1;
  bool bVar2;
  char *pcVar3;
  int iVar4;
  basic_ostream<> *pbVar5;
  char *pcVar6;
  Error_Log_Inserter aEStack_1c [8];
  Error_Log_Inserter aEStack_14 [8];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0x169e0  37  ??1Morphing_Mesh@Threedee_Engine@Housemarque@@QAE@XZ
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_100237c5;
  pvStack_c = ExceptionList;
  bVar1 = false;
  ExceptionList = &pvStack_c;
  pcVar3 = srRuntimeClass::getName(*(srRuntimeClass **)(this + 4));
  iVar4 = srClass::release(*(srClass **)(this + 4));
  if (iVar4 == 0) {
    bVar2 = Housemarque::Kernel::Logging_Enabled();
    if (bVar2) {
      bVar1 = true;
      iVar4 = Housemarque::Kernel::Error_Log(aEStack_1c);
      uStack_4 = 0;
      pbVar5 = std::operator<<(*(basic_ostream<> **)(iVar4 + 4),
                               s_Morphing_mesh_instance_wasnt_rel_10037544);
      pcVar6 = &DAT_10037540;
      pbVar5 = std::operator<<(pbVar5,pcVar3);
      std::operator<<(pbVar5,pcVar6);
    }
  }
  uStack_4 = 0xffffffff;
  if (bVar1) {
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter(aEStack_1c);
  }
  bVar1 = false;
  if (*(srClass **)this != (srClass *)0x0) {
    iVar4 = srClass::release(*(srClass **)this);
    if (iVar4 == 0) {
      bVar2 = Housemarque::Kernel::Logging_Enabled();
      if (bVar2) {
        bVar1 = true;
        iVar4 = Housemarque::Kernel::Error_Log(aEStack_14);
        uStack_4 = 1;
        pbVar5 = std::operator<<(*(basic_ostream<> **)(iVar4 + 4),
                                 s_Morphing_mesh_model_wasnt_releas_10037578);
        pcVar6 = &DAT_10037574;
        pbVar5 = std::operator<<(pbVar5,pcVar3);
        std::operator<<(pbVar5,pcVar6);
      }
    }
    uStack_4 = 0xffffffff;
    if (bVar1) {
      Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter(aEStack_14);
    }
  }
  ExceptionList = pvStack_c;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Morphing_Mesh::Disable(void)

void __fastcall Housemarque::Threedee_Engine::Morphing_Mesh::Disable(void)

{
  int in_ECX;
  
                    // 0x16b00  63  ?Disable@Morphing_Mesh@Threedee_Engine@Housemarque@@QAIXXZ
  srNode::setFlag(*(srNode **)(in_ECX + 4),0);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Morphing_Mesh::Enable(void)

void __fastcall Housemarque::Threedee_Engine::Morphing_Mesh::Enable(void)

{
  int in_ECX;
  
                    // 0x16b10  67  ?Enable@Morphing_Mesh@Threedee_Engine@Housemarque@@QAIXXZ
  srNode::clearFlag(*(srNode **)(in_ECX + 4),0);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Morphing_Mesh::Remove(void)

void __fastcall Housemarque::Threedee_Engine::Morphing_Mesh::Remove(void)

{
  int in_ECX;
  
                    // 0x16b20  120  ?Remove@Morphing_Mesh@Threedee_Engine@Housemarque@@QAIXXZ
  srNode::setParent(*(srNode **)(in_ECX + 4),(srNode *)0x0,0);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Morphing_Mesh::Add(class
// Housemarque::Threedee_Engine::Scene *)

void __fastcall Housemarque::Threedee_Engine::Morphing_Mesh::Add(Scene *param_1)

{
  srScene *psVar1;
  int iVar2;
  
                    // 0x16b30  48
                    // ?Add@Morphing_Mesh@Threedee_Engine@Housemarque@@QAIXPAVScene@23@@Z
  iVar2 = 0;
  psVar1 = Scene::Get_Sr_Scene();
  srNode::setParent(*(srNode **)(param_1 + 4),(srNode *)psVar1,iVar2);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Morphing_Mesh::Set_Location(class
// Housemarque::Template_Library::Vector_3<float> const &)

void __fastcall Housemarque::Threedee_Engine::Morphing_Mesh::Set_Location(Vector_3<float> *param_1)

{
  float *in_EDX;
  double local_18;
  double local_10;
  double local_8;
  
                    // 0x16b50  138
                    // ?Set_Location@Morphing_Mesh@Threedee_Engine@Housemarque@@QAIXABV?$Vector_3@M@Template_Library@3@@Z
  local_18 = (double)*in_EDX;
  local_10 = (double)in_EDX[1];
  local_8 = (double)in_EDX[2];
  srNode::setLocation(*(srNode **)(param_1 + 4),(srVector3T<double> *)&local_18);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Morphing_Mesh::Set_Base(class
// Housemarque::Template_Library::Matrix_3x3<float> const &)

void __fastcall Housemarque::Threedee_Engine::Morphing_Mesh::Set_Base(Matrix_3x3<float> *param_1)

{
  int iVar1;
  undefined4 *in_EDX;
  srMatrix3T<float> *psVar2;
  srMatrix3T<float> local_24 [36];
  
  psVar2 = local_24;
                    // 0x16b90  129
                    // ?Set_Base@Morphing_Mesh@Threedee_Engine@Housemarque@@QAIXABV?$Matrix_3x3@M@Template_Library@3@@Z
  for (iVar1 = 9; iVar1 != 0; iVar1 = iVar1 + -1) {
    *(undefined4 *)psVar2 = *in_EDX;
    in_EDX = in_EDX + 1;
    psVar2 = psVar2 + 4;
  }
  srNode::setRotation(*(srNode **)(param_1 + 4),local_24);
  return;
}



// WARNING: Restarted to delay deadcode elimination for space: stack
// public: __thiscall Housemarque::Threedee_Engine::Morph_Data::Morph_Data(class
// Housemarque::Threedee_Engine::Resource_Handler *,char const *,char const *,unsigned int,unsigned
// int,class Housemarque::Template_Library::Matrix_3x3<float> const &,class
// Housemarque::Template_Library::Vector_3<float> const &,class
// Housemarque::Template_Library::Vector_3<float> const &)

Vector_3<float> * __thiscall
Housemarque::Threedee_Engine::Morph_Data::Morph_Data
          (Morph_Data *this,Resource_Handler *param_1,char *param_2,char *param_3,uint param_4,
          uint param_5,Matrix_3x3<float> *param_6,Vector_3<float> *param_7,Vector_3<float> *param_8)

{
  basic_string<> *pbVar1;
  byte bVar2;
  int *piVar3;
  int *piVar4;
  int *piVar5;
  bool bVar6;
  int iVar7;
  undefined4 *puVar8;
  basic_ostream<> *pbVar9;
  char *pcVar10;
  basic_stringstream<> *pbVar11;
  list<> *plVar12;
  srMeshModel *this_00;
  Config_Base *this_01;
  uint uVar13;
  uint uVar14;
  code *pcVar15;
  srGFInterface *psVar16;
  char *in_stack_00000024;
  basic_istream<> bStack00000030;
  int *piStack00000034;
  int iStack00000038;
  basic_string<> bStack0000003c;
  char *in_stack_00000040;
  basic_string<> bStack0000004c;
  char *in_stack_00000050;
  code *pcStack0000005c;
  code *pcStack00000064;
  void *in_stack_000000a8;
  char in_stack_000000ac;
  FILE *in_stack_000000b4;
  code *pcStack000000b8;
  void *in_stack_0000133c;
  list<> *in_stack_0000134c;
  char *in_stack_00001350;
  char *in_stack_00001354;
  uint in_stack_00001358;
  uint in_stack_0000135c;
  float *in_stack_00001360;
  float *in_stack_00001364;
  float *in_stack_00001368;
  Vector_3<float> *pVVar17;
  float *pfVar18;
  float *pfVar19;
  char cVar20;
  char *pcVar21;
  float *pfVar22;
  char cVar23;
  char cVar24;
  undefined4 uStack_c;
  basic_string<> *pbStack_8;
  uint local_4;
  
                    // 0x16bc0  16
                    // ??0Morph_Data@Threedee_Engine@Housemarque@@QAE@PAVResource_Handler@12@PBD1IIABV?$Matrix_3x3@M@Template_Library@2@ABV?$Vector_3@M@52@3@Z
  local_4 = 0xffffffff;
  pbStack_8 = (basic_string<> *)&LAB_10023a44;
  uStack_c = ExceptionList;
  ExceptionList = &uStack_c;
  FUN_10021580();
  local_4 = 0;
  param_7 = (Vector_3<float> *)this_01;
  Housemarque::Template_Library::Config_Base::Config_Base(this_01);
  param_3 = (char *)CONCAT31(param_3._1_3_,uStack_c._3_1_);
  std::basic_string<>::_Tidy((basic_string<> *)&param_3,false);
  uVar13 = 0xffffffff;
  pcVar10 = s_not_defined_100375a4;
  do {
    if (uVar13 == 0) break;
    uVar13 = uVar13 - 1;
    cVar23 = *pcVar10;
    pcVar10 = pcVar10 + 1;
  } while (cVar23 != '\0');
  uVar13 = ~uVar13 - 1;
  bVar6 = std::basic_string<>::_Grow((basic_string<> *)&param_3,uVar13,true);
  if (bVar6) {
    pcVar10 = s_not_defined_100375a4;
    pcVar21 = (char *)param_4;
    for (uVar14 = uVar13 >> 2; uVar14 != 0; uVar14 = uVar14 - 1) {
      *(undefined4 *)pcVar21 = *(undefined4 *)pcVar10;
      pcVar10 = pcVar10 + 4;
      pcVar21 = pcVar21 + 4;
    }
    for (uVar14 = uVar13 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
      *pcVar21 = *pcVar10;
      pcVar10 = pcVar10 + 1;
      pcVar21 = pcVar21 + 1;
    }
    std::basic_string<>::_Eos((basic_string<> *)&param_3,uVar13);
  }
  param_8 = (Vector_3<float> *)CONCAT31(param_8._1_3_,uStack_c._3_1_);
  std::basic_string<>::_Tidy((basic_string<> *)&param_8,false);
  uVar13 = 0xffffffff;
  pcVar10 = s_hx_name_100375b0;
  do {
    if (uVar13 == 0) break;
    uVar13 = uVar13 - 1;
    cVar23 = *pcVar10;
    pcVar10 = pcVar10 + 1;
  } while (cVar23 != '\0');
  uVar13 = ~uVar13 - 1;
  bVar6 = std::basic_string<>::_Grow((basic_string<> *)&param_8,uVar13,true);
  if (bVar6) {
    pcVar10 = s_hx_name_100375b0;
    pcVar21 = in_stack_00000024;
    for (uVar14 = uVar13 >> 2; uVar14 != 0; uVar14 = uVar14 - 1) {
      *(undefined4 *)pcVar21 = *(undefined4 *)pcVar10;
      pcVar10 = pcVar10 + 4;
      pcVar21 = pcVar21 + 4;
    }
    for (uVar14 = uVar13 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
      *pcVar21 = *pcVar10;
      pcVar10 = pcVar10 + 1;
      pcVar21 = pcVar21 + 1;
    }
    std::basic_string<>::_Eos((basic_string<> *)&param_8,uVar13);
  }
  pbVar1 = (basic_string<> *)(this_01 + 0xc);
  *pbVar1 = param_3._0_1_;
  pbStack_8 = pbVar1;
  std::basic_string<>::_Tidy(pbVar1,false);
  std::basic_string<>::assign(pbVar1,(basic_string<> *)&param_3,0,*(uint *)npos_exref);
  param_1 = (Resource_Handler *)operator_new(0x20);
  if (param_1 != (Resource_Handler *)0x0) {
    FUN_10014030(param_1,(basic_string<> *)&param_8,this_01,pbVar1);
  }
  std::basic_string<>::_Tidy((basic_string<> *)&param_8,true);
  std::basic_string<>::_Tidy((basic_string<> *)&param_3,true);
  bStack0000003c = uStack_c._3_1_;
  std::basic_string<>::_Tidy(&stack0x0000003c,false);
  uVar13 = 0xffffffff;
  pcVar10 = s_not_defined_100375b8;
  do {
    if (uVar13 == 0) break;
    uVar13 = uVar13 - 1;
    cVar23 = *pcVar10;
    pcVar10 = pcVar10 + 1;
  } while (cVar23 != '\0');
  uVar13 = ~uVar13 - 1;
  bVar6 = std::basic_string<>::_Grow(&stack0x0000003c,uVar13,true);
  if (bVar6) {
    pcVar10 = s_not_defined_100375b8;
    for (uVar14 = uVar13 >> 2; uVar14 != 0; uVar14 = uVar14 - 1) {
      *(undefined4 *)in_stack_00000040 = *(undefined4 *)pcVar10;
      pcVar10 = pcVar10 + 4;
      in_stack_00000040 = in_stack_00000040 + 4;
    }
    for (uVar14 = uVar13 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
      *in_stack_00000040 = *pcVar10;
      pcVar10 = pcVar10 + 1;
      in_stack_00000040 = in_stack_00000040 + 1;
    }
    std::basic_string<>::_Eos(&stack0x0000003c,uVar13);
  }
  bStack0000004c = uStack_c._3_1_;
  std::basic_string<>::_Tidy(&stack0x0000004c,false);
  uVar13 = 0xffffffff;
  pcVar10 = s_default_mesh_name_100375c4;
  do {
    if (uVar13 == 0) break;
    uVar13 = uVar13 - 1;
    cVar23 = *pcVar10;
    pcVar10 = pcVar10 + 1;
  } while (cVar23 != '\0');
  uVar13 = ~uVar13 - 1;
  bVar6 = std::basic_string<>::_Grow(&stack0x0000004c,uVar13,true);
  if (bVar6) {
    pcVar10 = s_default_mesh_name_100375c4;
    for (uVar14 = uVar13 >> 2; uVar14 != 0; uVar14 = uVar14 - 1) {
      *(undefined4 *)in_stack_00000050 = *(undefined4 *)pcVar10;
      pcVar10 = pcVar10 + 4;
      in_stack_00000050 = in_stack_00000050 + 4;
    }
    for (uVar14 = uVar13 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
      *in_stack_00000050 = *pcVar10;
      pcVar10 = pcVar10 + 1;
      in_stack_00000050 = in_stack_00000050 + 1;
    }
    std::basic_string<>::_Eos(&stack0x0000004c,uVar13);
  }
  pbVar1 = (basic_string<> *)(this_01 + 0x1c);
  *pbVar1 = bStack0000003c;
  pbStack_8 = pbVar1;
  std::basic_string<>::_Tidy(pbVar1,false);
  std::basic_string<>::assign(pbVar1,&stack0x0000003c,0,*(uint *)npos_exref);
  param_1 = (Resource_Handler *)operator_new(0x20);
  if (param_1 != (Resource_Handler *)0x0) {
    FUN_10014030(param_1,&stack0x0000004c,this_01,pbVar1);
  }
  std::basic_string<>::_Tidy(&stack0x0000004c,true);
  std::basic_string<>::_Tidy(&stack0x0000003c,true);
  param_3 = (char *)CONCAT31(param_3._1_3_,uStack_c._3_1_);
  std::basic_string<>::_Tidy((basic_string<> *)&param_3,false);
  uVar13 = 0xffffffff;
  pcVar10 = s_not_defined_100375d8;
  do {
    if (uVar13 == 0) break;
    uVar13 = uVar13 - 1;
    cVar23 = *pcVar10;
    pcVar10 = pcVar10 + 1;
  } while (cVar23 != '\0');
  uVar13 = ~uVar13 - 1;
  bVar6 = std::basic_string<>::_Grow((basic_string<> *)&param_3,uVar13,true);
  if (bVar6) {
    pcVar10 = s_not_defined_100375d8;
    pcVar21 = (char *)param_4;
    for (uVar14 = uVar13 >> 2; uVar14 != 0; uVar14 = uVar14 - 1) {
      *(undefined4 *)pcVar21 = *(undefined4 *)pcVar10;
      pcVar10 = pcVar10 + 4;
      pcVar21 = pcVar21 + 4;
    }
    for (uVar14 = uVar13 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
      *pcVar21 = *pcVar10;
      pcVar10 = pcVar10 + 1;
      pcVar21 = pcVar21 + 1;
    }
    std::basic_string<>::_Eos((basic_string<> *)&param_3,uVar13);
  }
  param_8 = (Vector_3<float> *)CONCAT31(param_8._1_3_,uStack_c._3_1_);
  std::basic_string<>::_Tidy((basic_string<> *)&param_8,false);
  uVar13 = 0xffffffff;
  pcVar10 = s_material_path_100375e4;
  do {
    if (uVar13 == 0) break;
    uVar13 = uVar13 - 1;
    cVar23 = *pcVar10;
    pcVar10 = pcVar10 + 1;
  } while (cVar23 != '\0');
  uVar13 = ~uVar13 - 1;
  bVar6 = std::basic_string<>::_Grow((basic_string<> *)&param_8,uVar13,true);
  if (bVar6) {
    pcVar10 = s_material_path_100375e4;
    for (uVar14 = uVar13 >> 2; uVar14 != 0; uVar14 = uVar14 - 1) {
      *(undefined4 *)in_stack_00000024 = *(undefined4 *)pcVar10;
      pcVar10 = pcVar10 + 4;
      in_stack_00000024 = in_stack_00000024 + 4;
    }
    for (uVar14 = uVar13 & 3; uVar14 != 0; uVar14 = uVar14 - 1) {
      *in_stack_00000024 = *pcVar10;
      pcVar10 = pcVar10 + 1;
      in_stack_00000024 = in_stack_00000024 + 1;
    }
    std::basic_string<>::_Eos((basic_string<> *)&param_8,uVar13);
  }
  pbVar1 = (basic_string<> *)(this_01 + 0x2c);
  *pbVar1 = param_3._0_1_;
  pbStack_8 = pbVar1;
  std::basic_string<>::_Tidy(pbVar1,false);
  std::basic_string<>::assign(pbVar1,(basic_string<> *)&param_3,0,*(uint *)npos_exref);
  param_1 = (Resource_Handler *)operator_new(0x20);
  if (param_1 != (Resource_Handler *)0x0) {
    FUN_10014030(param_1,(basic_string<> *)&param_8,this_01,pbVar1);
  }
  std::basic_string<>::_Tidy((basic_string<> *)&param_8,true);
  std::basic_string<>::_Tidy((basic_string<> *)&param_3,true);
  *(undefined4 *)(this_01 + 0x3c) = 0;
  Housemarque::Template_Library::HTSC_Table_Base::HTSC_Table_Base
            ((HTSC_Table_Base *)(this_01 + 0x40),in_stack_0000135c);
  Housemarque::Template_Library::HTSC_Table_Base::HTSC_Table_Base
            ((HTSC_Table_Base *)(this_01 + 0x48),in_stack_00001358);
  bStack00000030 = (basic_istream<>)uStack_c._3_1_;
  piStack00000034 = (int *)operator_new(0xc);
  *piStack00000034 = (int)piStack00000034;
  piStack00000034[1] = (int)piStack00000034;
  iStack00000038 = 0;
  psVar16 = (srGFInterface *)0x0;
  pcStack0000005c = _vbtable__exref;
  std::ios_base::ios_base((ios_base *)&stack0x000000b8);
  uVar13 = 0x40;
  pcStack000000b8 = _vftable__exref;
  local_4 = 0x40;
  std::basic_istream<>::basic_istream<>
            ((basic_istream<> *)&stack0x0000005c,(basic_streambuf<> *)&stack0x00000064,false);
  std::basic_filebuf<>::basic_filebuf<>((basic_filebuf<> *)&stack0x00000064,(_iobuf *)0x0);
  *(code **)((int)&stack0x0000005c + *(int *)(pcStack0000005c + 4)) = _vftable__exref;
  iVar7 = std::basic_filebuf<>::open(in_stack_00001350,1);
  if (iVar7 == 0) {
    std::basic_ios<>::setstate
              ((basic_ios<> *)((int)&stack0x0000005c + *(int *)(pcStack0000005c + 4)),2,false);
  }
  if (((&stack0x00000060)[*(int *)(pcStack0000005c + 4)] & 6) != 0) {
    uVar13 = 0x41;
    local_4 = 0x41;
    puVar8 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&param_2);
    pbVar9 = std::operator<<((basic_ostream<> *)*puVar8,s_3de__cant_open__Morph_Data__conf_10037614)
    ;
    cVar23 = '\0';
    pbVar9 = std::operator<<(pbVar9,in_stack_00001350);
    std::operator<<(pbVar9,cVar23);
    iVar7 = 0x89;
    pcVar21 = s_E__work_ht_3de_3DE_Morph_cpp_100375f4;
    pcVar10 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(s__config_file_fail___10037640,pcVar10,pcVar21,iVar7);
  }
  if ((uVar13 & 1) != 0) {
    uVar13 = uVar13 & 0xfffffffe;
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&param_2);
  }
  Housemarque::Legacy::Segment::Segment
            ((Segment *)&stack0x0000018c,(basic_istream<> *)&stack0x0000005c,'{','}',';');
  bVar6 = std::operator==((basic_string<> *)
                          (-(uint)(&stack0x00000000 != (undefined *)0xfffffe74) &
                          (uint)&stack0x00000190),s_settings_10037654);
  if (!bVar6) {
    uVar13 = uVar13 | 2;
    local_4 = uVar13;
    puVar8 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&param_2);
    pbVar9 = std::operator<<((basic_ostream<> *)*puVar8,s_3de__Morph_Data__illegal_config_f_10037680
                            );
    cVar23 = '\0';
    pbVar9 = std::operator<<(pbVar9,in_stack_00001350);
    std::operator<<(pbVar9,cVar23);
    iVar7 = 0x8c;
    pcVar21 = s_E__work_ht_3de_3DE_Morph_cpp_10037660;
    pcVar10 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(s_mesh_data_file___settings__100376a8,pcVar10,pcVar21,iVar7);
  }
  if ((uVar13 & 2) != 0) {
    uVar13 = uVar13 & 0xfffffffd;
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&param_2);
  }
  pbVar11 = Housemarque::Legacy::Segment::Stream();
  if ((*(uint *)(pbVar11 + *(int *)(*(int *)pbVar11 + 4) + 4) & 1) == 0) {
    do {
      cVar24 = ';';
      cVar20 = '}';
      cVar23 = '{';
      pbVar11 = Housemarque::Legacy::Segment::Stream();
      Housemarque::Legacy::Segment::Segment
                ((Segment *)&stack0x0000022c,(basic_istream<> *)pbVar11,cVar23,cVar20,cVar24);
      bVar6 = std::operator==((basic_string<> *)
                              (-(uint)(&stack0x00000000 != (undefined *)0xfffffdd4) &
                              (uint)&stack0x00000230),in_stack_00001354);
      if (bVar6) {
        cVar24 = ';';
        cVar20 = '}';
        cVar23 = '{';
        pbVar11 = Housemarque::Legacy::Segment::Stream();
        Housemarque::Legacy::Segment::Segment
                  ((Segment *)&stack0x000000ec,(basic_istream<> *)pbVar11,cVar23,cVar20,cVar24);
        bVar6 = std::operator==((basic_string<> *)
                                (-(uint)(&stack0x00000000 != (undefined *)0xffffff14) &
                                (uint)&stack0x000000f0),s_material_list_100376c4);
        if (!bVar6) {
          uVar13 = uVar13 | 4;
          local_4 = uVar13;
          puVar8 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&param_2);
          pbVar9 = std::operator<<((basic_ostream<> *)*puVar8,
                                   s_Invalid_settings_segment_in_file_10037704);
          cVar23 = '\0';
          pcVar21 = s_with_handle__100376f4;
          pcVar10 = in_stack_00001354;
          pbVar9 = std::operator<<(pbVar9,in_stack_00001350);
          pbVar9 = std::operator<<(pbVar9,pcVar21);
          pbVar9 = std::operator<<(pbVar9,pcVar10);
          std::operator<<(pbVar9,cVar23);
          iVar7 = 0x95;
          pcVar21 = s_E__work_ht_3de_3DE_Morph_cpp_100376d4;
          pcVar10 = Housemarque::Kernel::Error::Tmp_Str();
          Housemarque::Kernel::Throw_Error
                    (s_material_list___material_list__10037728,pcVar10,pcVar21,iVar7);
        }
        if ((uVar13 & 4) != 0) {
          uVar13 = uVar13 & 0xfffffffb;
          Housemarque::Kernel::Inserter::~Inserter((Inserter *)&param_2);
        }
        pbVar11 = Housemarque::Legacy::Segment::Stream();
        Resource_Handler::Import_Material_List(in_stack_0000134c,&stack0x00000030,SUB41(pbVar11,0));
        pbVar11 = Housemarque::Legacy::Segment::Stream();
        if (((byte)pbVar11[*(int *)(*(int *)pbVar11 + 4) + 4] & 1) != 0) {
          uVar13 = uVar13 | 8;
          local_4 = uVar13;
          puVar8 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&param_1);
          pbVar9 = std::operator<<((basic_ostream<> *)*puVar8,
                                   s_Invalid_settings_segment_in_file_10037778);
          cVar23 = '\0';
          pcVar21 = s_with_handle__10037768;
          pcVar10 = in_stack_00001354;
          pbVar9 = std::operator<<(pbVar9,in_stack_00001350);
          pbVar9 = std::operator<<(pbVar9,pcVar21);
          pbVar9 = std::operator<<(pbVar9,pcVar10);
          std::operator<<(pbVar9,cVar23);
          iVar7 = 0x99;
          pcVar21 = s_E__work_ht_3de_3DE_Morph_cpp_10037748;
          pcVar10 = Housemarque::Kernel::Error::Tmp_Str();
          Housemarque::Kernel::Throw_Error(s__data_Stream___eof___1003779c,pcVar10,pcVar21,iVar7);
        }
        if ((uVar13 & 8) != 0) {
          uVar13 = uVar13 & 0xfffffff7;
          Housemarque::Kernel::Inserter::~Inserter((Inserter *)&param_1);
        }
        cVar24 = ';';
        cVar20 = '}';
        cVar23 = '{';
        pbVar11 = Housemarque::Legacy::Segment::Stream();
        Housemarque::Legacy::Segment::Segment
                  ((Segment *)&stack0x000002cc,(basic_istream<> *)pbVar11,cVar23,cVar20,cVar24);
        bVar6 = std::operator==((basic_string<> *)
                                (-(uint)(&stack0x00000000 != (undefined *)0xfffffd34) &
                                (uint)&stack0x000002d0),&DAT_100377b4);
        if (!bVar6) {
          uVar13 = uVar13 | 0x10;
          local_4 = uVar13;
          puVar8 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&pbStack_8);
          pbVar9 = std::operator<<((basic_ostream<> *)*puVar8,
                                   s_Invalid_settings_segment_in_file_100377ec);
          cVar23 = '\0';
          pcVar21 = s_with_handle__100377dc;
          pcVar10 = in_stack_00001354;
          pbVar9 = std::operator<<(pbVar9,in_stack_00001350);
          pbVar9 = std::operator<<(pbVar9,pcVar21);
          pbVar9 = std::operator<<(pbVar9,pcVar10);
          std::operator<<(pbVar9,cVar23);
          iVar7 = 0x9c;
          pcVar21 = s_E__work_ht_3de_3DE_Morph_cpp_100377bc;
          pcVar10 = Housemarque::Kernel::Error::Tmp_Str();
          Housemarque::Kernel::Throw_Error(s_mesh___mesh__10037810,pcVar10,pcVar21,iVar7);
        }
        if ((uVar13 & 0x10) != 0) {
          uVar13 = uVar13 & 0xffffffef;
          Housemarque::Kernel::Inserter::~Inserter((Inserter *)&pbStack_8);
        }
        pbVar11 = Housemarque::Legacy::Segment::Stream();
        Housemarque::Template_Library::operator>>((basic_istream<> *)pbVar11,(Config_Base *)param_7)
        ;
        psVar16 = (srGFInterface *)operator_new(0x28);
        if (psVar16 == (srGFInterface *)0x0) {
          psVar16 = (srGFInterface *)0x0;
        }
        else {
          pcVar15 = *(code **)(param_7 + 0x10);
          if (*(code **)(param_7 + 0x10) == (code *)0x0) {
            pcVar15 = _C_exref;
          }
          psVar16 = (srGFInterface *)srGFInterface::srGFInterface(psVar16,(char *)pcVar15);
        }
        srGFInterface::loadMaterials();
        Housemarque::Legacy::Segment::~Segment((Segment *)&stack0x000002cc);
        Housemarque::Legacy::Segment::~Segment((Segment *)&stack0x000000ec);
      }
      Housemarque::Legacy::Segment::~Segment((Segment *)&stack0x0000022c);
      pbVar11 = Housemarque::Legacy::Segment::Stream();
    } while (((byte)pbVar11[*(int *)(*(int *)pbVar11 + 4) + 4] & 1) == 0);
    if (psVar16 != (srGFInterface *)0x0) goto LAB_10017646;
  }
  uVar13 = uVar13 | 0x20;
  local_4 = uVar13;
  puVar8 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&pbStack_8);
  pbVar9 = std::operator<<((basic_ostream<> *)*puVar8,s_3de__Invalid_morph_config_file__10037864);
  cVar23 = '\0';
  pcVar21 = s___Cant_find_settings_for_handle__10037840;
  pcVar10 = in_stack_00001354;
  pbVar9 = std::operator<<(pbVar9,in_stack_00001350);
  pbVar9 = std::operator<<(pbVar9,pcVar21);
  pbVar9 = std::operator<<(pbVar9,pcVar10);
  std::operator<<(pbVar9,cVar23);
  iVar7 = 0xa4;
  pcVar21 = s_E__work_ht_3de_3DE_Morph_cpp_10037820;
  pcVar10 = Housemarque::Kernel::Error::Tmp_Str();
  Housemarque::Kernel::Throw_Error(s_mesh_file_10037888,pcVar10,pcVar21,iVar7);
LAB_10017646:
  if ((uVar13 & 0x20) != 0) {
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&pbStack_8);
  }
  param_3 = (char *)0x0;
  param_4 = 0;
  param_5 = 0;
  plVar12 = *(list<> **)(param_7 + 0x20);
  if (*(list<> **)(param_7 + 0x20) == (list<> *)0x0) {
    plVar12 = (list<> *)_C_exref;
  }
  this_00 = FUN_1001ae40((char *)psVar16,(reOrderData *)&stack0x00000030,plVar12,in_stack_00001360,
                         (float *)&param_3,in_stack_00001368,0);
  *(srMeshModel **)(param_7 + 0x3c) = this_00;
  srRuntimeClass::setName((srRuntimeClass *)this_00,in_stack_00001354);
  FUN_1001b140(*(srMeshModel **)(param_7 + 0x3c),'\x01');
  if (psVar16 != (srGFInterface *)0x0) {
    srGFInterface::~srGFInterface(psVar16);
    FUN_100212a4(psVar16);
  }
  bVar2 = (&stack0x00000060)[*(int *)(pcStack0000005c + 4)];
  while ((bVar2 & 1) == 0) {
    Housemarque::Legacy::Segment::Segment
              ((Segment *)&stack0x000000ec,(basic_istream<> *)&stack0x0000005c,'{','}',';');
    bVar6 = std::operator==((basic_string<> *)
                            (-(uint)(&stack0x00000000 != (undefined *)0xffffff14) &
                            (uint)&stack0x000000f0),s_sequence_10037894);
    if (bVar6) {
      pVVar17 = param_7 + 0x48;
      pfVar18 = in_stack_00001360;
      pfVar19 = in_stack_00001364;
      pfVar22 = in_stack_00001368;
      pcVar10 = in_stack_00001354;
      pbVar11 = Housemarque::Legacy::Segment::Stream();
      FUN_10018d30(&stack0x0000036c,(basic_istream<> *)pbVar11,(HTSC_Node_Base *)pVVar17,pfVar18,
                   pfVar19,pfVar22,pcVar10);
      pbStack_8 = (basic_string<> *)operator_new(0x2c);
      if (pbStack_8 != (basic_string<> *)0x0) {
        FUN_10019690(pbStack_8,(int)&stack0x0000036c);
      }
      Housemarque::Template_Library::HTSC_Table_Base::Add((HTSC_Node_Base *)(param_7 + 0x40));
      FUN_10019400((Config_Base *)&stack0x0000036c);
    }
    Housemarque::Legacy::Segment::~Segment((Segment *)&stack0x000000ec);
    bVar2 = (&stack0x00000060)[*(int *)(pcStack0000005c + 4)];
  }
  Housemarque::Legacy::Segment::~Segment((Segment *)&stack0x0000018c);
  pbStack_8 = (basic_string<> *)&stack0x000000b8;
  *(code **)((int)&stack0x0000005c + *(int *)(pcStack0000005c + 4)) = _vftable__exref;
  param_1 = (Resource_Handler *)&stack0x00000064;
  pcStack00000064 = _vftable__exref;
  if (((in_stack_000000ac != '\0') && (in_stack_000000b4 != (FILE *)0x0)) &&
     (iVar7 = fclose(in_stack_000000b4), iVar7 == 0)) {
    std::basic_filebuf<>::_Init((basic_filebuf<> *)&stack0x00000064,(_iobuf *)0x0,2);
  }
  if (in_stack_000000a8 != (void *)0x0) {
    FUN_10015120(in_stack_000000a8,1);
  }
  std::locale::~locale((locale *)&stack0x000000b0);
  std::basic_streambuf<>::~basic_streambuf<>((basic_streambuf<> *)&stack0x00000064);
  *(code **)((int)&stack0x0000005c + *(int *)(pcStack0000005c + 4)) = _vftable__exref;
  pcStack000000b8 = _vftable__exref;
  std::ios_base::~ios_base((ios_base *)&stack0x000000b8);
  piVar5 = piStack00000034;
  piVar4 = (int *)*piStack00000034;
  while (piVar4 != piVar5) {
    piVar3 = (int *)*piVar4;
    *(int *)piVar4[1] = *piVar4;
    *(int *)(*piVar4 + 4) = piVar4[1];
    FUN_100212a4(piVar4);
    iStack00000038 = iStack00000038 + -1;
    piVar4 = piVar3;
  }
  FUN_100212a4(piStack00000034);
  ExceptionList = in_stack_0000133c;
  return param_7;
}



// public: __thiscall Housemarque::Threedee_Engine::Morph_Data::Morph_Data(class
// Housemarque::Threedee_Engine::Resource_Handler *,class std::basic_ifstream<char,struct
// std::char_traits<char> > &,char const *,char const *,unsigned int,unsigned int)

Morph_Data * __thiscall
Housemarque::Threedee_Engine::Morph_Data::Morph_Data
          (Morph_Data *this,Resource_Handler *param_1,basic_ifstream<> *param_2,char *param_3,
          char *param_4,uint param_5,uint param_6)

{
  basic_string<> *pbVar1;
  basic_ifstream<> bVar2;
  int *piVar3;
  int *piVar4;
  srGFInterface *this_00;
  int *piVar5;
  bool bVar6;
  int iVar7;
  undefined4 *puVar8;
  basic_ostream<> *pbVar9;
  char *pcVar10;
  basic_stringstream<> *pbVar11;
  list<> *plVar12;
  srMeshModel *this_01;
  void *pvVar13;
  uint uVar14;
  uint uVar15;
  code *pcVar16;
  uint unaff_EDI;
  char cVar17;
  char *pcVar18;
  char cVar19;
  char cVar20;
  undefined uStack_4a5;
  basic_string<> *pbStack_4a4;
  uint local_4a0;
  Morph_Data *local_49c;
  srGFInterface *psStack_498;
  code **ppcStack_494;
  Inserter aIStack_490 [4];
  float fStack_48c;
  char *pcStack_488;
  undefined4 uStack_484;
  basic_string<> abStack_47c [4];
  char *pcStack_478;
  basic_istream<> abStack_46c [4];
  int *piStack_468;
  int iStack_464;
  float fStack_460;
  char *pcStack_45c;
  undefined4 uStack_458;
  basic_string<> abStack_450 [4];
  char *pcStack_44c;
  code *pcStack_440;
  byte abStack_43c [4];
  code *apcStack_438 [17];
  basic_string<> *pbStack_3f4;
  char cStack_3f0;
  locale alStack_3ec [4];
  FILE *pFStack_3e8;
  code *apcStack_3e4 [13];
  float afStack_3b0 [9];
  Segment aSStack_38c [4];
  undefined auStack_388 [156];
  Segment aSStack_2ec [4];
  undefined auStack_2e8 [156];
  char acStack_24c [256];
  Segment aSStack_14c [4];
  undefined auStack_148 [156];
  Segment aSStack_ac [4];
  undefined auStack_a8 [156];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0x17950  15
                    // ??0Morph_Data@Threedee_Engine@Housemarque@@QAE@PAVResource_Handler@12@AAV?$basic_ifstream@DU?$char_traits@D@std@@@std@@PBD2II@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10023cac;
  pvStack_c = ExceptionList;
  local_4a0 = 0;
  ExceptionList = &pvStack_c;
  local_49c = this;
  Housemarque::Template_Library::Config_Base::Config_Base((Config_Base *)this);
  uStack_4 = 0;
  fStack_48c = (float)CONCAT31(fStack_48c._1_3_,uStack_4a5);
  std::basic_string<>::_Tidy((basic_string<> *)&fStack_48c,false);
  uVar14 = 0xffffffff;
  pcVar10 = s_not_defined_100378a0;
  do {
    if (uVar14 == 0) break;
    uVar14 = uVar14 - 1;
    cVar19 = *pcVar10;
    pcVar10 = pcVar10 + 1;
  } while (cVar19 != '\0');
  uVar14 = ~uVar14 - 1;
  bVar6 = std::basic_string<>::_Grow((basic_string<> *)&fStack_48c,uVar14,true);
  if (bVar6) {
    pcVar10 = s_not_defined_100378a0;
    pcVar18 = pcStack_488;
    for (uVar15 = uVar14 >> 2; uVar15 != 0; uVar15 = uVar15 - 1) {
      *(undefined4 *)pcVar18 = *(undefined4 *)pcVar10;
      pcVar10 = pcVar10 + 4;
      pcVar18 = pcVar18 + 4;
    }
    for (uVar15 = uVar14 & 3; uVar15 != 0; uVar15 = uVar15 - 1) {
      *pcVar18 = *pcVar10;
      pcVar10 = pcVar10 + 1;
      pcVar18 = pcVar18 + 1;
    }
    std::basic_string<>::_Eos((basic_string<> *)&fStack_48c,uVar14);
  }
  uStack_4 = CONCAT31(uStack_4._1_3_,1);
  std::basic_string<>::_Tidy(abStack_47c,false);
  uVar14 = 0xffffffff;
  pcVar10 = s_hx_name_100378ac;
  do {
    if (uVar14 == 0) break;
    uVar14 = uVar14 - 1;
    cVar19 = *pcVar10;
    pcVar10 = pcVar10 + 1;
  } while (cVar19 != '\0');
  uVar14 = ~uVar14 - 1;
  bVar6 = std::basic_string<>::_Grow(abStack_47c,uVar14,true);
  if (bVar6) {
    pcVar10 = s_hx_name_100378ac;
    pcVar18 = pcStack_478;
    for (uVar15 = uVar14 >> 2; uVar15 != 0; uVar15 = uVar15 - 1) {
      *(undefined4 *)pcVar18 = *(undefined4 *)pcVar10;
      pcVar10 = pcVar10 + 4;
      pcVar18 = pcVar18 + 4;
    }
    for (uVar15 = uVar14 & 3; uVar15 != 0; uVar15 = uVar15 - 1) {
      *pcVar18 = *pcVar10;
      pcVar10 = pcVar10 + 1;
      pcVar18 = pcVar18 + 1;
    }
    std::basic_string<>::_Eos(abStack_47c,uVar14);
  }
  pbVar1 = (basic_string<> *)(this + 0xc);
  uStack_4._0_1_ = 2;
  *pbVar1 = fStack_48c._0_1_;
  pbStack_4a4 = pbVar1;
  std::basic_string<>::_Tidy(pbVar1,false);
  std::basic_string<>::assign(pbVar1,(basic_string<> *)&fStack_48c,0,*(uint *)npos_exref);
  uStack_4._0_1_ = 3;
  ppcStack_494 = (code **)operator_new(0x20);
  uStack_4._0_1_ = 4;
  if (ppcStack_494 != (code **)0x0) {
    FUN_10014030(ppcStack_494,abStack_47c,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 7;
  std::basic_string<>::_Tidy(abStack_47c,true);
  uStack_4 = CONCAT31(uStack_4._1_3_,6);
  std::basic_string<>::_Tidy((basic_string<> *)&fStack_48c,true);
  fStack_460 = (float)CONCAT31(fStack_460._1_3_,uStack_4a5);
  std::basic_string<>::_Tidy((basic_string<> *)&fStack_460,false);
  uVar14 = 0xffffffff;
  pcVar10 = s_not_defined_100378b4;
  do {
    if (uVar14 == 0) break;
    uVar14 = uVar14 - 1;
    cVar19 = *pcVar10;
    pcVar10 = pcVar10 + 1;
  } while (cVar19 != '\0');
  uVar14 = ~uVar14 - 1;
  bVar6 = std::basic_string<>::_Grow((basic_string<> *)&fStack_460,uVar14,true);
  if (bVar6) {
    pcVar10 = s_not_defined_100378b4;
    for (uVar15 = uVar14 >> 2; uVar15 != 0; uVar15 = uVar15 - 1) {
      *(undefined4 *)pcStack_45c = *(undefined4 *)pcVar10;
      pcVar10 = pcVar10 + 4;
      pcStack_45c = pcStack_45c + 4;
    }
    for (uVar15 = uVar14 & 3; uVar15 != 0; uVar15 = uVar15 - 1) {
      *pcStack_45c = *pcVar10;
      pcVar10 = pcVar10 + 1;
      pcStack_45c = pcStack_45c + 1;
    }
    std::basic_string<>::_Eos((basic_string<> *)&fStack_460,uVar14);
  }
  uStack_4 = CONCAT31(uStack_4._1_3_,8);
  std::basic_string<>::_Tidy(abStack_450,false);
  uVar14 = 0xffffffff;
  pcVar10 = s_default_mesh_name_100378c0;
  do {
    if (uVar14 == 0) break;
    uVar14 = uVar14 - 1;
    cVar19 = *pcVar10;
    pcVar10 = pcVar10 + 1;
  } while (cVar19 != '\0');
  uVar14 = ~uVar14 - 1;
  bVar6 = std::basic_string<>::_Grow(abStack_450,uVar14,true);
  if (bVar6) {
    pcVar10 = s_default_mesh_name_100378c0;
    for (uVar15 = uVar14 >> 2; uVar15 != 0; uVar15 = uVar15 - 1) {
      *(undefined4 *)pcStack_44c = *(undefined4 *)pcVar10;
      pcVar10 = pcVar10 + 4;
      pcStack_44c = pcStack_44c + 4;
    }
    for (uVar15 = uVar14 & 3; uVar15 != 0; uVar15 = uVar15 - 1) {
      *pcStack_44c = *pcVar10;
      pcVar10 = pcVar10 + 1;
      pcStack_44c = pcStack_44c + 1;
    }
    std::basic_string<>::_Eos(abStack_450,uVar14);
  }
  pbVar1 = (basic_string<> *)(this + 0x1c);
  uStack_4._0_1_ = 9;
  *pbVar1 = fStack_460._0_1_;
  pbStack_4a4 = pbVar1;
  std::basic_string<>::_Tidy(pbVar1,false);
  std::basic_string<>::assign(pbVar1,(basic_string<> *)&fStack_460,0,*(uint *)npos_exref);
  uStack_4._0_1_ = 10;
  ppcStack_494 = (code **)operator_new(0x20);
  uStack_4._0_1_ = 0xb;
  if (ppcStack_494 != (code **)0x0) {
    FUN_10014030(ppcStack_494,abStack_450,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0xe;
  std::basic_string<>::_Tidy(abStack_450,true);
  uStack_4 = CONCAT31(uStack_4._1_3_,0xd);
  std::basic_string<>::_Tidy((basic_string<> *)&fStack_460,true);
  fStack_48c = (float)CONCAT31(fStack_48c._1_3_,uStack_4a5);
  std::basic_string<>::_Tidy((basic_string<> *)&fStack_48c,false);
  uVar14 = 0xffffffff;
  pcVar10 = s_not_defined_100378d4;
  do {
    if (uVar14 == 0) break;
    uVar14 = uVar14 - 1;
    cVar19 = *pcVar10;
    pcVar10 = pcVar10 + 1;
  } while (cVar19 != '\0');
  uVar14 = ~uVar14 - 1;
  bVar6 = std::basic_string<>::_Grow((basic_string<> *)&fStack_48c,uVar14,true);
  if (bVar6) {
    pcVar10 = s_not_defined_100378d4;
    for (uVar15 = uVar14 >> 2; uVar15 != 0; uVar15 = uVar15 - 1) {
      *(undefined4 *)pcStack_488 = *(undefined4 *)pcVar10;
      pcVar10 = pcVar10 + 4;
      pcStack_488 = pcStack_488 + 4;
    }
    for (uVar15 = uVar14 & 3; uVar15 != 0; uVar15 = uVar15 - 1) {
      *pcStack_488 = *pcVar10;
      pcVar10 = pcVar10 + 1;
      pcStack_488 = pcStack_488 + 1;
    }
    std::basic_string<>::_Eos((basic_string<> *)&fStack_48c,uVar14);
  }
  uStack_4 = CONCAT31(uStack_4._1_3_,0xf);
  std::basic_string<>::_Tidy(abStack_47c,false);
  uVar14 = 0xffffffff;
  pcVar10 = s_material_path_100378e0;
  do {
    if (uVar14 == 0) break;
    uVar14 = uVar14 - 1;
    cVar19 = *pcVar10;
    pcVar10 = pcVar10 + 1;
  } while (cVar19 != '\0');
  uVar14 = ~uVar14 - 1;
  bVar6 = std::basic_string<>::_Grow(abStack_47c,uVar14,true);
  if (bVar6) {
    pcVar10 = s_material_path_100378e0;
    for (uVar15 = uVar14 >> 2; uVar15 != 0; uVar15 = uVar15 - 1) {
      *(undefined4 *)pcStack_478 = *(undefined4 *)pcVar10;
      pcVar10 = pcVar10 + 4;
      pcStack_478 = pcStack_478 + 4;
    }
    for (uVar15 = uVar14 & 3; uVar15 != 0; uVar15 = uVar15 - 1) {
      *pcStack_478 = *pcVar10;
      pcVar10 = pcVar10 + 1;
      pcStack_478 = pcStack_478 + 1;
    }
    std::basic_string<>::_Eos(abStack_47c,uVar14);
  }
  pbVar1 = (basic_string<> *)(this + 0x2c);
  uStack_4._0_1_ = 0x10;
  *pbVar1 = fStack_48c._0_1_;
  pbStack_4a4 = pbVar1;
  std::basic_string<>::_Tidy(pbVar1,false);
  std::basic_string<>::assign(pbVar1,(basic_string<> *)&fStack_48c,0,*(uint *)npos_exref);
  uStack_4._0_1_ = 0x11;
  ppcStack_494 = (code **)operator_new(0x20);
  uStack_4._0_1_ = 0x12;
  if (ppcStack_494 != (code **)0x0) {
    FUN_10014030(ppcStack_494,abStack_47c,(Config_Base *)this,pbVar1);
  }
  uStack_4._0_1_ = 0x15;
  std::basic_string<>::_Tidy(abStack_47c,true);
  uStack_4._0_1_ = 0x14;
  std::basic_string<>::_Tidy((basic_string<> *)&fStack_48c,true);
  *(undefined4 *)(this + 0x3c) = 0;
  Housemarque::Template_Library::HTSC_Table_Base::HTSC_Table_Base
            ((HTSC_Table_Base *)(this + 0x40),param_6);
  uStack_4._0_1_ = 0x16;
  Housemarque::Template_Library::HTSC_Table_Base::HTSC_Table_Base
            ((HTSC_Table_Base *)(this + 0x48),param_5);
  uStack_4._0_1_ = 0x17;
  piStack_468 = (int *)operator_new(0xc);
  *piStack_468 = (int)piStack_468;
  piStack_468[1] = (int)piStack_468;
  iStack_464 = 0;
  uStack_4._0_1_ = 0x18;
  psStack_498 = (srGFInterface *)0x0;
  pcStack_440 = _vbtable__exref;
  std::ios_base::ios_base((ios_base *)apcStack_3e4);
  uVar14 = 0x40;
  apcStack_3e4[0] = _vftable__exref;
  local_4a0 = 0x40;
  uStack_4 = CONCAT31(uStack_4._1_3_,0x19);
  std::basic_istream<>::basic_istream<>
            ((basic_istream<> *)&pcStack_440,(basic_streambuf<> *)apcStack_438,false);
  uStack_4 = 0x1a;
  std::basic_filebuf<>::basic_filebuf<>((basic_filebuf<> *)apcStack_438,(_iobuf *)0x0);
  *(code **)(abStack_43c + *(int *)(pcStack_440 + 4) + -4) = _vftable__exref;
  uStack_4 = CONCAT31(uStack_4._1_3_,0x1b);
  iVar7 = std::basic_filebuf<>::open(param_3,1);
  if (iVar7 == 0) {
    std::basic_ios<>::setstate
              ((basic_ios<> *)(abStack_43c + *(int *)(pcStack_440 + 4) + -4),2,false);
  }
  uStack_4._0_1_ = 0x1c;
  if ((abStack_43c[*(int *)(pcStack_440 + 4)] & 6) != 0) {
    uVar14 = 0x41;
    local_4a0 = 0x41;
    puVar8 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(aIStack_490);
    uStack_4 = CONCAT31(uStack_4._1_3_,0x1d);
    pbVar9 = std::operator<<((basic_ostream<> *)*puVar8,s_3de__cant_open__Morph_Data__conf_10037910)
    ;
    cVar19 = '\0';
    pbVar9 = std::operator<<(pbVar9,param_3);
    std::operator<<(pbVar9,cVar19);
    iVar7 = 0xd0;
    pcVar18 = s_E__work_ht_3de_3DE_Morph_cpp_100378f0;
    pcVar10 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(s__config_file_fail___1003793c,pcVar10,pcVar18,iVar7);
  }
  uStack_4 = 0x1c;
  if ((uVar14 & 1) != 0) {
    uVar14 = uVar14 & 0xfffffffe;
    Housemarque::Kernel::Inserter::~Inserter(aIStack_490);
  }
  Housemarque::Legacy::Segment::Segment(aSStack_38c,(basic_istream<> *)&pcStack_440,'{','}',';');
  uStack_4._0_1_ = 0x1e;
  bVar6 = std::operator==((basic_string<> *)
                          (-(uint)(&stack0x00000000 != (undefined *)0x38c) & (uint)auStack_388),
                          s_settings_10037950);
  if (!bVar6) {
    uVar14 = uVar14 | 2;
    local_4a0 = uVar14;
    puVar8 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(aIStack_490);
    uStack_4 = CONCAT31(uStack_4._1_3_,0x1f);
    pbVar9 = std::operator<<((basic_ostream<> *)*puVar8,s_3de__Morph_Data__illegal_config_f_1003797c
                            );
    cVar19 = '\0';
    pbVar9 = std::operator<<(pbVar9,param_3);
    std::operator<<(pbVar9,cVar19);
    iVar7 = 0xd3;
    pcVar18 = s_E__work_ht_3de_3DE_Morph_cpp_1003795c;
    pcVar10 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(s_mesh_data_file___settings__100379a4,pcVar10,pcVar18,iVar7);
  }
  uStack_4 = 0x1e;
  if ((uVar14 & 2) != 0) {
    uVar14 = uVar14 & 0xfffffffd;
    Housemarque::Kernel::Inserter::~Inserter(aIStack_490);
  }
  pbVar11 = Housemarque::Legacy::Segment::Stream();
  if ((*(uint *)(pbVar11 + *(int *)(*(int *)pbVar11 + 4) + 4) & 1) == 0) {
    do {
      cVar20 = ';';
      cVar17 = '}';
      cVar19 = '{';
      pbVar11 = Housemarque::Legacy::Segment::Stream();
      Housemarque::Legacy::Segment::Segment
                (aSStack_2ec,(basic_istream<> *)pbVar11,cVar19,cVar17,cVar20);
      uStack_4._0_1_ = 0x20;
      bVar6 = std::operator==((basic_string<> *)
                              (-(uint)(&stack0x00000000 != (undefined *)0x2ec) & (uint)auStack_2e8),
                              param_4);
      if (bVar6) {
        cVar20 = ';';
        cVar17 = '}';
        cVar19 = '{';
        pbVar11 = Housemarque::Legacy::Segment::Stream();
        Housemarque::Legacy::Segment::Segment
                  (aSStack_ac,(basic_istream<> *)pbVar11,cVar19,cVar17,cVar20);
        uStack_4._0_1_ = 0x21;
        bVar6 = std::operator==((basic_string<> *)
                                (-(uint)(&stack0x00000000 != (undefined *)0xac) & (uint)auStack_a8),
                                s_material_list_100379c0);
        if (!bVar6) {
          uVar14 = uVar14 | 4;
          local_4a0 = uVar14;
          puVar8 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(aIStack_490);
          uStack_4 = CONCAT31(uStack_4._1_3_,0x22);
          pbVar9 = std::operator<<((basic_ostream<> *)*puVar8,
                                   s_Invalid_settings_segment_in_file_10037a00);
          cVar19 = '\0';
          pcVar18 = s_with_handle__100379f0;
          pcVar10 = param_4;
          pbVar9 = std::operator<<(pbVar9,param_3);
          pbVar9 = std::operator<<(pbVar9,pcVar18);
          pbVar9 = std::operator<<(pbVar9,pcVar10);
          std::operator<<(pbVar9,cVar19);
          iVar7 = 0xdc;
          pcVar18 = s_E__work_ht_3de_3DE_Morph_cpp_100379d0;
          pcVar10 = Housemarque::Kernel::Error::Tmp_Str();
          Housemarque::Kernel::Throw_Error
                    (s_material_list___material_list__10037a24,pcVar10,pcVar18,iVar7);
        }
        uStack_4 = 0x21;
        if ((uVar14 & 4) != 0) {
          uVar14 = uVar14 & 0xfffffffb;
          Housemarque::Kernel::Inserter::~Inserter(aIStack_490);
        }
        pbVar11 = Housemarque::Legacy::Segment::Stream();
        Resource_Handler::Import_Material_List((list<> *)param_1,abStack_46c,SUB41(pbVar11,0));
        pbVar11 = Housemarque::Legacy::Segment::Stream();
        if (((byte)pbVar11[*(int *)(*(int *)pbVar11 + 4) + 4] & 1) != 0) {
          uVar14 = uVar14 | 8;
          local_4a0 = uVar14;
          puVar8 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&ppcStack_494);
          uStack_4 = CONCAT31(uStack_4._1_3_,0x23);
          pbVar9 = std::operator<<((basic_ostream<> *)*puVar8,
                                   s_Invalid_settings_segment_in_file_10037a74);
          cVar19 = '\0';
          pcVar18 = s_with_handle__10037a64;
          pcVar10 = param_4;
          pbVar9 = std::operator<<(pbVar9,param_3);
          pbVar9 = std::operator<<(pbVar9,pcVar18);
          pbVar9 = std::operator<<(pbVar9,pcVar10);
          std::operator<<(pbVar9,cVar19);
          iVar7 = 0xe0;
          pcVar18 = s_E__work_ht_3de_3DE_Morph_cpp_10037a44;
          pcVar10 = Housemarque::Kernel::Error::Tmp_Str();
          Housemarque::Kernel::Throw_Error(s__data_Stream___eof___10037a98,pcVar10,pcVar18,iVar7);
        }
        uStack_4 = 0x21;
        if ((uVar14 & 8) != 0) {
          uVar14 = uVar14 & 0xfffffff7;
          Housemarque::Kernel::Inserter::~Inserter((Inserter *)&ppcStack_494);
        }
        cVar20 = ';';
        cVar17 = '}';
        cVar19 = '{';
        pbVar11 = Housemarque::Legacy::Segment::Stream();
        Housemarque::Legacy::Segment::Segment
                  (aSStack_14c,(basic_istream<> *)pbVar11,cVar19,cVar17,cVar20);
        uStack_4._0_1_ = 0x24;
        bVar6 = std::operator==((basic_string<> *)
                                (-(uint)(&stack0x00000000 != (undefined *)0x14c) & (uint)auStack_148
                                ),&DAT_10037ab0);
        if (!bVar6) {
          uVar14 = uVar14 | 0x10;
          local_4a0 = uVar14;
          puVar8 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&pbStack_4a4);
          uStack_4 = CONCAT31(uStack_4._1_3_,0x25);
          pbVar9 = std::operator<<((basic_ostream<> *)*puVar8,
                                   s_Invalid_settings_segment_in_file_10037ae8);
          cVar19 = '\0';
          pcVar18 = s_with_handle__10037ad8;
          pcVar10 = param_4;
          pbVar9 = std::operator<<(pbVar9,param_3);
          pbVar9 = std::operator<<(pbVar9,pcVar18);
          pbVar9 = std::operator<<(pbVar9,pcVar10);
          std::operator<<(pbVar9,cVar19);
          iVar7 = 0xe3;
          pcVar18 = s_E__work_ht_3de_3DE_Morph_cpp_10037ab8;
          pcVar10 = Housemarque::Kernel::Error::Tmp_Str();
          Housemarque::Kernel::Throw_Error(s_mesh___mesh__10037b0c,pcVar10,pcVar18,iVar7);
        }
        uStack_4 = 0x24;
        if ((uVar14 & 0x10) != 0) {
          uVar14 = uVar14 & 0xffffffef;
          Housemarque::Kernel::Inserter::~Inserter((Inserter *)&pbStack_4a4);
        }
        pbVar11 = Housemarque::Legacy::Segment::Stream();
        Housemarque::Template_Library::operator>>
                  ((basic_istream<> *)pbVar11,(Config_Base *)local_49c);
        if (psStack_498 != (srGFInterface *)0x0) {
          srGFInterface::~srGFInterface(psStack_498);
          FUN_100212a4(psStack_498);
        }
        psStack_498 = (srGFInterface *)operator_new(0x28);
        uStack_4._0_1_ = 0x26;
        if (psStack_498 == (srGFInterface *)0x0) {
          psStack_498 = (srGFInterface *)0x0;
        }
        else {
          pcVar16 = *(code **)(local_49c + 0x10);
          if (*(code **)(local_49c + 0x10) == (code *)0x0) {
            pcVar16 = _C_exref;
          }
          psStack_498 = (srGFInterface *)srGFInterface::srGFInterface(psStack_498,(char *)pcVar16);
        }
        uStack_4._0_1_ = 0x24;
        srGFInterface::loadMaterials();
        uStack_4._0_1_ = 0x21;
        Housemarque::Legacy::Segment::~Segment(aSStack_14c);
        uStack_4._0_1_ = 0x20;
        Housemarque::Legacy::Segment::~Segment(aSStack_ac);
      }
      uStack_4 = CONCAT31(uStack_4._1_3_,0x1e);
      Housemarque::Legacy::Segment::~Segment(aSStack_2ec);
      pbVar11 = Housemarque::Legacy::Segment::Stream();
    } while (((byte)pbVar11[*(int *)(*(int *)pbVar11 + 4) + 4] & 1) == 0);
    if (psStack_498 != (srGFInterface *)0x0) goto LAB_100183ec;
  }
  uVar14 = uVar14 | 0x20;
  local_4a0 = uVar14;
  puVar8 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&pbStack_4a4);
  uStack_4 = CONCAT31(uStack_4._1_3_,0x27);
  pbVar9 = std::operator<<((basic_ostream<> *)*puVar8,s_3de__Invalid_morph_config_file__10037b60);
  cVar19 = '\0';
  pcVar18 = s___Cant_find_settings_for_handle__10037b3c;
  pcVar10 = param_4;
  pbVar9 = std::operator<<(pbVar9,param_3);
  pbVar9 = std::operator<<(pbVar9,pcVar18);
  pbVar9 = std::operator<<(pbVar9,pcVar10);
  std::operator<<(pbVar9,cVar19);
  iVar7 = 0xec;
  pcVar18 = s_E__work_ht_3de_3DE_Morph_cpp_10037b1c;
  pcVar10 = Housemarque::Kernel::Error::Tmp_Str();
  Housemarque::Kernel::Throw_Error(s_mesh_file_10037b84,pcVar10,pcVar18,iVar7);
LAB_100183ec:
  uStack_4 = 0x1e;
  if ((uVar14 & 0x20) != 0) {
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&pbStack_4a4);
  }
  Housemarque::Template_Library::M3x3_Idn(afStack_3b0);
  this_00 = psStack_498;
  fStack_460 = 0.01;
  pcStack_45c = (char *)0x3c23d70a;
  uStack_458 = 0x3c23d70a;
  fStack_48c = 0.0;
  pcStack_488 = (char *)0x0;
  uStack_484 = 0;
  plVar12 = *(list<> **)(local_49c + 0x20);
  if (*(list<> **)(local_49c + 0x20) == (list<> *)0x0) {
    plVar12 = (list<> *)_C_exref;
  }
  this_01 = FUN_1001ae40((char *)psStack_498,(reOrderData *)abStack_46c,plVar12,afStack_3b0,
                         &fStack_48c,&fStack_460,0);
  *(srMeshModel **)(local_49c + 0x3c) = this_01;
  srRuntimeClass::setName((srRuntimeClass *)this_01,param_4);
  if (this_00 != (srGFInterface *)0x0) {
    srGFInterface::~srGFInterface(this_00);
    FUN_100212a4(this_00);
  }
  pcVar10 = acStack_24c;
  for (iVar7 = 0x40; iVar7 != 0; iVar7 = iVar7 + -1) {
    pcVar10[0] = '\0';
    pcVar10[1] = '\0';
    pcVar10[2] = '\0';
    pcVar10[3] = '\0';
    pcVar10 = pcVar10 + 4;
  }
  std::basic_istream<>::getline((basic_istream<> *)param_2,acStack_24c,0x100,'\0');
  pcVar10 = acStack_24c;
  for (iVar7 = 0x40; iVar7 != 0; iVar7 = iVar7 + -1) {
    pcVar10[0] = '\0';
    pcVar10[1] = '\0';
    pcVar10[2] = '\0';
    pcVar10[3] = '\0';
    pcVar10 = pcVar10 + 4;
  }
  std::basic_istream<>::getline((basic_istream<> *)param_2,acStack_24c,0x100,'\0');
  bVar2 = param_2[*(int *)(*(int *)param_2 + 4) + 4];
  while (((byte)bVar2 & 1) == 0) {
    pcVar10 = acStack_24c;
    for (iVar7 = 0x40; iVar7 != 0; iVar7 = iVar7 + -1) {
      pcVar10[0] = '\0';
      pcVar10[1] = '\0';
      pcVar10[2] = '\0';
      pcVar10[3] = '\0';
      pcVar10 = pcVar10 + 4;
    }
    std::basic_istream<>::getline((basic_istream<> *)param_2,acStack_24c,0x100,'\0');
    iVar7 = -1;
    pcVar10 = acStack_24c;
    do {
      if (iVar7 == 0) break;
      iVar7 = iVar7 + -1;
      cVar19 = *pcVar10;
      pcVar10 = pcVar10 + 1;
    } while (cVar19 != '\0');
    if (iVar7 == -2) break;
    pbStack_4a4 = (basic_string<> *)operator_new(0x2c);
    uStack_4._0_1_ = 0x28;
    if (pbStack_4a4 == (basic_string<> *)0x0) {
      pvVar13 = (void *)0x0;
    }
    else {
      pvVar13 = FUN_10019480(pbStack_4a4,(basic_istream<> *)param_2,acStack_24c);
    }
    uVar14 = 0;
    uStack_4 = CONCAT31(uStack_4._1_3_,0x1e);
    if (*(int *)((int)pvVar13 + 0x1c) != 0) {
      do {
        Housemarque::Template_Library::HTSC_Table_Base::Add((HTSC_Node_Base *)(local_49c + 0x48));
        uVar14 = uVar14 + 1;
      } while (uVar14 < *(uint *)((int)pvVar13 + 0x1c));
    }
    pcVar10 = acStack_24c;
    for (iVar7 = 0x40; iVar7 != 0; iVar7 = iVar7 + -1) {
      pcVar10[0] = '\0';
      pcVar10[1] = '\0';
      pcVar10[2] = '\0';
      pcVar10[3] = '\0';
      pcVar10 = pcVar10 + 4;
    }
    std::basic_istream<>::read((int)acStack_24c,(void *)0x3,unaff_EDI);
    Housemarque::Template_Library::HTSC_Table_Base::Add((HTSC_Node_Base *)(local_49c + 0x40));
    bVar2 = param_2[*(int *)(*(int *)param_2 + 4) + 4];
  }
  uStack_4._0_1_ = 0x1c;
  Housemarque::Legacy::Segment::~Segment(aSStack_38c);
  pbStack_4a4 = (basic_string<> *)apcStack_3e4;
  *(code **)(abStack_43c + *(int *)(pcStack_440 + 4) + -4) = _vftable__exref;
  ppcStack_494 = apcStack_438;
  apcStack_438[0] = _vftable__exref;
  uStack_4 = CONCAT31(uStack_4._1_3_,0x2b);
  if (((cStack_3f0 != '\0') && (pFStack_3e8 != (FILE *)0x0)) &&
     (iVar7 = fclose(pFStack_3e8), iVar7 == 0)) {
    std::basic_filebuf<>::_Init((basic_filebuf<> *)apcStack_438,(_iobuf *)0x0,2);
  }
  if (pbStack_3f4 != (basic_string<> *)0x0) {
    std::basic_string<>::_Tidy(pbStack_3f4,true);
    FUN_100212a4(pbStack_3f4);
  }
  std::locale::~locale(alStack_3ec);
  uStack_4._0_1_ = 0x29;
  std::basic_streambuf<>::~basic_streambuf<>((basic_streambuf<> *)apcStack_438);
  *(code **)(abStack_43c + *(int *)(pcStack_440 + 4) + -4) = _vftable__exref;
  uStack_4 = CONCAT31(uStack_4._1_3_,0x18);
  apcStack_3e4[0] = _vftable__exref;
  std::ios_base::~ios_base((ios_base *)apcStack_3e4);
  piVar5 = piStack_468;
  piVar4 = (int *)*piStack_468;
  while (piVar4 != piVar5) {
    piVar3 = (int *)*piVar4;
    *(int *)piVar4[1] = *piVar4;
    *(int *)(*piVar4 + 4) = piVar4[1];
    FUN_100212a4(piVar4);
    iStack_464 = iStack_464 + -1;
    piVar4 = piVar3;
  }
  FUN_100212a4(piStack_468);
  ExceptionList = pvStack_c;
  return local_49c;
}



// public: __thiscall Housemarque::Threedee_Engine::Morph_Data::~Morph_Data(void)

void __thiscall Housemarque::Threedee_Engine::Morph_Data::~Morph_Data(Morph_Data *this)

{
  char *pcVar1;
  char cVar2;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0x18740  36  ??1Morph_Data@Threedee_Engine@Housemarque@@QAE@XZ
  puStack_8 = &LAB_10023cf5;
  pvStack_c = ExceptionList;
  local_4 = 5;
  ExceptionList = &pvStack_c;
  srClass::release(*(srClass **)(this + 0x3c));
  local_4._0_1_ = 4;
  Housemarque::Template_Library::HTSC_Table_Base::~HTSC_Table_Base((HTSC_Table_Base *)(this + 0x48))
  ;
  local_4 = CONCAT31(local_4._1_3_,3);
  Housemarque::Template_Library::HTSC_Table_Base::~HTSC_Table_Base((HTSC_Table_Base *)(this + 0x40))
  ;
  if (*(int *)(this + 0x30) != 0) {
    pcVar1 = (char *)(*(int *)(this + 0x30) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(this + 0x30) = 0;
  *(undefined4 *)(this + 0x34) = 0;
  *(undefined4 *)(this + 0x38) = 0;
  if (*(int *)(this + 0x20) != 0) {
    pcVar1 = (char *)(*(int *)(this + 0x20) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(this + 0x20) = 0;
  *(undefined4 *)(this + 0x24) = 0;
  *(undefined4 *)(this + 0x28) = 0;
  if (*(int *)(this + 0x10) != 0) {
    pcVar1 = (char *)(*(int *)(this + 0x10) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(this + 0x10) = 0;
  *(undefined4 *)(this + 0x14) = 0;
  *(undefined4 *)(this + 0x18) = 0;
  local_4 = 0xffffffff;
  Housemarque::Template_Library::Config_Base::~Config_Base((Config_Base *)this);
  ExceptionList = pvStack_c;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Morph_Data::Export_Data(class
// std::basic_ostream<char,struct std::char_traits<char> > &,char const *)

void __fastcall
Housemarque::Threedee_Engine::Morph_Data::Export_Data(basic_ostream<> *param_1,char *param_2)

{
  char cVar1;
  code cVar2;
  int iVar3;
  DLIL_Node_Base *pDVar4;
  code *pcVar5;
  uint uVar6;
  code *_Filehandle;
  uint unaff_EBX;
  char *pcVar7;
  uint unaff_EDI;
  code *pcVar8;
  char *in_stack_00000004;
  uint uStack_c;
  
                    // 0x18840  70
                    // ?Export_Data@Morph_Data@Threedee_Engine@Housemarque@@QAIXAAV?$basic_ostream@DU?$char_traits@D@std@@@std@@PBD@Z
  uVar6 = 0xffffffff;
  pcVar7 = s_Morph_Data_Export_10037ba8;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar1 = *pcVar7;
    pcVar7 = pcVar7 + 1;
  } while (cVar1 != '\0');
  std::basic_ostream<>::write(0x10037ba8,(void *)(~uVar6 - 1),unaff_EDI);
  std::basic_ostream<>::write(0x100384f8,(void *)0x1,unaff_EDI);
  uVar6 = 0xffffffff;
  pcVar7 = in_stack_00000004;
  do {
    if (uVar6 == 0) break;
    uVar6 = uVar6 - 1;
    cVar1 = *pcVar7;
    pcVar7 = pcVar7 + 1;
  } while (cVar1 != '\0');
  std::basic_ostream<>::write((int)in_stack_00000004,(void *)(~uVar6 - 1),unaff_EDI);
  std::basic_ostream<>::write(0x100384fc,(void *)0x1,unaff_EDI);
  uStack_c = 0;
  if (*(int *)(param_1 + 0x44) != 0) {
    do {
      pDVar4 = Housemarque::Template_Library::DLIL_List_Base::Head();
      while ((pDVar4 != (DLIL_Node_Base *)0x0 &&
             (pDVar4 = pDVar4 + -4, pDVar4 != (DLIL_Node_Base *)0x0))) {
        pcVar5 = *(code **)(pDVar4 + 0x10);
        if (*(code **)(pDVar4 + 0x10) == (code *)0x0) {
          pcVar5 = _C_exref;
        }
        uVar6 = 0xffffffff;
        pcVar8 = pcVar5;
        do {
          if (uVar6 == 0) break;
          uVar6 = uVar6 - 1;
          cVar2 = *pcVar8;
          pcVar8 = pcVar8 + 1;
        } while (cVar2 != (code)0x0);
        std::basic_ostream<>::write((int)pcVar5,(void *)(~uVar6 - 1),unaff_EBX);
        std::basic_ostream<>::write(0x10038500,(void *)0x1,unaff_EBX);
        in_stack_00000004 = *(char **)(pDVar4 + 0x1c);
        std::basic_ostream<>::write((int)&stack0x00000004,(void *)0x4,unaff_EBX);
        pcVar7 = (char *)0x0;
        if (in_stack_00000004 != (char *)0x0) {
          do {
            iVar3 = *(int *)(*(int *)(pDVar4 + 0x20) + (int)pcVar7 * 4);
            pcVar5 = *(code **)(iVar3 + 0x28);
            if (pcVar5 == (code *)0x0) {
              pcVar5 = _C_exref;
            }
            pcVar8 = *(code **)(iVar3 + 0x28);
            _Filehandle = _C_exref;
            if (pcVar8 != (code *)0x0) {
              _Filehandle = pcVar8;
            }
            uVar6 = 0xffffffff;
            do {
              if (uVar6 == 0) break;
              uVar6 = uVar6 - 1;
              cVar2 = *pcVar5;
              pcVar5 = pcVar5 + 1;
            } while (cVar2 != (code)0x0);
            std::basic_ostream<>::write((int)_Filehandle,(void *)(~uVar6 - 1),unaff_EBX);
            std::basic_ostream<>::write(0x10038504,(void *)0x1,unaff_EBX);
            FUN_1001a100(*(void **)(*(int *)(pDVar4 + 0x20) + (int)pcVar7 * 4),param_2);
            pcVar7 = pcVar7 + 1;
          } while (pcVar7 < in_stack_00000004);
        }
        std::basic_ostream<>::write
                  (*(int *)(pDVar4 + 0x24),(void *)((int)in_stack_00000004 * 4),unaff_EBX);
        std::basic_ostream<>::write
                  (*(int *)(pDVar4 + 0x28),(void *)((int)in_stack_00000004 * 4),unaff_EBX);
        std::basic_ostream<>::write(0x10037bbc,(void *)0x3,unaff_EBX);
        pDVar4 = Housemarque::Template_Library::DLIL_Node_Base::Succ();
      }
      uStack_c = uStack_c + 1;
    } while (uStack_c < *(uint *)(param_1 + 0x44));
  }
  return;
}



// public: int __fastcall Housemarque::Threedee_Engine::Morph_Data::Get_Vertex_Count(void)

int __fastcall Housemarque::Threedee_Engine::Morph_Data::Get_Vertex_Count(void)

{
  int in_ECX;
  
                    // 0x18a00  98  ?Get_Vertex_Count@Morph_Data@Threedee_Engine@Housemarque@@QAIHXZ
  if (*(int *)(in_ECX + 0x3c) == 0) {
    Housemarque::Kernel::Throw_Error
              ((char *)0x0,s_default_model_10037be0,s_E__work_ht_3de_3DE_Morph_cpp_10037bc0,0x163);
    return *(int *)(*(int *)(in_ECX + 0x3c) + 0x22c);
  }
  return *(int *)(*(int *)(in_ECX + 0x3c) + 0x22c);
}



// public: int __fastcall Housemarque::Threedee_Engine::Morph_Data::Get_Polygon_Count(void)

int __fastcall Housemarque::Threedee_Engine::Morph_Data::Get_Polygon_Count(void)

{
  int in_ECX;
  
                    // 0x18a40  86
                    // ?Get_Polygon_Count@Morph_Data@Threedee_Engine@Housemarque@@QAIHXZ
  if (*(int *)(in_ECX + 0x3c) == 0) {
    Housemarque::Kernel::Throw_Error
              ((char *)0x0,s_default_model_10037c10,s_E__work_ht_3de_3DE_Morph_cpp_10037bf0,0x169);
    return *(int *)(*(int *)(in_ECX + 0x3c) + 0x230);
  }
  return *(int *)(*(int *)(in_ECX + 0x3c) + 0x230);
}



// public: float * __fastcall
// Housemarque::Threedee_Engine::Morph_Data::Get_Texture_Coordinates(void)

float * __fastcall Housemarque::Threedee_Engine::Morph_Data::Get_Texture_Coordinates(void)

{
  srVector2T<float> *psVar1;
  int in_ECX;
  
                    // 0x18a80  97
                    // ?Get_Texture_Coordinates@Morph_Data@Threedee_Engine@Housemarque@@QAIPAMXZ
  if (*(int *)(in_ECX + 0x3c) == 0) {
    Housemarque::Kernel::Throw_Error
              ((char *)0x0,s_default_model_10037c40,s_E__work_ht_3de_3DE_Morph_cpp_10037c20,0x16f);
  }
  psVar1 = srMeshModel::getVertexTexCoords(*(srMeshModel **)(in_ECX + 0x3c),0,0,1);
  return (float *)psVar1;
}



// public: int * __fastcall Housemarque::Threedee_Engine::Morph_Data::Get_Polygon_Indices(void)

int * __fastcall Housemarque::Threedee_Engine::Morph_Data::Get_Polygon_Indices(void)

{
  srVector3i *psVar1;
  int in_ECX;
  
                    // 0x18ac0  87
                    // ?Get_Polygon_Indices@Morph_Data@Threedee_Engine@Housemarque@@QAIPAHXZ
  if (*(int *)(in_ECX + 0x3c) == 0) {
    Housemarque::Kernel::Throw_Error
              ((char *)0x0,s_default_model_10037c70,s_E__work_ht_3de_3DE_Morph_cpp_10037c50,0x175);
  }
  psVar1 = srMeshModel::getPolyVertex(*(srMeshModel **)(in_ECX + 0x3c));
  return (int *)psVar1;
}



// public: class srShader __fastcall Housemarque::Threedee_Engine::Morph_Data::Get_Shader(void)

undefined4 * __fastcall
Housemarque::Threedee_Engine::Morph_Data::Get_Shader(int param_1,undefined4 *param_2)

{
  undefined4 *puVar1;
  int local_4;
  
                    // 0x18af0  92
                    // ?Get_Shader@Morph_Data@Threedee_Engine@Housemarque@@QAI?AVsrShader@@XZ
  local_4 = param_1;
  if (*(int *)(param_1 + 0x3c) == 0) {
    Housemarque::Kernel::Throw_Error
              ((char *)0x0,s_default_model_10037ca0,s_E__work_ht_3de_3DE_Morph_cpp_10037c80,0x17b);
  }
  puVar1 = (undefined4 *)srMeshModel::getShader(*(srMeshModel **)(param_1 + 0x3c),&local_4);
  *param_2 = *puVar1;
  return param_2;
}



// public: void __fastcall Housemarque::Threedee_Engine::Morph_Data::Get_Bounding_Sphere(class
// Housemarque::Template_Library::Vector_3<float> &,float &)

void __fastcall
Housemarque::Threedee_Engine::Morph_Data::Get_Bounding_Sphere
          (Vector_3<float> *param_1,float *param_2)

{
  undefined4 in_stack_00000004;
  
                    // 0x18b30  74
                    // ?Get_Bounding_Sphere@Morph_Data@Threedee_Engine@Housemarque@@QAIXAAV?$Vector_3@M@Template_Library@3@AAM@Z
  if (*(int *)(param_1 + 0x3c) == 0) {
    Housemarque::Kernel::Throw_Error
              ((char *)0x0,s_default_model_10037cd0,s_E__work_ht_3de_3DE_Morph_cpp_10037cb0,0x181);
  }
  (**(code **)(**(int **)(param_1 + 0x3c) + 0x28))(param_2,in_stack_00000004);
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Morph_Data::Get_Vertex_Locations(class
// Housemarque::Template_Library::Vector_3<float> *,unsigned int,char const *,float)

void __fastcall
Housemarque::Threedee_Engine::Morph_Data::Get_Vertex_Locations
          (Vector_3<float> *param_1,uint param_2,char *param_3,float param_4)

{
  HTSC_Node_Base *this;
  float in_stack_0000000c;
  HTSC_Node_String local_28 [28];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0x18b70  99
                    // ?Get_Vertex_Locations@Morph_Data@Threedee_Engine@Housemarque@@QAIXPAV?$Vector_3@M@Template_Library@3@IPBDM@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10023d08;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  Housemarque::Template_Library::HTSC_Node_String::HTSC_Node_String(local_28,(char *)param_4);
  uStack_4 = 0;
  this = Housemarque::Template_Library::HTSC_Table_Base::Find((HTSC_Node_Base *)(param_1 + 0x40));
  uStack_4 = 0xffffffff;
  Housemarque::Template_Library::HTSC_Node_String::~HTSC_Node_String(local_28);
  FUN_10019a30(this,param_2,(uint)param_3,in_stack_0000000c);
  ExceptionList = pvStack_c;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Morph_Data::Update_Mesh(class
// Housemarque::Threedee_Engine::Morphing_Mesh &,char const *,float)

void __fastcall
Housemarque::Threedee_Engine::Morph_Data::Update_Mesh
          (Morphing_Mesh *param_1,char *param_2,float param_3)

{
  int *piVar1;
  uint uVar2;
  HTSC_Node_Base *this;
  undefined4 uVar3;
  float in_stack_00000008;
  HTSC_Node_String local_28 [28];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0x18bf0  153
                    // ?Update_Mesh@Morph_Data@Threedee_Engine@Housemarque@@QAIXAAVMorphing_Mesh@23@PBDM@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10023d1b;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  Housemarque::Template_Library::HTSC_Node_String::HTSC_Node_String(local_28,(char *)param_3);
  uStack_4 = 0;
  this = Housemarque::Template_Library::HTSC_Table_Base::Find((HTSC_Node_Base *)(param_1 + 0x40));
  uStack_4 = 0xffffffff;
  Housemarque::Template_Library::HTSC_Node_String::~HTSC_Node_String(local_28);
  if (*(int *)param_2 == 0) {
    uVar3 = (**(code **)(**(int **)(param_1 + 0x3c) + 0x24))();
    *(undefined4 *)param_2 = uVar3;
    (**(code **)(*(int *)(*(int *)(param_2 + 4) + 0x138) + 4))(uVar3);
  }
  FUN_10019810(this,*(srMeshModel **)param_2,in_stack_00000008);
  piVar1 = *(int **)param_2;
  if ((*(byte *)(piVar1 + 0xe4) & 1) == 0) {
    uVar2 = piVar1[0xe4];
    piVar1[0xe4] = uVar2 | 1;
    piVar1[0xe4] = uVar2 | 9;
    (**(code **)(*piVar1 + 0x34))(0);
  }
  if ((*(byte *)(piVar1 + 0xe4) & 2) == 0) {
    uVar2 = piVar1[0xe4];
    piVar1[0xe4] = uVar2 | 2;
    piVar1[0xe4] = uVar2 | 10;
  }
  if ((*(byte *)(piVar1 + 0xe4) & 4) == 0) {
    uVar2 = piVar1[0xe4];
    piVar1[0xe4] = uVar2 | 4;
    piVar1[0xe4] = uVar2 | 0xc;
  }
  if ((*(byte *)(piVar1 + 0xe4) & 8) == 0) {
    uVar2 = piVar1[0xe4];
    piVar1[0xe4] = uVar2 | 8;
    piVar1[0xe4] = uVar2 | 8;
  }
  ExceptionList = pvStack_c;
  return;
}



Config_Base * __thiscall
FUN_10018d30(void *this,basic_istream<> *param_1,HTSC_Node_Base *param_2,float *param_3,
            float *param_4,float *param_5,char *param_6)

{
  basic_istream<> bVar1;
  bool bVar2;
  undefined4 *puVar3;
  basic_ostream<> *pbVar4;
  char *pcVar5;
  basic_stringstream<> *pbVar6;
  basic_string<> *pbVar7;
  HTSC_Node_Base *pHVar8;
  uint uVar9;
  uint uVar10;
  code *pcVar11;
  undefined4 *puVar12;
  Config_Base *pCVar13;
  char *pcVar14;
  char cVar15;
  int iVar16;
  basic_string<> bStack_1cd;
  srGFInterface *psStack_1cc;
  uint local_1c8;
  basic_string<> *pbStack_1c4;
  Config_Base *local_1c0;
  srGFInterface *psStack_1bc;
  basic_string<> abStack_1b8 [4];
  undefined4 *puStack_1b4;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  basic_string<> abStack_1a8 [4];
  char *pcStack_1a4;
  basic_string<> abStack_198 [4];
  char *pcStack_194;
  basic_string<> abStack_188 [4];
  char *pcStack_184;
  Config_Base aCStack_16c [12];
  basic_string<> abStack_160 [16];
  undefined4 uStack_150;
  Segment aSStack_14c [4];
  undefined auStack_148 [156];
  Segment aSStack_ac [4];
  undefined auStack_a8 [156];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10023e43;
  pvStack_c = ExceptionList;
  local_1c8 = 0;
  ExceptionList = &pvStack_c;
  local_1c0 = (Config_Base *)this;
  Housemarque::Template_Library::Config_Base::Config_Base((Config_Base *)this);
  uStack_4 = 0;
  abStack_188[0] = bStack_1cd;
  std::basic_string<>::_Tidy(abStack_188,false);
  uVar9 = 0xffffffff;
  pcVar5 = s_not_defined_10037ce0;
  do {
    if (uVar9 == 0) break;
    uVar9 = uVar9 - 1;
    cVar15 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar15 != '\0');
  uVar9 = ~uVar9 - 1;
  bVar2 = std::basic_string<>::_Grow(abStack_188,uVar9,true);
  if (bVar2) {
    pcVar5 = s_not_defined_10037ce0;
    for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
      *(undefined4 *)pcStack_184 = *(undefined4 *)pcVar5;
      pcVar5 = pcVar5 + 4;
      pcStack_184 = pcStack_184 + 4;
    }
    for (uVar10 = uVar9 & 3; uVar10 != 0; uVar10 = uVar10 - 1) {
      *pcStack_184 = *pcVar5;
      pcVar5 = pcVar5 + 1;
      pcStack_184 = pcStack_184 + 1;
    }
    std::basic_string<>::_Eos(abStack_188,uVar9);
  }
  uStack_4 = CONCAT31(uStack_4._1_3_,1);
  std::basic_string<>::_Tidy(abStack_1b8,false);
  uVar9 = 0xffffffff;
  pcVar5 = &DAT_10037cec;
  do {
    if (uVar9 == 0) break;
    uVar9 = uVar9 - 1;
    cVar15 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar15 != '\0');
  uVar9 = ~uVar9 - 1;
  bVar2 = std::basic_string<>::_Grow(abStack_1b8,uVar9,true);
  if (bVar2) {
    puVar3 = (undefined4 *)&DAT_10037cec;
    puVar12 = puStack_1b4;
    for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
      *puVar12 = *puVar3;
      puVar3 = puVar3 + 1;
      puVar12 = puVar12 + 1;
    }
    for (uVar10 = uVar9 & 3; uVar10 != 0; uVar10 = uVar10 - 1) {
      *(undefined *)puVar12 = *(undefined *)puVar3;
      puVar3 = (undefined4 *)((int)puVar3 + 1);
      puVar12 = (undefined4 *)((int)puVar12 + 1);
    }
    std::basic_string<>::_Eos(abStack_1b8,uVar9);
  }
  pbVar7 = (basic_string<> *)((int)this + 0xc);
  uStack_4._0_1_ = 2;
  *pbVar7 = abStack_188[0];
  pbStack_1c4 = pbVar7;
  std::basic_string<>::_Tidy(pbVar7,false);
  std::basic_string<>::assign(pbVar7,abStack_188,0,*(uint *)npos_exref);
  uStack_4._0_1_ = 3;
  psStack_1cc = (srGFInterface *)operator_new(0x20);
  uStack_4._0_1_ = 4;
  if (psStack_1cc != (srGFInterface *)0x0) {
    FUN_10014030(psStack_1cc,abStack_1b8,(Config_Base *)this,pbVar7);
  }
  uStack_4._0_1_ = 7;
  std::basic_string<>::_Tidy(abStack_1b8,true);
  uStack_4 = CONCAT31(uStack_4._1_3_,6);
  std::basic_string<>::_Tidy(abStack_188,true);
  abStack_1a8[0] = bStack_1cd;
  std::basic_string<>::_Tidy(abStack_1a8,false);
  uVar9 = 0xffffffff;
  pcVar5 = s_not_defined_10037cf4;
  do {
    if (uVar9 == 0) break;
    uVar9 = uVar9 - 1;
    cVar15 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar15 != '\0');
  uVar9 = ~uVar9 - 1;
  bVar2 = std::basic_string<>::_Grow(abStack_1a8,uVar9,true);
  if (bVar2) {
    pcVar5 = s_not_defined_10037cf4;
    for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
      *(undefined4 *)pcStack_1a4 = *(undefined4 *)pcVar5;
      pcVar5 = pcVar5 + 4;
      pcStack_1a4 = pcStack_1a4 + 4;
    }
    for (uVar10 = uVar9 & 3; uVar10 != 0; uVar10 = uVar10 - 1) {
      *pcStack_1a4 = *pcVar5;
      pcVar5 = pcVar5 + 1;
      pcStack_1a4 = pcStack_1a4 + 1;
    }
    std::basic_string<>::_Eos(abStack_1a8,uVar9);
  }
  uStack_4 = CONCAT31(uStack_4._1_3_,8);
  std::basic_string<>::_Tidy(abStack_198,false);
  uVar9 = 0xffffffff;
  pcVar5 = s_hx_name_10037d00;
  do {
    if (uVar9 == 0) break;
    uVar9 = uVar9 - 1;
    cVar15 = *pcVar5;
    pcVar5 = pcVar5 + 1;
  } while (cVar15 != '\0');
  uVar9 = ~uVar9 - 1;
  bVar2 = std::basic_string<>::_Grow(abStack_198,uVar9,true);
  if (bVar2) {
    pcVar5 = s_hx_name_10037d00;
    for (uVar10 = uVar9 >> 2; uVar10 != 0; uVar10 = uVar10 - 1) {
      *(undefined4 *)pcStack_194 = *(undefined4 *)pcVar5;
      pcVar5 = pcVar5 + 4;
      pcStack_194 = pcStack_194 + 4;
    }
    for (uVar10 = uVar9 & 3; uVar10 != 0; uVar10 = uVar10 - 1) {
      *pcStack_194 = *pcVar5;
      pcVar5 = pcVar5 + 1;
      pcStack_194 = pcStack_194 + 1;
    }
    std::basic_string<>::_Eos(abStack_198,uVar9);
  }
  pbVar7 = (basic_string<> *)((int)this + 0x1c);
  uStack_4._0_1_ = 9;
  *pbVar7 = abStack_1a8[0];
  psStack_1cc = (srGFInterface *)pbVar7;
  pbStack_1c4 = pbVar7;
  std::basic_string<>::_Tidy(pbVar7,false);
  std::basic_string<>::assign(pbVar7,abStack_1a8,0,*(uint *)npos_exref);
  uStack_4._0_1_ = 10;
  psStack_1bc = (srGFInterface *)operator_new(0x20);
  uStack_4._0_1_ = 0xb;
  if (psStack_1bc != (srGFInterface *)0x0) {
    FUN_10014030(psStack_1bc,abStack_198,(Config_Base *)this,pbVar7);
  }
  uStack_4._0_1_ = 0xe;
  std::basic_string<>::_Tidy(abStack_198,true);
  uStack_4._0_1_ = 0xd;
  std::basic_string<>::_Tidy(abStack_1a8,true);
  *(undefined4 *)((int)this + 0xfcc) = 0;
  Housemarque::Legacy::Segment::Segment(aSStack_ac,param_1,'{','}',';');
  uStack_4._0_1_ = 0xf;
  bVar2 = std::operator==((basic_string<> *)
                          (-(uint)(&stack0x00000000 != (undefined *)0xac) & (uint)auStack_a8),
                          s_settings_10037d08);
  if (!bVar2) {
    local_1c8 = 1;
    puVar3 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&psStack_1cc);
    uStack_4 = CONCAT31(uStack_4._1_3_,0x10);
    pbVar4 = std::operator<<((basic_ostream<> *)*puVar3,s_3de___Morph_Data__illegal_config_10037d34)
    ;
    std::operator<<(pbVar4,'\0');
    iVar16 = 0x1b0;
    pcVar14 = s_E__work_ht_3de_3DE_Morph_cpp_10037d14;
    pcVar5 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(s_name_str_____settings__10037d5c,pcVar5,pcVar14,iVar16);
  }
  uStack_4 = 0xf;
  if ((local_1c8 & 1) != 0) {
    local_1c8 = local_1c8 & 0xfffffffe;
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&psStack_1cc);
  }
  pbVar6 = Housemarque::Legacy::Segment::Stream();
  pCVar13 = local_1c0;
  Housemarque::Template_Library::operator>>((basic_istream<> *)pbVar6,local_1c0);
  psStack_1bc = (srGFInterface *)operator_new(0x28);
  uStack_4._0_1_ = 0x11;
  if (psStack_1bc == (srGFInterface *)0x0) {
    psStack_1cc = (srGFInterface *)0x0;
  }
  else {
    pcVar11 = *(code **)(pbStack_1c4 + 4);
    if (*(code **)(pbStack_1c4 + 4) == (code *)0x0) {
      pcVar11 = _C_exref;
    }
    psStack_1cc = (srGFInterface *)srGFInterface::srGFInterface(psStack_1bc,(char *)pcVar11);
  }
  uStack_4._0_1_ = 0xf;
  if (psStack_1cc == (srGFInterface *)0x0) {
    pcVar11 = *(code **)(pbStack_1c4 + 4);
    if (*(code **)(pbStack_1c4 + 4) == (code *)0x0) {
      pcVar11 = _C_exref;
    }
    local_1c8 = local_1c8 | 2;
    puVar3 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&pbStack_1c4);
    uStack_4 = CONCAT31(uStack_4._1_3_,0x12);
    pbVar4 = std::operator<<((basic_ostream<> *)*puVar3,s_3de__Cant_open_file__10037d94);
    cVar15 = '\0';
    pbVar4 = std::operator<<(pbVar4,(char *)pcVar11);
    std::operator<<(pbVar4,cVar15);
    iVar16 = 0x1b4;
    pcVar14 = s_E__work_ht_3de_3DE_Morph_cpp_10037d74;
    pcVar5 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(&DAT_10037dac,pcVar5,pcVar14,iVar16);
    pCVar13 = local_1c0;
  }
  uStack_4 = 0xf;
  if ((local_1c8 & 2) != 0) {
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&pbStack_1c4);
  }
  bVar1 = param_1[*(int *)(*(int *)param_1 + 4) + 4];
  while (((byte)bVar1 & 1) == 0) {
    Housemarque::Legacy::Segment::Segment(aSStack_14c,param_1,'{','}',';');
    uStack_4._0_1_ = 0x13;
    bVar2 = std::operator==((basic_string<> *)
                            (-(uint)(&stack0x00000000 != (undefined *)0x14c) & (uint)auStack_148),
                            s_frame_10037db0);
    if (bVar2) {
      pcVar5 = param_6;
      pbVar6 = Housemarque::Legacy::Segment::Stream();
      FUN_10019aa0(aCStack_16c,(basic_istream<> *)pbVar6,pcVar5);
      uStack_4._0_1_ = 0x14;
      pbVar7 = (basic_string<> *)std::operator+(abStack_1b8,pCVar13 + 0x1c,abStack_160);
      uStack_4._0_1_ = 0x15;
      Housemarque::Template_Library::HTSC_Node_String::HTSC_Node_String
                ((HTSC_Node_String *)abStack_188,pbVar7);
      uStack_4._0_1_ = 0x16;
      pHVar8 = Housemarque::Template_Library::HTSC_Table_Base::Find(param_2);
      uStack_4._0_1_ = 0x15;
      Housemarque::Template_Library::HTSC_Node_String::~HTSC_Node_String
                ((HTSC_Node_String *)abStack_188);
      uStack_4._0_1_ = 0x14;
      if (puStack_1b4 != (undefined4 *)0x0) {
        pcVar5 = (char *)((int)puStack_1b4 + -1);
        cVar15 = *pcVar5;
        if ((cVar15 == '\0') || (cVar15 == -1)) {
          FUN_100212a4(pcVar5);
        }
        else {
          *pcVar5 = cVar15 + -1;
        }
      }
      puStack_1b4 = (undefined4 *)0x0;
      uStack_1b0 = 0;
      uStack_1ac = 0;
      if (pHVar8 == (HTSC_Node_Base *)0x0) {
        psStack_1bc = (srGFInterface *)operator_new(0x34);
        uStack_4._0_1_ = 0x17;
        if (psStack_1bc == (srGFInterface *)0x0) {
          pHVar8 = (HTSC_Node_Base *)0x0;
        }
        else {
          pHVar8 = (HTSC_Node_Base *)
                   FUN_10019ea0(psStack_1bc,abStack_160,pCVar13 + 0x1c,(char *)psStack_1cc,param_3,
                                param_4,param_5);
        }
        uStack_4._0_1_ = 0x14;
        Housemarque::Template_Library::HTSC_Table_Base::Add(param_2);
      }
      *(HTSC_Node_Base **)(pCVar13 + *(int *)(pCVar13 + 0xfcc) * 4 + 0x2c) = pHVar8;
      *(undefined4 *)(pCVar13 + *(int *)(pCVar13 + 0xfcc) * 4 + 0x7fc) = uStack_150;
      *(int *)(pCVar13 + 0xfcc) = *(int *)(pCVar13 + 0xfcc) + 1;
      uStack_4._0_1_ = 0x13;
      FUN_10019d80(aCStack_16c);
    }
    uStack_4 = CONCAT31(uStack_4._1_3_,0xf);
    Housemarque::Legacy::Segment::~Segment(aSStack_14c);
    bVar1 = param_1[*(int *)(*(int *)param_1 + 4) + 4];
  }
  if (psStack_1cc != (srGFInterface *)0x0) {
    srGFInterface::~srGFInterface(psStack_1cc);
    FUN_100212a4(psStack_1cc);
  }
  uStack_4 = CONCAT31(uStack_4._1_3_,0xd);
  Housemarque::Legacy::Segment::~Segment(aSStack_ac);
  ExceptionList = pvStack_c;
  return pCVar13;
}



void __fastcall FUN_10019400(Config_Base *param_1)

{
  char *pcVar1;
  char cVar2;
  
  if (*(int *)(param_1 + 0x20) != 0) {
    pcVar1 = (char *)(*(int *)(param_1 + 0x20) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(param_1 + 0x20) = 0;
  *(undefined4 *)(param_1 + 0x24) = 0;
  *(undefined4 *)(param_1 + 0x28) = 0;
  if (*(int *)(param_1 + 0x10) != 0) {
    pcVar1 = (char *)(*(int *)(param_1 + 0x10) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 != '\0') && (cVar2 != -1)) {
      *pcVar1 = cVar2 + -1;
      *(undefined4 *)(param_1 + 0x10) = 0;
      *(undefined4 *)(param_1 + 0x14) = 0;
      *(undefined4 *)(param_1 + 0x18) = 0;
      Housemarque::Template_Library::Config_Base::~Config_Base(param_1);
      return;
    }
    FUN_100212a4(pcVar1);
  }
  *(undefined4 *)(param_1 + 0x10) = 0;
  *(undefined4 *)(param_1 + 0x14) = 0;
  *(undefined4 *)(param_1 + 0x18) = 0;
  Housemarque::Template_Library::Config_Base::~Config_Base(param_1);
  return;
}



void * __thiscall FUN_10019480(void *this,basic_istream<> *param_1,char *param_2)

{
  int *_FileHandle;
  char cVar1;
  bool bVar2;
  void *pvVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  char *pcVar7;
  uint unaff_EDI;
  char *pcVar8;
  basic_string<> abStack_128 [4];
  char *pcStack_124;
  uint uStack_120;
  undefined4 uStack_11c;
  uint uStack_118;
  void *pvStack_114;
  void *local_110;
  char acStack_10c [256];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10023e85;
  pvStack_c = ExceptionList;
  bVar2 = false;
  ExceptionList = &pvStack_c;
  local_110 = this;
  Housemarque::Template_Library::HTSC_Node_String::HTSC_Node_String
            ((HTSC_Node_String *)this,param_2);
  _FileHandle = (int *)((int)this + 0x1c);
  uStack_4 = 0;
  *(undefined ***)this = &PTR_FUN_10025e98;
  std::basic_istream<>::read((int)_FileHandle,(void *)0x4,unaff_EDI);
  pvVar3 = operator_new(*_FileHandle << 2);
  *(void **)((int)this + 0x20) = pvVar3;
  pvVar3 = operator_new(*_FileHandle << 2);
  *(void **)((int)this + 0x24) = pvVar3;
  pvVar3 = operator_new(*_FileHandle << 2);
  *(void **)((int)this + 0x28) = pvVar3;
  uStack_118 = 0;
  if (*_FileHandle != 0) {
    do {
      pcVar7 = acStack_10c;
      for (iVar4 = 0x40; iVar4 != 0; iVar4 = iVar4 + -1) {
        pcVar7[0] = '\0';
        pcVar7[1] = '\0';
        pcVar7[2] = '\0';
        pcVar7[3] = '\0';
        pcVar7 = pcVar7 + 4;
      }
      std::basic_istream<>::getline(param_1,acStack_10c,0x100,'\0');
      pvVar3 = operator_new(0x34);
      uStack_4._0_1_ = 1;
      pvStack_114 = pvVar3;
      if (pvVar3 == (void *)0x0) {
        pvVar3 = (void *)0x0;
      }
      else {
        uVar5 = 0xffffffff;
        pcVar7 = acStack_10c;
        do {
          if (uVar5 == 0) break;
          uVar5 = uVar5 - 1;
          cVar1 = *pcVar7;
          pcVar7 = pcVar7 + 1;
        } while (cVar1 != '\0');
        uVar5 = ~uVar5 - 1;
        pcStack_124 = (char *)0x0;
        uStack_120 = 0;
        uStack_11c = 0;
        bVar2 = std::basic_string<>::_Grow(abStack_128,uVar5,true);
        if (bVar2) {
          pcVar7 = acStack_10c;
          pcVar8 = pcStack_124;
          for (uVar6 = uVar5 >> 2; uVar6 != 0; uVar6 = uVar6 - 1) {
            *(undefined4 *)pcVar8 = *(undefined4 *)pcVar7;
            pcVar7 = pcVar7 + 4;
            pcVar8 = pcVar8 + 4;
          }
          for (uVar6 = uVar5 & 3; uVar6 != 0; uVar6 = uVar6 - 1) {
            *pcVar8 = *pcVar7;
            pcVar7 = pcVar7 + 1;
            pcVar8 = pcVar8 + 1;
          }
          pcStack_124[uVar5] = '\0';
          pvVar3 = pvStack_114;
          uStack_120 = uVar5;
        }
        bVar2 = true;
        uStack_4 = CONCAT31(uStack_4._1_3_,2);
        pvVar3 = FUN_10019dd0(pvVar3,param_1,abStack_128);
      }
      uVar5 = uStack_118;
      *(void **)(*(int *)((int)this + 0x20) + uStack_118 * 4) = pvVar3;
      uStack_4 = 0;
      if (bVar2) {
        bVar2 = false;
        if (pcStack_124 != (char *)0x0) {
          cVar1 = pcStack_124[-1];
          if ((cVar1 == '\0') || (cVar1 == -1)) {
            FUN_100212a4(pcStack_124 + -1);
          }
          else {
            pcStack_124[-1] = cVar1 + -1;
          }
        }
        pcStack_124 = (char *)0x0;
        uStack_120 = 0;
        uStack_11c = 0;
      }
      uStack_118 = uVar5 + 1;
    } while (uStack_118 < *(uint *)((int)this + 0x1c));
  }
  std::basic_istream<>::read
            (*(int *)((int)this + 0x24),(void *)(*(int *)((int)this + 0x1c) << 2),unaff_EDI);
  std::basic_istream<>::read
            (*(int *)((int)this + 0x28),(void *)(*(int *)((int)this + 0x1c) << 2),unaff_EDI);
  ExceptionList = pvStack_c;
  return this;
}



void * __thiscall FUN_10019690(void *this,int param_1)

{
  int iVar1;
  float fVar2;
  code *pcVar3;
  void *pvVar4;
  uint uVar5;
  float *pfVar6;
  undefined4 *puVar7;
  int iVar8;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10023e98;
  pvStack_c = ExceptionList;
  pcVar3 = *(code **)(param_1 + 0x10);
  if (*(code **)(param_1 + 0x10) == (code *)0x0) {
    pcVar3 = _C_exref;
  }
  ExceptionList = &pvStack_c;
  Housemarque::Template_Library::HTSC_Node_String::HTSC_Node_String
            ((HTSC_Node_String *)this,(char *)pcVar3);
  *(undefined ***)this = &PTR_FUN_10025e98;
  iVar8 = *(int *)(param_1 + 0xfcc);
  *(int *)((int)this + 0x1c) = iVar8;
  uStack_4 = 0;
  pvVar4 = operator_new(iVar8 << 2);
  *(void **)((int)this + 0x20) = pvVar4;
  pvVar4 = operator_new(*(int *)((int)this + 0x1c) << 2);
  *(void **)((int)this + 0x24) = pvVar4;
  pvVar4 = operator_new(*(int *)((int)this + 0x1c) << 2);
  *(void **)((int)this + 0x28) = pvVar4;
  uVar5 = 0;
  if (*(int *)((int)this + 0x1c) != 0) {
    puVar7 = (undefined4 *)(param_1 + 0x7fc);
    do {
      *(undefined4 *)(*(int *)((int)this + 0x20) + uVar5 * 4) = puVar7[-500];
      *(undefined4 *)(*(int *)((int)this + 0x24) + uVar5 * 4) = *puVar7;
      uVar5 = uVar5 + 1;
      puVar7 = puVar7 + 1;
    } while (uVar5 < *(uint *)((int)this + 0x1c));
  }
  fVar2 = 0.0;
  iVar8 = *(int *)((int)this + 0x1c) + -1;
  if (iVar8 != 0) {
    pfVar6 = *(float **)((int)this + 0x24);
    do {
      fVar2 = fVar2 + *pfVar6;
      pfVar6 = pfVar6 + 1;
      iVar8 = iVar8 + -1;
    } while (iVar8 != 0);
  }
  uVar5 = 0;
  if (*(int *)((int)this + 0x1c) != 0) {
    do {
      *(undefined4 *)(*(int *)((int)this + 0x28) + uVar5 * 4) = 0;
      uVar5 = uVar5 + 1;
    } while (uVar5 < *(uint *)((int)this + 0x1c));
  }
  uVar5 = 1;
  if (1 < *(uint *)((int)this + 0x1c)) {
    do {
      pfVar6 = (float *)(*(int *)((int)this + 0x28) + uVar5 * 4);
      iVar8 = uVar5 * 4;
      uVar5 = uVar5 + 1;
      *pfVar6 = *(float *)(*(int *)((int)this + 0x24) + -4 + iVar8) + pfVar6[-1];
    } while (uVar5 < *(uint *)((int)this + 0x1c));
  }
  uVar5 = 0;
  if (*(int *)((int)this + 0x1c) != 0) {
    do {
      iVar8 = uVar5 * 4;
      iVar1 = uVar5 * 4;
      uVar5 = uVar5 + 1;
      *(float *)(*(int *)((int)this + 0x28) + iVar1) =
           *(float *)(*(int *)((int)this + 0x28) + iVar8) / fVar2;
    } while (uVar5 < *(uint *)((int)this + 0x1c));
  }
  ExceptionList = pvStack_c;
  return this;
}



void __fastcall FUN_100197d0(HTSC_Node_String *param_1)

{
  *(undefined ***)param_1 = &PTR_FUN_10025e98;
  FUN_100212a4(*(void **)(param_1 + 0x28));
  FUN_100212a4(*(void **)(param_1 + 0x20));
  FUN_100212a4(*(void **)(param_1 + 0x24));
  Housemarque::Template_Library::HTSC_Node_String::~HTSC_Node_String(param_1);
  return;
}



void __thiscall FUN_10019810(void *this,srMeshModel *param_1,float param_2)

{
  int iVar1;
  srVector3T<float> *psVar2;
  uint uVar3;
  float *pfVar4;
  
  psVar2 = srMeshModel::getVertexLoc(param_1);
  uVar3 = 0;
  if (*(int *)((int)this + 0x1c) != 2) {
    pfVar4 = *(float **)((int)this + 0x28);
    do {
      pfVar4 = pfVar4 + 1;
      if (param_2 - *pfVar4 < 0.0) break;
      uVar3 = uVar3 + 1;
    } while (uVar3 < *(int *)((int)this + 0x1c) - 2U);
  }
  iVar1 = *(int *)((int)this + 0x28);
  FUN_100198a0((int)psVar2,*(int *)(*(int *)(*(int *)((int)this + 0x20) + uVar3 * 4) + 0x20),
               *(int *)(*(int *)(*(int *)((int)this + 0x20) + 4 + uVar3 * 4) + 0x20),
               (param_2 - *(float *)(iVar1 + uVar3 * 4)) /
               (*(float *)(iVar1 + 4 + uVar3 * 4) - *(float *)(iVar1 + uVar3 * 4)),
               *(uint *)(param_1 + 0x22c));
  return;
}



void __cdecl FUN_100198a0(int param_1,int param_2,int param_3,float param_4,uint param_5)

{
  float fVar1;
  uint uVar2;
  int iVar3;
  uint uVar4;
  float local_79c;
  float local_798;
  float local_794;
  int local_790;
  float local_78c;
  float local_788;
  float local_784;
  undefined local_780 [1920];
  
  uVar2 = 0;
  if (param_5 != 0) {
    fVar1 = 1.0 - param_4;
    local_790 = param_2 - param_3;
    iVar3 = param_3 - param_1;
    do {
      uVar4 = param_5 - uVar2;
      if (0xa0 < uVar4) {
        uVar4 = 0xa0;
      }
      local_79c = fVar1;
      local_798 = fVar1;
      local_794 = fVar1;
      if (uVar4 != 0) {
        if (((fVar1 == 0.0) && (fVar1 == 0.0)) && (fVar1 == 0.0)) {
          if (uVar4 * 3 != 0) {
            (**(code **)(**(int **)vp_exref + 0x38))(local_780,0,uVar4 * 3);
          }
        }
        else {
          (**(code **)(**(int **)vp_exref + 0x124))
                    (local_780,&local_79c,local_790 + iVar3 + param_1,uVar4);
        }
      }
      local_78c = param_4;
      local_788 = param_4;
      local_784 = param_4;
      if (uVar4 != 0) {
        if (param_4 == 0.0) {
          if (uVar4 * 3 != 0) {
            (**(code **)(**(int **)vp_exref + 0x38))(param_1,0,uVar4 * 3);
          }
        }
        else {
          (**(code **)(**(int **)vp_exref + 0x124))(param_1,&local_78c,param_1 + iVar3,uVar4);
        }
      }
      if (uVar4 * 3 != 0) {
        (**(code **)(**(int **)vp_exref + 0xd8))(param_1,param_1,local_780,uVar4 * 3);
      }
      uVar2 = uVar2 + 0xa0;
      param_1 = param_1 + 0x780;
    } while (uVar2 < param_5);
  }
  return;
}



void __thiscall FUN_10019a30(void *this,int param_1,uint param_2,float param_3)

{
  int iVar1;
  uint uVar2;
  float *pfVar3;
  
  uVar2 = 0;
  if (*(int *)((int)this + 0x1c) != 2) {
    pfVar3 = *(float **)((int)this + 0x28);
    do {
      pfVar3 = pfVar3 + 1;
      if (param_3 - *pfVar3 < 0.0) break;
      uVar2 = uVar2 + 1;
    } while (uVar2 < *(int *)((int)this + 0x1c) - 2U);
  }
  iVar1 = *(int *)((int)this + 0x28);
  FUN_100198a0(param_1,*(int *)(*(int *)(*(int *)((int)this + 0x20) + uVar2 * 4) + 0x20),
               *(int *)(*(int *)(*(int *)((int)this + 0x20) + 4 + uVar2 * 4) + 0x20),
               (param_3 - *(float *)(iVar1 + uVar2 * 4)) /
               (*(float *)(iVar1 + 4 + uVar2 * 4) - *(float *)(iVar1 + uVar2 * 4)),param_2);
  return;
}



void * __thiscall FUN_10019aa0(void *this,basic_istream<> *param_1,char *param_2)

{
  basic_string<> *this_00;
  char cVar1;
  basic_istream<> bVar2;
  bool bVar3;
  basic_stringstream<> *pbVar4;
  uint uVar5;
  char *pcVar6;
  basic_string<> *pbStack_f8;
  basic_string<> abStack_f4 [16];
  basic_string<> abStack_e4 [16];
  basic_string<> abStack_d4 [16];
  basic_string<> abStack_c4 [4];
  int iStack_c0;
  void *local_b4;
  void *pvStack_b0;
  Segment aSStack_ac [4];
  undefined auStack_a8 [156];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10023f11;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_b4 = this;
  Housemarque::Template_Library::Config_Base::Config_Base((Config_Base *)this);
  uStack_4 = 0;
  std::basic_string<>::_Tidy(abStack_e4,false);
  uVar5 = 0xffffffff;
  pcVar6 = s_not_defined_10037db8;
  do {
    if (uVar5 == 0) break;
    uVar5 = uVar5 - 1;
    cVar1 = *pcVar6;
    pcVar6 = pcVar6 + 1;
  } while (cVar1 != '\0');
  std::basic_string<>::assign(abStack_e4,s_not_defined_10037db8,~uVar5 - 1);
  uStack_4._0_1_ = 1;
  std::basic_string<>::_Tidy(abStack_f4,false);
  uVar5 = 0xffffffff;
  pcVar6 = s_mesh_name_10037dc4;
  do {
    if (uVar5 == 0) break;
    uVar5 = uVar5 - 1;
    cVar1 = *pcVar6;
    pcVar6 = pcVar6 + 1;
  } while (cVar1 != '\0');
  std::basic_string<>::assign(abStack_f4,s_mesh_name_10037dc4,~uVar5 - 1);
  this_00 = (basic_string<> *)((int)this + 0xc);
  uStack_4._0_1_ = 2;
  pbStack_f8 = this_00;
  std::basic_string<>::basic_string<>(this_00,abStack_e4);
  uStack_4._0_1_ = 3;
  pvStack_b0 = operator_new(0x20);
  uStack_4._0_1_ = 4;
  if (pvStack_b0 != (void *)0x0) {
    FUN_10014030(pvStack_b0,abStack_f4,(Config_Base *)this,this_00);
  }
  uStack_4._0_1_ = 7;
  std::basic_string<>::_Tidy(abStack_f4,true);
  uStack_4._0_1_ = 6;
  std::basic_string<>::_Tidy(abStack_e4,true);
  std::basic_string<>::_Tidy(abStack_d4,false);
  uVar5 = 0xffffffff;
  pcVar6 = s_weight_10037dd0;
  do {
    if (uVar5 == 0) break;
    uVar5 = uVar5 - 1;
    cVar1 = *pcVar6;
    pcVar6 = pcVar6 + 1;
  } while (cVar1 != '\0');
  std::basic_string<>::assign(abStack_d4,s_weight_10037dd0,~uVar5 - 1);
  uStack_4._0_1_ = 8;
  pbStack_f8 = (basic_string<> *)0x3f800000;
  FUN_100145a0((void *)((int)this + 0x1c),(Config_Base *)this,abStack_d4,&pbStack_f8);
  uStack_4._0_1_ = 6;
  std::basic_string<>::_Tidy(abStack_d4,true);
  std::basic_string<>::_Tidy(abStack_c4,false);
  bVar2 = param_1[*(int *)(*(int *)param_1 + 4) + 4];
  uStack_4._0_1_ = 9;
  while (((byte)bVar2 & 1) == 0) {
    Housemarque::Legacy::Segment::Segment(aSStack_ac,param_1,'{','}',';');
    uStack_4 = CONCAT31(uStack_4._1_3_,10);
    bVar3 = std::operator==((basic_string<> *)
                            (-(uint)(&stack0x00000000 != (undefined *)0xac) & (uint)auStack_a8),
                            param_2);
    if (bVar3) {
      pbVar4 = Housemarque::Legacy::Segment::Stream();
      Housemarque::Template_Library::operator>>((basic_istream<> *)pbVar4,(Config_Base *)this);
      std::basic_string<>::assign
                (abStack_c4,(basic_string<> *)((int)this + 0xc),0,*(uint *)npos_exref);
    }
    bVar3 = std::operator==((basic_string<> *)
                            (-(uint)(&stack0x00000000 != (undefined *)0xac) & (uint)auStack_a8),
                            s_general_10037dd8);
    if (bVar3) {
      pbVar4 = Housemarque::Legacy::Segment::Stream();
      Housemarque::Template_Library::operator>>((basic_istream<> *)pbVar4,(Config_Base *)this);
      pbStack_f8 = *(basic_string<> **)((int)this + 0x1c);
    }
    uStack_4._0_1_ = 9;
    Housemarque::Legacy::Segment::~Segment(aSStack_ac);
    bVar2 = param_1[*(int *)(*(int *)param_1 + 4) + 4];
  }
  *(basic_string<> **)((int)this + 0x1c) = pbStack_f8;
  std::basic_string<>::assign((basic_string<> *)((int)this + 0xc),abStack_c4,0,*(uint *)npos_exref);
  if (iStack_c0 != 0) {
    cVar1 = *(char *)(iStack_c0 + -1);
    if ((cVar1 == '\0') || (cVar1 == -1)) {
      FUN_100212a4((void *)(iStack_c0 + -1));
    }
    else {
      *(char *)(iStack_c0 + -1) = cVar1 + -1;
    }
  }
  ExceptionList = pvStack_c;
  return this;
}



void __fastcall FUN_10019d80(Config_Base *param_1)

{
  char *pcVar1;
  char cVar2;
  
  if (*(int *)(param_1 + 0x10) != 0) {
    pcVar1 = (char *)(*(int *)(param_1 + 0x10) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(param_1 + 0x10) = 0;
  *(undefined4 *)(param_1 + 0x14) = 0;
  *(undefined4 *)(param_1 + 0x18) = 0;
  Housemarque::Template_Library::Config_Base::~Config_Base(param_1);
  return;
}



void * __thiscall FUN_10019dd0(void *this,undefined4 param_1,basic_string<> *param_2)

{
  int *_FileHandle;
  code *pcVar1;
  void *_FileHandle_00;
  uint unaff_EDI;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10023f30;
  pvStack_c = ExceptionList;
  pcVar1 = *(code **)(param_2 + 4);
  if (*(code **)(param_2 + 4) == (code *)0x0) {
    pcVar1 = _C_exref;
  }
  ExceptionList = &pvStack_c;
  Housemarque::Template_Library::HTSC_Node_String::HTSC_Node_String
            ((HTSC_Node_String *)this,(char *)pcVar1);
  *(basic_string<> *)((int)this + 0x24) = *param_2;
  *(undefined4 *)((int)this + 0x28) = 0;
  *(undefined4 *)((int)this + 0x2c) = 0;
  *(undefined4 *)((int)this + 0x30) = 0;
  uStack_4 = 0;
  std::basic_string<>::assign((basic_string<> *)((int)this + 0x24),param_2,0,*(uint *)npos_exref);
  _FileHandle = (int *)((int)this + 0x1c);
  uStack_4 = CONCAT31(uStack_4._1_3_,1);
  *(undefined ***)this = &PTR_FUN_10025ea8;
  std::basic_istream<>::read((int)_FileHandle,(void *)0x4,unaff_EDI);
  _FileHandle_00 = srHeap::allocate((srHeap *)srHeap_exref,*_FileHandle * 0xc);
  if (_FileHandle_00 == (void *)0x0) {
    _FileHandle_00 = (void *)0x0;
  }
  *(void **)((int)this + 0x20) = _FileHandle_00;
  std::basic_istream<>::read((int)_FileHandle_00,(void *)(*_FileHandle * 0xc),unaff_EDI);
  ExceptionList = pvStack_c;
  return this;
}



void * __thiscall
FUN_10019ea0(void *this,basic_string<> *param_1,float *param_2,char *param_3,float *param_4,
            float *param_5,float *param_6)

{
  srVector3T<float> *psVar1;
  int *piVar2;
  int *piVar3;
  int *piVar4;
  bool bVar5;
  basic_string<> *pbVar6;
  list<> *plVar7;
  srMeshModel *this_00;
  void *pvVar8;
  srVector3T<float> *psVar9;
  int iVar10;
  uint uVar11;
  undefined4 *puVar12;
  reOrderData arStack_28 [4];
  int *piStack_24;
  int iStack_20;
  basic_string<> local_1c [16];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10023f60;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  pbVar6 = (basic_string<> *)std::operator+(local_1c,param_2,param_1);
  local_4 = 0;
  Housemarque::Template_Library::HTSC_Node_String::HTSC_Node_String((HTSC_Node_String *)this,pbVar6)
  ;
  local_4._0_1_ = 2;
  std::basic_string<>::_Tidy(local_1c,true);
  std::operator+((int)this + 0x24,param_2,param_1);
  local_4._0_1_ = 3;
  *(undefined ***)this = &PTR_FUN_10025ea8;
  arStack_28[0] = param_1._0_1_;
  piStack_24 = (int *)operator_new(0xc);
  *piStack_24 = (int)piStack_24;
  piStack_24[1] = (int)piStack_24;
  iStack_20 = 0;
  local_4 = CONCAT31(local_4._1_3_,4);
  bVar5 = std::operator!=(param_1,s_not_defined_10037de0);
  if (!bVar5) {
    Housemarque::Kernel::Throw_Error
              ((char *)0x0,s_name_____not_defined__10037e0c,s_E__work_ht_3de_3DE_Morph_cpp_10037dec,
               0x27b);
  }
  plVar7 = *(list<> **)(param_1 + 4);
  if (*(list<> **)(param_1 + 4) == (list<> *)0x0) {
    plVar7 = (list<> *)_C_exref;
  }
  this_00 = FUN_1001ae40(param_3,arStack_28,plVar7,param_4,param_5,param_6,0);
  iVar10 = *(int *)(this_00 + 0x22c);
  *(int *)((int)this + 0x1c) = iVar10;
  pvVar8 = srHeap::allocate((srHeap *)srHeap_exref,iVar10 * 0xc);
  if (pvVar8 == (void *)0x0) {
    pvVar8 = (void *)0x0;
  }
  *(void **)((int)this + 0x20) = pvVar8;
  psVar9 = srMeshModel::getVertexLoc(this_00);
  uVar11 = 0;
  if (*(int *)((int)this + 0x1c) != 0) {
    iVar10 = 0;
    do {
      psVar1 = psVar9 + iVar10;
      puVar12 = (undefined4 *)(*(int *)((int)this + 0x20) + iVar10);
      *puVar12 = *(undefined4 *)psVar1;
      puVar12[1] = *(undefined4 *)(psVar1 + 4);
      puVar12[2] = *(undefined4 *)(psVar1 + 8);
      uVar11 = uVar11 + 1;
      iVar10 = iVar10 + 0xc;
    } while (uVar11 < *(uint *)((int)this + 0x1c));
  }
  srClass::release((srClass *)this_00);
  piVar4 = piStack_24;
  piVar3 = (int *)*piStack_24;
  while (piVar3 != piVar4) {
    piVar2 = (int *)*piVar3;
    *(int *)piVar3[1] = *piVar3;
    *(int *)(*piVar3 + 4) = piVar3[1];
    FUN_100212a4(piVar3);
    iStack_20 = iStack_20 + -1;
    piVar3 = piVar2;
  }
  FUN_100212a4(piStack_24);
  ExceptionList = pvStack_c;
  return this;
}



void __fastcall FUN_1001a060(HTSC_Node_String *param_1)

{
  char *pcVar1;
  char cVar2;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_10023f7f;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined ***)param_1 = &PTR_FUN_10025ea8;
  local_4 = 1;
  srHeap::free(*(void **)(param_1 + 0x20));
  if (*(int *)(param_1 + 0x28) != 0) {
    pcVar1 = (char *)(*(int *)(param_1 + 0x28) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(param_1 + 0x28) = 0;
  *(undefined4 *)(param_1 + 0x2c) = 0;
  *(undefined4 *)(param_1 + 0x30) = 0;
  local_4 = 0xffffffff;
  Housemarque::Template_Library::HTSC_Node_String::~HTSC_Node_String(param_1);
  ExceptionList = pvStack_c;
  return;
}



void __thiscall FUN_1001a100(void *this,undefined4 param_1)

{
  uint unaff_EDI;
  
  std::basic_ostream<>::write((int)this + 0x1c,(void *)0x4,unaff_EDI);
  std::basic_ostream<>::write
            (*(int *)((int)this + 0x20),(void *)(*(int *)((int)this + 0x1c) * 0xc),unaff_EDI);
  return;
}



void * __thiscall FUN_1001a140(void *this,byte param_1)

{
  srModelInstance::~srModelInstance((srModelInstance *)this);
  if ((param_1 & 1) != 0) {
    srHeap::free(this);
  }
  return this;
}



void __thiscall
Housemarque::Template_Library::HTSC_Table_Base::~HTSC_Table_Base(HTSC_Table_Base *this)

{
                    // WARNING: Could not recover jumptable at 0x1001a170. Too many branches
                    // WARNING: Treating indirect jump as call
  ~HTSC_Table_Base(this);
  return;
}



void __thiscall
Housemarque::Template_Library::HTSC_Table_Base::~HTSC_Table_Base(HTSC_Table_Base *this)

{
                    // WARNING: Could not recover jumptable at 0x1001a180. Too many branches
                    // WARNING: Treating indirect jump as call
  ~HTSC_Table_Base(this);
  return;
}



void * __thiscall FUN_1001a190(void *this,byte param_1)

{
  FUN_100197d0((HTSC_Node_String *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void * __thiscall FUN_1001a1b0(void *this,byte param_1)

{
  FUN_1001a060((HTSC_Node_String *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



srModelInstance * __fastcall FUN_1001a260(srModelInstance *param_1)

{
  srModelInstance *this;
  
  this = (srModelInstance *)(**(code **)(*(int *)param_1 + 0x20))();
  srModelInstance::operator=(this,param_1);
  return this;
}



void FUN_1001a2a0(void)

{
                    // WARNING: Could not recover jumptable at 0x1001a2a5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::Init((Init *)&DAT_10038509);
  return;
}



void FUN_1001a2b0(void)

{
  FUN_100214c2(FUN_1001a2c0);
  return;
}



void FUN_1001a2c0(void)

{
                    // WARNING: Could not recover jumptable at 0x1001a2c5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::~Init((Init *)&DAT_10038509);
  return;
}



void FUN_1001a2e0(void)

{
                    // WARNING: Could not recover jumptable at 0x1001a2e5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::_Winit((_Winit *)&DAT_10038508);
  return;
}



void FUN_1001a2f0(void)

{
  FUN_100214c2(FUN_1001a300);
  return;
}



void FUN_1001a300(void)

{
                    // WARNING: Could not recover jumptable at 0x1001a305. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::~_Winit((_Winit *)&DAT_10038508);
  return;
}



// public: __thiscall Housemarque::Threedee_Engine::Skin_Mesh::Skin::Skin(void)

Skin * __thiscall Housemarque::Threedee_Engine::Skin_Mesh::Skin::Skin(Skin *this)

{
                    // 0x1a310  20  ??0Skin@Skin_Mesh@Threedee_Engine@Housemarque@@QAE@XZ
  Housemarque::Bone_Import::Skin::Skin((Skin *)this);
  this[0x18] = (Skin)0x0;
  *(undefined4 *)(this + 0x20) = 0;
  *(undefined4 *)(this + 0x28) = 0;
  *(undefined4 *)(this + 0x30) = 0;
  *(undefined4 *)(this + 0x34) = 0;
  *(undefined4 *)(this + 0x3c) = 0;
  *(undefined ***)this = &PTR_FUN_10025ecc;
  return this;
}



// public: virtual __thiscall Housemarque::Threedee_Engine::Skin_Mesh::Skin::~Skin(void)

void __thiscall Housemarque::Threedee_Engine::Skin_Mesh::Skin::~Skin(Skin *this)

{
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0x1a340  40  ??1Skin@Skin_Mesh@Threedee_Engine@Housemarque@@UAE@XZ
  puStack_8 = &LAB_10023f99;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined ***)this = &PTR_FUN_10025ecc;
  local_4 = 0;
  srHeap::free(*(void **)(this + 0x3c));
  FUN_100212a4(*(void **)(this + 0x30));
  FUN_100212a4(*(void **)(this + 0x34));
  if (*(srClass **)(this + 0x28) != (srClass *)0x0) {
    srClass::release(*(srClass **)(this + 0x28));
  }
  local_4 = 0xffffffff;
  Housemarque::Bone_Import::Skin::~Skin((Skin *)this);
  ExceptionList = pvStack_c;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Skin_Mesh::Skin::Import_Skin(float,class
// Housemarque::Bone_Import::Hierarchy const &,class Housemarque::Bone_Import::Skin_File &,char
// const *,class srMeshModel *,class reOrderData const &,float,bool)

void __fastcall
Housemarque::Threedee_Engine::Skin_Mesh::Skin::Import_Skin
          (float param_1,Hierarchy *param_2,Skin_File *param_3,char *param_4,srMeshModel *param_5,
          reOrderData *param_6,float param_7,bool param_8)

{
  float *pfVar1;
  float fVar2;
  int iVar3;
  undefined4 *puVar4;
  basic_ostream<> *pbVar5;
  char *pcVar6;
  void *pvVar7;
  uint uVar8;
  uint uVar9;
  float *pfVar10;
  uint *puVar11;
  bool bVar12;
  undefined3 in_stack_00000019;
  char *pcVar13;
  int iVar14;
  void *local_10;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0x1a3c0  105
                    // ?Import_Skin@Skin@Skin_Mesh@Threedee_Engine@Housemarque@@QAIXMABVHierarchy@Bone_Import@4@AAVSkin_File@64@PBDPAVsrMeshModel@@ABVreOrderData@@M_N@Z
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10023fbb;
  pvStack_c = ExceptionList;
  bVar12 = *(int *)((int)param_1 + 0x20) != 0;
  ExceptionList = &pvStack_c;
  if (bVar12) {
    ExceptionList = &pvStack_c;
    puVar4 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&local_10);
    local_4 = 0;
    pbVar5 = std::operator<<((basic_ostream<> *)*puVar4,s_Skin_data_already_imported_10038018);
    std::operator<<(pbVar5,'\0');
    iVar14 = 0x2f;
    pcVar13 = s_E__work_ht_3de_3de_skin_cpp_10037ffc;
    pcVar6 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(s__hier_10038034,pcVar6,pcVar13,iVar14);
  }
  local_4 = 0xffffffff;
  if (bVar12) {
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&local_10);
  }
  *(Skin_File **)((int)param_1 + 0x1c) = param_3;
  *(Hierarchy **)((int)param_1 + 0x20) = param_2;
  *(undefined4 *)((int)param_1 + 0x40) = _param_8;
  *(undefined *)((int)param_1 + 0x18) = 0;
  *(reOrderData **)((int)param_1 + 0x28) = param_6;
  Housemarque::Bone_Import::Skin::Import((Skin_File *)param_1,param_4,(Hierarchy *)param_5);
  if (*(int *)(param_4 + 4) != *(int *)((int)param_1 + 4)) {
    Housemarque::Kernel::Throw_Error
              ((char *)0x0,s_file_Bone_Count______bone_count_10038058,
               s_E__work_ht_3de_3de_skin_cpp_1003803c,0x3d);
  }
  if (*(int *)(param_6 + 0x14) == 0) {
    iVar14 = 0;
  }
  else {
    iVar14 = *(int *)(param_6 + 0x18) - *(int *)(param_6 + 0x14) >> 2;
  }
  *(int *)((int)param_1 + 0x2c) = iVar14;
  pvVar7 = operator_new(iVar14 << 2);
  *(void **)((int)param_1 + 0x30) = pvVar7;
  pvVar7 = operator_new(*(int *)((int)param_1 + 0x2c) << 2);
  *(void **)((int)param_1 + 0x34) = pvVar7;
  uVar8 = 0;
  if (*(int *)((int)param_1 + 0x2c) != 0) {
    do {
      *(undefined4 *)(*(int *)((int)param_1 + 0x30) + uVar8 * 4) =
           *(undefined4 *)
            (*(int *)(param_6 + 4) + *(int *)(*(int *)(param_6 + 0x14) + uVar8 * 4) * 4);
      *(undefined4 *)(*(int *)((int)param_1 + 0x34) + uVar8 * 4) =
           *(undefined4 *)(*(int *)(param_6 + 0x24) + uVar8 * 4);
      uVar8 = uVar8 + 1;
    } while (uVar8 < *(uint *)((int)param_1 + 0x2c));
  }
  uVar8 = 0;
  if (*(int *)((int)param_1 + 4) != 0) {
    do {
      uVar9 = 0;
      if (*(int *)(*(int *)((int)param_1 + 8) + uVar8 * 4) != 0) {
        do {
          iVar14 = *(int *)(*(int *)((int)param_1 + 0xc) + uVar8 * 4);
          *(undefined4 *)(iVar14 + uVar9 * 4) =
               *(undefined4 *)(*(int *)(param_6 + 4) + *(int *)(iVar14 + uVar9 * 4) * 4);
          uVar9 = uVar9 + 1;
        } while (uVar9 < *(uint *)(*(int *)((int)param_1 + 8) + uVar8 * 4));
      }
      uVar8 = uVar8 + 1;
    } while (uVar8 < *(uint *)((int)param_1 + 4));
  }
  *(undefined4 *)((int)param_1 + 0x24) = *(undefined4 *)(*(int *)((int)param_1 + 0x28) + 0x22c);
  iVar14 = *(int *)((int)param_1 + 4);
  *(undefined4 *)((int)param_1 + 0x38) = 0;
  if (iVar14 != 0) {
    puVar11 = *(uint **)((int)param_1 + 8);
    do {
      if (*(uint *)((int)param_1 + 0x38) < *puVar11) {
        *(uint *)((int)param_1 + 0x38) = *puVar11;
      }
      puVar11 = puVar11 + 1;
      iVar14 = iVar14 + -1;
    } while (iVar14 != 0);
  }
  pvVar7 = srHeap::allocate((srHeap *)srHeap_exref,*(int *)((int)param_1 + 0x38) * 0xc);
  if (pvVar7 == (void *)0x0) {
    pvVar7 = (void *)0x0;
  }
  *(void **)((int)param_1 + 0x3c) = pvVar7;
  uVar8 = 0;
  if (*(int *)((int)param_1 + 4) != 0) {
    do {
      uVar9 = 0;
      if (*(int *)(*(int *)((int)param_1 + 8) + uVar8 * 4) != 0) {
        iVar14 = 0;
        do {
          fVar2 = *(float *)((int)param_1 + 0x40);
          iVar3 = *(int *)(*(int *)((int)param_1 + 0x10) + uVar8 * 4);
          pfVar1 = (float *)(iVar3 + iVar14);
          pfVar10 = (float *)(iVar3 + iVar14);
          uVar9 = uVar9 + 1;
          iVar14 = iVar14 + 0xc;
          *pfVar10 = fVar2 * *pfVar1;
          pfVar10[1] = fVar2 * pfVar10[1];
          pfVar10[2] = fVar2 * pfVar10[2];
        } while (uVar9 < *(uint *)(*(int *)((int)param_1 + 8) + uVar8 * 4));
      }
      uVar8 = uVar8 + 1;
    } while (uVar8 < *(uint *)((int)param_1 + 4));
  }
  ExceptionList = local_10;
  return;
}



// public: class srMeshModel * __fastcall
// Housemarque::Threedee_Engine::Skin_Mesh::Skin::Set_Frame(class Housemarque::Bone_Import::Frame
// &,bool)

srMeshModel * __fastcall
Housemarque::Threedee_Engine::Skin_Mesh::Skin::Set_Frame(Frame *param_1,bool param_2)

{
  srVector3T<float> *psVar1;
  srVector3T<float> *psVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  int iVar8;
  int *piVar9;
  srVector3T<float> *psVar10;
  Matrix_3x3<float> *pMVar11;
  Vector_3<float> *pVVar12;
  float *pfVar13;
  undefined4 *puVar14;
  uint uVar15;
  undefined3 in_register_00000009;
  int iVar16;
  uint uVar17;
  Frame in_stack_00000004;
  int iStack_a0;
  float fStack_70;
  float fStack_6c;
  float fStack_68;
  undefined4 auStack_64 [9];
  undefined auStack_40 [64];
  
                    // 0x1a600  136
                    // ?Set_Frame@Skin@Skin_Mesh@Threedee_Engine@Housemarque@@QAIPAVsrMeshModel@@AAVFrame@Bone_Import@4@_N@Z
  if (in_stack_00000004 != param_1[0x18]) {
    param_1[0x18] = in_stack_00000004;
    srMeshModel::flipFaces(*(srMeshModel **)(param_1 + 0x28));
  }
  psVar10 = srMeshModel::getVertexLoc(*(srMeshModel **)(param_1 + 0x28));
  if (*(int *)(param_1 + 0x24) * 3 != 0) {
    (**(code **)(**(int **)vp_exref + 0x38))(psVar10,0,*(int *)(param_1 + 0x24) * 3);
  }
  uVar17 = 0;
  if (*(int *)(param_1 + 4) != 0) {
    do {
      pMVar11 = Housemarque::Bone_Import::Frame::Base(CONCAT31(in_register_00000009,param_2));
      puVar14 = auStack_64;
      iStack_a0 = 3;
      do {
        iVar16 = 3;
        do {
          *puVar14 = *(undefined4 *)(((int)pMVar11 - (int)auStack_64) + (int)puVar14);
          puVar14 = puVar14 + 1;
          iVar16 = iVar16 + -1;
        } while (iVar16 != 0);
        iStack_a0 = iStack_a0 + -1;
      } while (iStack_a0 != 0);
      pVVar12 = Housemarque::Bone_Import::Frame::Offset(CONCAT31(in_register_00000009,param_2));
      fStack_68 = *(float *)(param_1 + 0x40);
      fStack_70 = fStack_68 * *(float *)pVVar12;
      fStack_6c = fStack_68 * *(float *)(pVVar12 + 4);
      fStack_68 = fStack_68 * *(float *)(pVVar12 + 8);
      iVar16 = *(int *)(*(int *)(param_1 + 8) + uVar17 * 4);
      uVar6 = *(undefined4 *)(param_1 + 0x3c);
      uVar7 = *(undefined4 *)(*(int *)(param_1 + 0x10) + uVar17 * 4);
      if (iVar16 != 0) {
        FUN_1001ab10(auStack_40,(int)auStack_64,&fStack_70);
        (**(code **)(**(int **)vp_exref + 0x224))(uVar6,uVar7,auStack_40,iVar16);
      }
      uVar15 = 0;
      if (*(int *)(*(int *)(param_1 + 8) + uVar17 * 4) != 0) {
        iVar16 = 0;
        do {
          pfVar13 = (float *)(iVar16 + *(int *)(param_1 + 0x3c));
          fVar3 = *(float *)(*(int *)(*(int *)(param_1 + 0x14) + uVar17 * 4) + uVar15 * 4);
          uVar15 = uVar15 + 1;
          iVar16 = iVar16 + 0xc;
          fVar4 = pfVar13[1];
          fVar5 = pfVar13[2];
          iVar8 = *(int *)(*(int *)(*(int *)(param_1 + 0xc) + uVar17 * 4) + -4 + uVar15 * 4);
          psVar1 = psVar10 + iVar8 * 0xc;
          *(float *)psVar1 = *pfVar13 * fVar3 + *(float *)(psVar10 + iVar8 * 0xc);
          *(float *)(psVar1 + 4) = fVar4 * fVar3 + *(float *)(psVar1 + 4);
          *(float *)(psVar1 + 8) = fVar3 * fVar5 + *(float *)(psVar1 + 8);
        } while (uVar15 < *(uint *)(*(int *)(param_1 + 8) + uVar17 * 4));
      }
      uVar17 = uVar17 + 1;
    } while (uVar17 < *(uint *)(param_1 + 4));
  }
  uVar17 = 0;
  if (*(int *)(param_1 + 0x2c) != 0) {
    do {
      psVar1 = psVar10 + *(int *)(*(int *)(param_1 + 0x30) + uVar17 * 4) * 0xc;
      psVar2 = psVar10 + *(int *)(*(int *)(param_1 + 0x34) + uVar17 * 4) * 0xc;
      *(undefined4 *)psVar2 = *(undefined4 *)psVar1;
      *(undefined4 *)(psVar2 + 4) = *(undefined4 *)(psVar1 + 4);
      *(undefined4 *)(psVar2 + 8) = *(undefined4 *)(psVar1 + 8);
      uVar17 = uVar17 + 1;
    } while (uVar17 < *(uint *)(param_1 + 0x2c));
  }
  piVar9 = *(int **)(param_1 + 0x28);
  if ((*(byte *)(piVar9 + 0xe4) & 1) == 0) {
    uVar17 = piVar9[0xe4];
    piVar9[0xe4] = uVar17 | 1;
    piVar9[0xe4] = uVar17 | 9;
    (**(code **)(*piVar9 + 0x34))(0);
  }
  if ((*(byte *)(piVar9 + 0xe4) & 2) == 0) {
    uVar17 = piVar9[0xe4];
    piVar9[0xe4] = uVar17 | 2;
    piVar9[0xe4] = uVar17 | 10;
  }
  if ((*(byte *)(piVar9 + 0xe4) & 4) == 0) {
    uVar17 = piVar9[0xe4];
    piVar9[0xe4] = uVar17 | 4;
    piVar9[0xe4] = uVar17 | 0xc;
  }
  if ((*(byte *)(piVar9 + 0xe4) & 8) == 0) {
    uVar17 = piVar9[0xe4];
    piVar9[0xe4] = uVar17 | 8;
    piVar9[0xe4] = uVar17 | 8;
  }
  return *(srMeshModel **)(param_1 + 0x28);
}



// public: class srModel const * __fastcall
// Housemarque::Threedee_Engine::Skin_Mesh::Skin::Get_Sr_Model(void)const 

srModel * __fastcall Housemarque::Threedee_Engine::Skin_Mesh::Skin::Get_Sr_Model(void)

{
  int in_ECX;
  
                    // 0x1a8a0  93
                    // ?Get_Sr_Model@Skin@Skin_Mesh@Threedee_Engine@Housemarque@@QBIPBVsrModel@@XZ
  return *(srModel **)(in_ECX + 0x28);
}



// public: float __fastcall
// Housemarque::Threedee_Engine::Skin_Mesh::Skin::LOD_Switch_Distance(void)const 

float __fastcall Housemarque::Threedee_Engine::Skin_Mesh::Skin::LOD_Switch_Distance(void)

{
  int in_ECX;
  
                    // 0x1a8b0  109
                    // ?LOD_Switch_Distance@Skin@Skin_Mesh@Threedee_Engine@Housemarque@@QBIMXZ
  return *(float *)(in_ECX + 0x1c);
}



// public: __thiscall Housemarque::Threedee_Engine::Skin_Mesh::Skin_Mesh(void)

Skin_Mesh * __thiscall Housemarque::Threedee_Engine::Skin_Mesh::Skin_Mesh(Skin_Mesh *this)

{
  void *pvVar1;
  Skin_Mesh local_11;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0x1a8c0  21  ??0Skin_Mesh@Threedee_Engine@Housemarque@@QAE@XZ
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10023fcd;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *this = local_11;
  pvVar1 = operator_new(0xc);
  *(void **)pvVar1 = pvVar1;
  *(void **)((int)pvVar1 + 4) = pvVar1;
  *(void **)(this + 4) = pvVar1;
  *(undefined4 *)(this + 8) = 0;
  local_4 = 0;
  Trimesh::Trimesh((Trimesh *)(this + 0x10));
  ExceptionList = local_c;
  return this;
}



// public: __thiscall Housemarque::Threedee_Engine::Skin_Mesh::~Skin_Mesh(void)

void __thiscall Housemarque::Threedee_Engine::Skin_Mesh::~Skin_Mesh(Skin_Mesh *this)

{
  int *piVar1;
  int *piVar2;
  void **ppvVar3;
  int *piVar4;
  void *local_c;
  undefined *puStack_8;
  uint local_4;
  
                    // 0x1a920  41  ??1Skin_Mesh@Threedee_Engine@Housemarque@@QAE@XZ
  puStack_8 = &LAB_10023fea;
  local_c = ExceptionList;
  piVar4 = (int *)**(int **)(this + 4);
  local_4 = 1;
  ExceptionList = &local_c;
  ppvVar3 = &local_c;
  if (piVar4 != *(int **)(this + 4)) {
    do {
      if ((undefined4 *)piVar4[2] != (undefined4 *)0x0) {
        (*(code *)**(undefined4 **)piVar4[2])(1);
      }
      piVar4 = (int *)*piVar4;
      ppvVar3 = (void **)ExceptionList;
    } while (piVar4 != (int *)*(int *)(this + 4));
  }
  ExceptionList = ppvVar3;
  local_4 = local_4 & 0xffffff00;
  Trimesh::~Trimesh((Trimesh *)(this + 0x10));
  piVar4 = *(int **)(this + 4);
  piVar2 = (int *)*piVar4;
  while (piVar2 != piVar4) {
    piVar1 = (int *)*piVar2;
    *(int *)piVar2[1] = *piVar2;
    *(int *)(*piVar2 + 4) = piVar2[1];
    FUN_100212a4(piVar2);
    *(int *)(this + 8) = *(int *)(this + 8) + -1;
    piVar2 = piVar1;
  }
  FUN_100212a4(*(void **)(this + 4));
  *(undefined4 *)(this + 4) = 0;
  *(undefined4 *)(this + 8) = 0;
  ExceptionList = local_c;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Skin_Mesh::Add_Skin_Lod(class
// Housemarque::Threedee_Engine::Skin_Mesh::Skin *)

void __fastcall Housemarque::Threedee_Engine::Skin_Mesh::Add_Skin_Lod(Skin *param_1)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  undefined4 *puVar4;
  int in_EDX;
  
                    // 0x1a9d0  50
                    // ?Add_Skin_Lod@Skin_Mesh@Threedee_Engine@Housemarque@@QAIXPAVSkin@123@@Z
  puVar1 = (undefined4 *)**(undefined4 **)(param_1 + 4);
  while ((puVar1 != *(undefined4 **)(param_1 + 4) &&
         (*(float *)(puVar1[2] + 0x1c) <= *(float *)(in_EDX + 0x1c)))) {
    puVar1 = (undefined4 *)*puVar1;
  }
  puVar4 = (undefined4 *)puVar1[1];
  puVar2 = (undefined4 *)operator_new(0xc);
  puVar3 = puVar1;
  if (puVar1 == (undefined4 *)0x0) {
    puVar3 = puVar2;
  }
  *puVar2 = puVar3;
  if (puVar4 == (undefined4 *)0x0) {
    puVar4 = puVar2;
  }
  puVar2[1] = puVar4;
  puVar1[1] = puVar2;
  *(undefined4 **)puVar2[1] = puVar2;
  if (puVar2 + 2 != (int *)0x0) {
    puVar2[2] = in_EDX;
  }
  *(int *)(param_1 + 8) = *(int *)(param_1 + 8) + 1;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Skin_Mesh::Set_Frame(class
// Housemarque::Bone_Import::Frame &,float,bool)

void __fastcall
Housemarque::Threedee_Engine::Skin_Mesh::Set_Frame(Frame *param_1,float param_2,bool param_3)

{
  int *piVar1;
  int *piVar2;
  srMeshModel *psVar3;
  undefined3 in_stack_00000005;
  
                    // 0x1aa40  137
                    // ?Set_Frame@Skin_Mesh@Threedee_Engine@Housemarque@@QAIXAAVFrame@Bone_Import@3@M_N@Z
  piVar1 = *(int **)(param_1 + 4);
  piVar2 = (int *)*piVar1;
  psVar3 = (srMeshModel *)0x0;
  if (piVar2 != piVar1) {
    do {
      if (_param_3 <= *(float *)(piVar2[2] + 0x1c)) break;
      piVar2 = (int *)*piVar2;
    } while (piVar2 != piVar1);
    if (piVar2 != piVar1) {
      psVar3 = Skin::Set_Frame((Frame *)piVar2[2],SUB41(param_2,0));
    }
  }
  (**(code **)(*(int *)(param_1 + 0x148) + 4))(psVar3);
  return;
}



// public: class std::list<class Housemarque::Threedee_Engine::Skin_Mesh::Skin *,class
// std::allocator<class Housemarque::Threedee_Engine::Skin_Mesh::Skin *> > const & __fastcall
// Housemarque::Threedee_Engine::Skin_Mesh::Skin_List(void)

list<> * __fastcall Housemarque::Threedee_Engine::Skin_Mesh::Skin_List(void)

{
  list<> *in_ECX;
  
                    // 0x1aa90  148
                    // ?Skin_List@Skin_Mesh@Threedee_Engine@Housemarque@@QAIABV?$list@PAVSkin@Skin_Mesh@Threedee_Engine@Housemarque@@V?$allocator@PAVSkin@Skin_Mesh@Threedee_Engine@Housemarque@@@std@@@std@@XZ
  return in_ECX;
}



void * __thiscall FUN_1001aaa0(void *this,byte param_1)

{
  Housemarque::Threedee_Engine::Skin_Mesh::Skin::~Skin((Skin *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void __fastcall FUN_1001aac0(int param_1)

{
  int *piVar1;
  int *piVar2;
  int *piVar3;
  
  piVar1 = *(int **)(param_1 + 4);
  piVar3 = (int *)*piVar1;
  while (piVar3 != piVar1) {
    piVar2 = (int *)*piVar3;
    *(int *)piVar3[1] = *piVar3;
    *(int *)(*piVar3 + 4) = piVar3[1];
    FUN_100212a4(piVar3);
    *(int *)(param_1 + 8) = *(int *)(param_1 + 8) + -1;
    piVar3 = piVar2;
  }
  FUN_100212a4(*(void **)(param_1 + 4));
  *(undefined4 *)(param_1 + 4) = 0;
  *(undefined4 *)(param_1 + 8) = 0;
  return;
}



void __thiscall FUN_1001ab10(void *this,int param_1,undefined4 *param_2)

{
  undefined4 *puVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 *puVar5;
  int iVar6;
  undefined4 *puVar7;
  
  puVar5 = (undefined4 *)(param_1 + 8);
  iVar6 = 3;
  puVar7 = (undefined4 *)this;
  do {
    puVar1 = puVar5 + -2;
    uVar2 = puVar5[-1];
    uVar3 = *puVar5;
    puVar5 = puVar5 + 3;
    uVar4 = *param_2;
    *puVar7 = *puVar1;
    puVar7[1] = uVar2;
    puVar7[2] = uVar3;
    param_2 = param_2 + 1;
    iVar6 = iVar6 + -1;
    puVar7[3] = uVar4;
    puVar7 = puVar7 + 4;
  } while (iVar6 != 0);
  *(undefined4 *)((int)this + 0x30) = 0;
  *(undefined4 *)((int)this + 0x34) = 0;
  *(undefined4 *)((int)this + 0x38) = 0;
  *(undefined4 *)((int)this + 0x3c) = 0x3f800000;
  return;
}



void FUN_1001abd0(void)

{
                    // WARNING: Could not recover jumptable at 0x1001abd5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::Init((Init *)&DAT_1003850d);
  return;
}



void FUN_1001abe0(void)

{
  FUN_100214c2(FUN_1001abf0);
  return;
}



void FUN_1001abf0(void)

{
                    // WARNING: Could not recover jumptable at 0x1001abf5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::~Init((Init *)&DAT_1003850d);
  return;
}



void FUN_1001ac10(void)

{
                    // WARNING: Could not recover jumptable at 0x1001ac15. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::_Winit((_Winit *)&DAT_1003850c);
  return;
}



void FUN_1001ac20(void)

{
  FUN_100214c2(FUN_1001ac30);
  return;
}



void FUN_1001ac30(void)

{
                    // WARNING: Could not recover jumptable at 0x1001ac35. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::~_Winit((_Winit *)&DAT_1003850c);
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined __cdecl FUN_1001ac40(srMeshModel *param_1)

{
  int iVar1;
  int iVar2;
  undefined uVar3;
  srVector3i *psVar4;
  uint uVar5;
  int *piVar6;
  undefined uVar7;
  uint uVar8;
  uint *puVar9;
  bool bVar10;
  undefined local_46;
  undefined local_45;
  int *piStack_44;
  int iStack_40;
  int iStack_3c;
  srMeshModel *psStack_38;
  undefined4 uStack_34;
  undefined4 auStack_30 [2];
  uint auStack_28 [3];
  undefined local_1c [4];
  int *piStack_18;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10024008;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_1001c5d0(local_1c,&local_45,0,&local_46);
  iVar1 = *(int *)(param_1 + 0x230);
  iVar2 = *(int *)(param_1 + 0x22c);
  local_4 = 0;
  psVar4 = srMeshModel::getPolyVertex(param_1);
  if (iVar1 != 0) {
    do {
      iStack_40 = iVar1;
      auStack_28[0] = *(uint *)psVar4;
      auStack_28[1] = *(undefined4 *)(psVar4 + 4);
      auStack_28[2] = *(undefined4 *)(psVar4 + 8);
      puVar9 = auStack_28;
      uVar8 = 1;
      do {
        uVar5 = uVar8;
        if (uVar8 == 3) {
          uVar5 = 0;
        }
        param_1 = (srMeshModel *)FUN_1001adf0(*puVar9,auStack_28[uVar5],iVar2);
        piStack_44 = (int *)FUN_1001c720(local_1c,&iStack_3c,(uint *)&param_1);
        piStack_44 = (int *)*piStack_44;
        if (piStack_44 == piStack_18) {
          psStack_38 = param_1;
          uStack_34 = 0;
          piVar6 = (int *)FUN_1001c5a0(local_1c,auStack_30,(uint *)&psStack_38);
          *(undefined4 *)(*piVar6 + 0x10) = 1;
        }
        else {
          piStack_44[4] = piStack_44[4] + 1;
        }
        puVar9 = puVar9 + 1;
        bVar10 = uVar8 < 3;
        uVar8 = uVar8 + 1;
      } while (bVar10);
      psVar4 = psVar4 + 0xc;
      iVar1 = iStack_40 + -1;
    } while (iStack_40 + -1 != 0);
    iStack_40 = 0;
  }
  piStack_44 = (int *)*piStack_18;
  uVar7 = 1;
  uVar3 = 1;
  if (piStack_44 != piStack_18) {
    do {
      uVar7 = uVar3;
      if (piStack_44[4] != 2) {
        uVar7 = 0;
      }
      FUN_1001cf10((int *)&piStack_44);
      uVar3 = uVar7;
    } while (piStack_44 != piStack_18);
  }
  local_4 = 0xffffffff;
  FUN_1001c600(local_1c,&iStack_3c,(int *)*piStack_18,piStack_18);
  FUN_100212a4(piStack_18);
  piStack_18 = (int *)0x0;
  uStack_10 = 0;
  std::_Lockit::_Lockit((_Lockit *)&param_1);
  _DAT_10038514 = _DAT_10038514 + -1;
  if (_DAT_10038514 == 0) {
    FUN_100212a4(DAT_10038510);
    DAT_10038510 = (void *)0x0;
  }
  std::_Lockit::~_Lockit((_Lockit *)&param_1);
  ExceptionList = pvStack_c;
  return uVar7;
}



int __cdecl FUN_1001adf0(uint param_1,uint param_2,int param_3)

{
  uint uVar1;
  
  uVar1 = param_1;
  if (param_2 < param_1) {
    uVar1 = _ftol();
    param_2 = param_1;
  }
  return uVar1 * param_3 + param_2;
}



bool __cdecl FUN_1001ae30(uint param_1)

{
  return (bool)('\x01' - ((param_1 - 1 & param_1) != 0));
}



srMeshModel * __cdecl
FUN_1001ae40(char *param_1,reOrderData *param_2,list<> *param_3,float *param_4,float *param_5,
            float *param_6,undefined4 param_7)

{
  srMeshModel *psVar1;
  undefined4 *puVar2;
  basic_ostream<> *pbVar3;
  char *pcVar4;
  char *pcVar5;
  srMeshModel *psVar6;
  int iVar7;
  float afStack_3c [3];
  float afStack_30 [9];
  void *pvStack_c;
  undefined *puStack_8;
  srMeshModel *local_4;
  
  local_4 = (srMeshModel *)0xffffffff;
  puStack_8 = &LAB_1002402a;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  psVar1 = srGFInterface::getModelByName(param_1,param_3,param_2,(Import_Statistics *)0x0,false);
  if (psVar1 == (srMeshModel *)0x0) {
    puVar2 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&param_7);
    local_4 = psVar1;
    pbVar3 = std::operator<<((basic_ostream<> *)*puVar2,s_Import_error__cant_find_model__10038094);
    psVar6 = psVar1;
    pbVar3 = std::operator<<(pbVar3,(char *)param_3);
    std::operator<<(pbVar3,(char)psVar6);
    iVar7 = 0x6f;
    pcVar5 = s_E__work_ht_3de_3DE_Util_cpp_10038078;
    pcVar4 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(s_model_100380b4,pcVar4,pcVar5,iVar7);
  }
  local_4 = (srMeshModel *)0xffffffff;
  if (psVar1 == (srMeshModel *)0x0) {
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&param_7);
  }
  FUN_10005770(param_1,(char *)param_3,afStack_3c);
  FUN_100058f0(param_1,(char *)param_3,afStack_30);
  if (Housemarque::Threedee_Engine::Trimesh_Import_Skip_Transform_Hack != true) {
    FUN_1001af50(psVar1,param_4,param_5,param_6,afStack_30,afStack_3c);
  }
  ExceptionList = pvStack_c;
  return psVar1;
}



void __cdecl
FUN_1001af50(srMeshModel *param_1,float *param_2,float *param_3,float *param_4,float *param_5,
            float *param_6)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  int iVar7;
  uint uVar8;
  srVector3T<float> *psVar9;
  srVector3T<float> *psVar10;
  srVector3T<float> *psVar11;
  int iVar12;
  float *pfVar13;
  float10 fVar14;
  float10 fVar15;
  float afStack_24 [6];
  float afStack_c [3];
  
  iVar7 = *(int *)(param_1 + 0x22c);
  psVar9 = srMeshModel::getVertexLoc(param_1);
  pfVar13 = afStack_24;
  for (iVar12 = 9; iVar12 != 0; iVar12 = iVar12 + -1) {
    *pfVar13 = *param_2;
    param_2 = param_2 + 1;
    pfVar13 = pfVar13 + 1;
  }
  for (; iVar7 != 0; iVar7 = iVar7 + -1) {
    fVar1 = *(float *)(psVar9 + 8);
    fVar2 = param_5[2];
    fVar3 = *(float *)(psVar9 + 4);
    fVar4 = param_5[1];
    fVar5 = *(float *)psVar9;
    fVar6 = *param_5;
    fVar14 = FUN_1001d3e0(param_5 + 3,(float *)psVar9);
    fVar15 = FUN_1001d3e0(param_5 + 6,(float *)psVar9);
    *(float *)psVar9 = fVar5 * fVar6 + fVar3 * fVar4 + fVar1 * fVar2;
    *(float *)(psVar9 + 4) = (float)fVar14;
    *(float *)(psVar9 + 8) = (float)fVar15;
    *(float *)psVar9 = *(float *)psVar9 + *param_6;
    *(float *)(psVar9 + 4) = param_6[1] + *(float *)(psVar9 + 4);
    *(float *)(psVar9 + 8) = *(float *)(psVar9 + 8) + param_6[2];
    fVar1 = afStack_24[0] * *(float *)psVar9;
    fVar3 = afStack_24[1] * *(float *)(psVar9 + 4);
    fVar2 = afStack_24[2] * *(float *)(psVar9 + 8);
    fVar14 = FUN_1001d3e0(afStack_24 + 3,(float *)psVar9);
    fVar15 = FUN_1001d3e0(afStack_c,(float *)psVar9);
    *(float *)psVar9 = fVar2 + fVar3 + fVar1;
    *(float *)(psVar9 + 4) = (float)fVar14;
    *(float *)(psVar9 + 8) = (float)fVar15;
    iVar12 = 3;
    psVar10 = psVar9;
    pfVar13 = param_4;
    do {
      fVar1 = *pfVar13;
      psVar11 = psVar10 + 4;
      pfVar13 = pfVar13 + 1;
      iVar12 = iVar12 + -1;
      *(float *)psVar10 = fVar1 * *(float *)psVar10;
      psVar10 = psVar11;
    } while (iVar12 != 0);
    iVar12 = 3;
    pfVar13 = param_3;
    do {
      fVar1 = *pfVar13;
      pfVar13 = pfVar13 + 1;
      iVar12 = iVar12 + -1;
      *(float *)psVar9 = fVar1 + *(float *)psVar9;
      psVar9 = psVar9 + 4;
    } while (iVar12 != 0);
    psVar9 = psVar11;
  }
  if (((byte)param_1[0x390] & 1) == 0) {
    uVar8 = *(uint *)(param_1 + 0x390);
    *(uint *)(param_1 + 0x390) = uVar8 | 1;
    *(uint *)(param_1 + 0x390) = uVar8 | 9;
    (**(code **)(*(int *)param_1 + 0x34))(0);
  }
  if (((byte)param_1[0x390] & 2) == 0) {
    uVar8 = *(uint *)(param_1 + 0x390);
    *(uint *)(param_1 + 0x390) = uVar8 | 2;
    *(uint *)(param_1 + 0x390) = uVar8 | 10;
  }
  if (((byte)param_1[0x390] & 4) == 0) {
    uVar8 = *(uint *)(param_1 + 0x390);
    *(uint *)(param_1 + 0x390) = uVar8 | 4;
    *(uint *)(param_1 + 0x390) = uVar8 | 0xc;
  }
  if (((byte)param_1[0x390] & 8) == 0) {
    uVar8 = *(uint *)(param_1 + 0x390);
    *(uint *)(param_1 + 0x390) = uVar8 | 8;
    *(uint *)(param_1 + 0x390) = uVar8 | 8;
  }
  return;
}



void __cdecl FUN_1001b140(srMeshModel *param_1,char param_2)

{
  undefined4 *puVar1;
  srShader *psVar2;
  srPtr<> *psVar3;
  srPtr<> *psVar4;
  int *piVar5;
  int iVar6;
  uint uVar7;
  e_side eVar8;
  uint uVar9;
  uint local_38;
  uint local_34;
  srVector3i *local_30;
  undefined4 local_2c [4];
  undefined4 local_1c;
  uint local_18;
  srVector3T<float> *local_14;
  undefined4 local_10 [4];
  
  if (param_1 != (srMeshModel *)0x0) {
    local_34 = 0;
    local_18 = 0;
    local_30 = (srVector3i *)0x0;
    local_14 = (srVector3T<float> *)0x0;
    puVar1 = local_10;
    iVar6 = 4;
    do {
      puVar1[-7] = 0;
      *puVar1 = 0;
      puVar1 = puVar1 + 1;
      iVar6 = iVar6 + -1;
    } while (iVar6 != 0);
    local_38 = *(uint *)(param_1 + 0x230);
    local_1c = *(undefined4 *)(param_1 + 0x22c);
    local_30 = srMeshModel::getPolyVertex(param_1);
    local_14 = srMeshModel::getVertexLoc(param_1);
    uVar9 = 0;
    local_34 = 0;
    local_18 = 0;
    if (*(int *)(param_1 + 0x228) != 0) {
      do {
        psVar2 = srMeshModel::getPolyShader(param_1,uVar9,0);
        if ((psVar2 != (srShader *)0x0) && (local_34 < 4)) {
          psVar2 = srMeshModel::getPolyShader(param_1,uVar9,1);
          local_2c[local_34] = psVar2;
          local_34 = local_34 + 1;
        }
        uVar7 = 0;
        do {
          psVar3 = srMeshModel::getPolyTexture(param_1,uVar9,uVar7,0);
          if ((psVar3 != (srPtr<> *)0x0) && (local_34 < 4)) {
            psVar3 = srMeshModel::getPolyTexture(param_1,uVar9,uVar7,1);
            local_2c[local_34] = psVar3;
            local_34 = local_34 + 1;
          }
          uVar7 = uVar7 + 1;
        } while (uVar7 < 2);
        eVar8 = 0;
        do {
          psVar4 = srMeshModel::getVertexMaterial(param_1,uVar9,eVar8,0);
          if ((psVar4 != (srPtr<> *)0x0) && (local_18 < 4)) {
            psVar4 = srMeshModel::getVertexMaterial(param_1,uVar9,eVar8,1);
            local_10[local_18] = psVar4;
            local_18 = local_18 + 1;
          }
          eVar8 = eVar8 + 1;
        } while (eVar8 < 2);
        uVar9 = uVar9 + 1;
      } while (uVar9 < *(uint *)(param_1 + 0x228));
    }
    piVar5 = FUN_1001b340(&local_38,(-(uint)(param_2 != '\0') & 0xfffffffe) + 7);
    if (piVar5 != (int *)0x0) {
      (**(code **)(*(int *)param_1 + 0x38))(*piVar5);
      (**(code **)(*(int *)param_1 + 0x3c))(piVar5[1]);
      FUN_100212a4((void *)*piVar5);
      FUN_100212a4((void *)piVar5[1]);
      FUN_100212a4(piVar5);
    }
  }
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __fastcall FUN_1001b2d0(void *param_1)

{
  void *local_4;
  
  local_4 = param_1;
  FUN_1001c600(param_1,&local_4,(int *)**(int **)((int)param_1 + 4),*(int **)((int)param_1 + 4));
  FUN_100212a4(*(void **)((int)param_1 + 4));
  *(undefined4 *)((int)param_1 + 4) = 0;
  *(undefined4 *)((int)param_1 + 0xc) = 0;
  std::_Lockit::_Lockit((_Lockit *)&local_4);
  _DAT_10038514 = _DAT_10038514 + -1;
  if (_DAT_10038514 == 0) {
    FUN_100212a4(DAT_10038510);
    DAT_10038510 = (void *)0x0;
  }
  std::_Lockit::~_Lockit((_Lockit *)&local_4);
  return;
}



int * __cdecl FUN_1001b340(uint *param_1,uint param_2)

{
  undefined4 uVar1;
  int *piVar2;
  int *piVar3;
  void *pvVar4;
  uint uVar5;
  undefined4 *puVar6;
  undefined4 *puVar7;
  int *piVar8;
  int *piVar9;
  uint *puVar10;
  uint uVar11;
  int iVar12;
  int iVar13;
  uint *puVar14;
  undefined4 *puVar15;
  undefined4 *puVar16;
  int iVar17;
  int iVar18;
  uint *puVar19;
  undefined4 *puVar20;
  uint uVar21;
  undefined4 *puVar22;
  uint local_50;
  uint *local_4c;
  uint local_48;
  int local_44;
  int local_40;
  uint local_3c;
  uint *local_38;
  undefined4 *local_34;
  undefined4 uStack_30;
  int *local_2c;
  uint local_28;
  undefined4 *local_24;
  uint local_20;
  void *local_1c;
  void *local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_1002405b;
  local_c = ExceptionList;
  if ((*param_1 == 0) || (param_1[7] == 0)) {
    return (int *)0x0;
  }
  ExceptionList = &local_c;
  piVar3 = (int *)operator_new(8);
  local_4 = 0;
  local_2c = piVar3;
  if (piVar3 == (int *)0x0) {
    piVar3 = (int *)0x0;
  }
  else {
    uVar5 = param_1[7];
    pvVar4 = operator_new(*param_1 << 2);
    *piVar3 = (int)pvVar4;
    pvVar4 = operator_new(uVar5 * 4);
    piVar3[1] = (int)pvVar4;
  }
  uVar5 = 0;
  local_4 = 0xffffffff;
  if (*param_1 != 0) {
    do {
      *(uint *)(*piVar3 + uVar5 * 4) = uVar5;
      uVar5 = uVar5 + 1;
    } while (uVar5 < *param_1);
  }
  uVar5 = 0;
  if (param_1[7] != 0) {
    do {
      *(uint *)(piVar3[1] + uVar5 * 4) = uVar5;
      uVar5 = uVar5 + 1;
    } while (uVar5 < param_1[7]);
  }
  local_38 = (uint *)(param_2 & 1);
  if ((local_38 == (uint *)0x0) && ((param_2 & 2) == 0)) {
    ExceptionList = local_c;
    return piVar3;
  }
  puVar6 = (undefined4 *)operator_new(*param_1 << 2);
  puVar7 = (undefined4 *)operator_new(param_1[7] << 2);
  iVar17 = *param_1 * 4;
  if (0 < iVar17) {
    if ((iVar17 < 8) || (((uint)puVar6 & 7) == 0)) {
      puVar15 = puVar6;
      for (uVar5 = *param_1 & 0x3fffffff; uVar5 != 0; uVar5 = uVar5 - 1) {
        *puVar15 = 0;
        puVar15 = puVar15 + 1;
      }
      for (iVar17 = 0; iVar17 != 0; iVar17 = iVar17 + -1) {
        *(undefined *)puVar15 = 0;
        puVar15 = (undefined4 *)((int)puVar15 + 1);
      }
    }
    else {
      uVar21 = 8 - ((uint)puVar6 & 7);
      puVar15 = puVar6;
      for (uVar5 = uVar21 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        *puVar15 = 0;
        puVar15 = puVar15 + 1;
      }
      for (uVar5 = uVar21 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *(undefined *)puVar15 = 0;
        puVar15 = (undefined4 *)((int)puVar15 + 1);
      }
      puVar15 = (undefined4 *)(uVar21 + (int)puVar6);
      for (uVar5 = iVar17 - uVar21 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        *puVar15 = 0;
        puVar15 = puVar15 + 1;
      }
      for (uVar5 = iVar17 - uVar21 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *(undefined *)puVar15 = 0;
        puVar15 = (undefined4 *)((int)puVar15 + 1);
      }
    }
  }
  local_34 = (undefined4 *)(param_1[7] * 4);
  if (0 < (int)local_34) {
    if (((int)local_34 < 8) || (((uint)puVar7 & 7) == 0)) {
      puVar15 = puVar7;
      for (uVar5 = param_1[7] & 0x3fffffff; uVar5 != 0; uVar5 = uVar5 - 1) {
        *puVar15 = 0;
        puVar15 = puVar15 + 1;
      }
      for (iVar17 = 0; iVar17 != 0; iVar17 = iVar17 + -1) {
        *(undefined *)puVar15 = 0;
        puVar15 = (undefined4 *)((int)puVar15 + 1);
      }
    }
    else {
      uVar21 = 8 - ((uint)puVar7 & 7);
      puVar15 = puVar7;
      for (uVar5 = uVar21 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        *puVar15 = 0;
        puVar15 = puVar15 + 1;
      }
      for (uVar5 = uVar21 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *(undefined *)puVar15 = 0;
        puVar15 = (undefined4 *)((int)puVar15 + 1);
      }
      puVar15 = (undefined4 *)(uVar21 + (int)puVar7);
      for (uVar5 = (int)local_34 - uVar21 >> 2; uVar5 != 0; uVar5 = uVar5 - 1) {
        *puVar15 = 0;
        puVar15 = puVar15 + 1;
      }
      for (uVar5 = (int)local_34 - uVar21 & 3; uVar5 != 0; uVar5 = uVar5 - 1) {
        *(undefined *)puVar15 = 0;
        puVar15 = (undefined4 *)((int)puVar15 + 1);
      }
    }
  }
  if (local_38 != (uint *)0x0) {
    if (param_1[1] != 0) {
      piVar8 = (int *)operator_new(*param_1 << 2);
      local_3c = 0;
      if (param_1[1] != 0) {
        local_4c = param_1 + 3;
        do {
          if (*param_1 != 0) {
            (**(code **)(**(int **)vp_exref + 0xa4))(piVar8,*local_4c,*piVar3,*param_1);
          }
          uVar5 = 0;
          local_48 = 1;
          if (1 < *param_1) {
            do {
              if (puVar6[local_48] != puVar6[local_48 - 1]) {
                iVar17 = local_48 - uVar5;
                local_34 = (undefined4 *)(*piVar3 + uVar5 * 4);
                puVar14 = (uint *)(piVar8 + uVar5);
                uVar5 = local_48;
                if (1 < iVar17) {
                  iVar12 = 0;
                  local_40 = 0;
                  if (1 < iVar17) {
                    iVar18 = iVar17 + -1;
                    puVar10 = puVar14;
                    do {
                      if (*puVar10 <= puVar10[1]) {
                        iVar12 = iVar12 + 1;
                      }
                      iVar18 = iVar18 + -1;
                      puVar10 = puVar10 + 1;
                      local_40 = iVar12;
                    } while (iVar18 != 0);
                  }
                  if (local_40 + 1 != iVar17) {
                    if (local_40 < iVar17 / 3) {
                      local_44 = iVar17 / 2;
                      if (0 < local_44) {
                        puVar10 = puVar14 + iVar17 + -1;
                        iVar12 = (int)local_34 - (int)puVar14;
                        puVar19 = puVar14;
                        do {
                          uVar1 = *(undefined4 *)(iVar12 + (int)puVar19);
                          *(undefined4 *)(iVar12 + (int)puVar19) =
                               *(undefined4 *)(iVar12 + (int)puVar10);
                          *(undefined4 *)(iVar12 + (int)puVar10) = uVar1;
                          uVar21 = *puVar19;
                          *puVar19 = *puVar10;
                          *puVar10 = uVar21;
                          puVar10 = puVar10 + -1;
                          puVar19 = puVar19 + 1;
                          local_44 = local_44 + -1;
                        } while (local_44 != 0);
                      }
                      if (local_40 != 0) {
                        FUN_1001daa0((int)local_34,(int)puVar14,0,iVar17);
                      }
                    }
                    else if (local_40 < 0x32) {
                      FUN_1001daa0((int)local_34,(int)puVar14,0,iVar17);
                    }
                    else {
                      FUN_1001db40((int)local_34,(int)puVar14,0,iVar17 + -1);
                    }
                  }
                }
              }
              local_48 = local_48 + 1;
            } while (local_48 < *param_1);
          }
          iVar17 = *param_1 - uVar5;
          local_34 = (undefined4 *)(*piVar3 + uVar5 * 4);
          local_38 = (uint *)(piVar8 + uVar5);
          if (1 < iVar17) {
            iVar12 = 0;
            local_44 = 0;
            if (1 < iVar17) {
              local_40 = iVar17 + -1;
              puVar14 = local_38;
              do {
                if (*puVar14 <= puVar14[1]) {
                  iVar12 = iVar12 + 1;
                }
                local_40 = local_40 + -1;
                puVar14 = puVar14 + 1;
                local_44 = iVar12;
              } while (local_40 != 0);
            }
            if (local_44 + 1 != iVar17) {
              if (local_44 < iVar17 / 3) {
                local_40 = iVar17 / 2;
                if (0 < local_40) {
                  puVar14 = local_38 + iVar17 + -1;
                  iVar12 = (int)local_34 - (int)local_38;
                  puVar10 = local_38;
                  do {
                    uVar1 = *(undefined4 *)(iVar12 + (int)puVar10);
                    *(undefined4 *)(iVar12 + (int)puVar10) = *(undefined4 *)(iVar12 + (int)puVar14);
                    *(undefined4 *)(iVar12 + (int)puVar14) = uVar1;
                    uVar5 = *puVar10;
                    *puVar10 = *puVar14;
                    *puVar14 = uVar5;
                    puVar14 = puVar14 + -1;
                    puVar10 = puVar10 + 1;
                    local_40 = local_40 + -1;
                  } while (local_40 != 0);
                }
                if (local_44 != 0) {
                  FUN_1001daa0((int)local_34,(int)local_38,0,iVar17);
                }
              }
              else if (local_44 < 0x32) {
                FUN_1001daa0((int)local_34,(int)local_38,0,iVar17);
              }
              else {
                FUN_1001db40((int)local_34,(int)local_38,0,iVar17 + -1);
              }
            }
          }
          iVar17 = 0;
          *puVar6 = 0;
          uVar5 = 1;
          if (1 < *param_1) {
            piVar2 = piVar8;
            do {
              piVar9 = piVar2 + 1;
              if (*piVar9 != *piVar2) {
                iVar17 = iVar17 + 1;
              }
              *(int *)(((int)puVar6 - (int)piVar8) + (int)piVar9) = iVar17;
              uVar5 = uVar5 + 1;
              piVar2 = piVar9;
            } while (uVar5 < *param_1);
          }
          local_3c = local_3c + 1;
          local_4c = local_4c + 1;
        } while (local_3c < param_1[1]);
      }
      FUN_100212a4(piVar8);
    }
    if ((param_2 & 4) != 0) {
      uVar5 = *param_1;
      local_50 = 0;
      if (uVar5 != 0) {
        do {
          uVar21 = local_50;
          if (local_50 < uVar5) {
            piVar8 = puVar6 + local_50;
            do {
              if (*piVar8 != puVar6[local_50]) break;
              uVar21 = uVar21 + 1;
              piVar8 = piVar8 + 1;
            } while (uVar21 < uVar5);
          }
          uVar21 = uVar21 - local_50;
          if (2 < uVar21) {
            uVar5 = param_1[2];
            local_10 = 0;
            local_1c = (void *)0x0;
            local_18 = (void *)0x0;
            local_14 = 0xffffffff;
            FUN_1001d7b0((int *)&local_1c);
            local_4 = 1;
            local_28 = uVar5;
            local_20 = uVar21;
            local_24 = (undefined4 *)operator_new(uVar21 * 8);
            uVar5 = 0;
            if (local_20 != 0) {
              do {
                local_24[uVar5 * 2] = 0;
                local_24[uVar5 * 2 + 1] = 0;
                uVar5 = uVar5 + 1;
              } while (uVar5 < local_20);
            }
            uVar5 = 0;
            local_4 = 2;
            if (uVar21 != 0) {
              iVar17 = local_50 << 2;
              do {
                local_24[uVar5 * 2] = *(undefined4 *)(iVar17 + *piVar3);
                local_24[uVar5 * 2 + 1] = 0;
                uVar5 = uVar5 + 1;
                iVar17 = iVar17 + 4;
              } while (uVar5 < uVar21);
            }
            piVar8 = (int *)(*piVar3 + local_50 * 4);
            FUN_1001c390((int *)&local_28);
            puVar10 = (uint *)operator_new(local_20 * 4);
            uVar5 = 0;
            puVar14 = puVar10;
            if (local_20 != 0) {
              do {
                uVar11 = FUN_1001c250(&local_28,uVar5);
                *puVar14 = uVar11;
                uVar5 = uVar5 + 1;
                puVar14 = puVar14 + 1;
              } while (uVar5 < local_20);
            }
            FUN_1001d4b0(local_24,puVar10,local_20);
            FUN_1001c390((int *)&local_28);
            FUN_100212a4(puVar10);
            uVar5 = 0;
            if (local_20 != 0) {
              do {
                piVar2 = local_24 + uVar5 * 2;
                if ((local_24 + uVar5 * 2)[1] == 0) {
joined_r0x1001b999:
                  piVar9 = piVar2;
                  if (piVar9 != (int *)0x0) {
                    piVar9[1] = 1;
                    *piVar8 = *piVar9;
                    piVar8 = piVar8 + 1;
                    uVar11 = 0;
                    do {
                      local_3c = FUN_1001c1f0(&local_28,piVar9,uVar11);
                      for (iVar17 = FUN_1001d430(&local_1c,&local_3c,-1); iVar17 != -1;
                          iVar17 = FUN_1001d430(&local_1c,&local_3c,iVar17)) {
                        piVar2 = *(int **)((int)local_18 + iVar17 * 0xc + 8);
                        if (piVar2[1] == 0) goto joined_r0x1001b999;
                      }
                      uVar11 = uVar11 + 1;
                    } while (uVar11 < 3);
                  }
                }
                uVar5 = uVar5 + 1;
              } while (uVar5 < local_20);
            }
            local_4 = 0xffffffff;
            FUN_100212a4(local_24);
            if (local_1c != (void *)0x0) {
              FUN_100212a4(local_1c);
            }
            if (local_18 != (void *)0x0) {
              FUN_100212a4(local_18);
            }
          }
          uVar5 = *param_1;
          local_50 = local_50 + uVar21;
        } while (local_50 < uVar5);
      }
    }
  }
  if ((param_2 & 2) == 0) goto LAB_1001c1b0;
  if (param_1[8] != 0) {
    piVar8 = (int *)operator_new(param_1[7] << 2);
    local_50 = 0;
    if (param_1[8] != 0) {
      local_4c = param_1 + 10;
      do {
        if (param_1[7] != 0) {
          (**(code **)(**(int **)vp_exref + 0xa4))(piVar8,*local_4c,piVar3[1],param_1[7]);
        }
        uVar5 = 0;
        local_48 = 1;
        if (1 < param_1[7]) {
          do {
            if (puVar7[local_48] != puVar7[local_48 - 1]) {
              iVar17 = local_48 - uVar5;
              local_34 = (undefined4 *)(piVar3[1] + uVar5 * 4);
              puVar14 = (uint *)(piVar8 + uVar5);
              uVar5 = local_48;
              if (1 < iVar17) {
                iVar12 = 0;
                local_44 = 0;
                if (1 < iVar17) {
                  iVar18 = iVar17 + -1;
                  puVar10 = puVar14;
                  do {
                    if (*puVar10 <= puVar10[1]) {
                      iVar12 = iVar12 + 1;
                    }
                    iVar18 = iVar18 + -1;
                    puVar10 = puVar10 + 1;
                    local_44 = iVar12;
                  } while (iVar18 != 0);
                }
                if (local_44 + 1 != iVar17) {
                  if (local_44 < iVar17 / 3) {
                    local_40 = iVar17 / 2;
                    if (0 < local_40) {
                      puVar10 = puVar14 + iVar17 + -1;
                      iVar12 = (int)local_34 - (int)puVar14;
                      puVar19 = puVar14;
                      do {
                        uVar1 = *(undefined4 *)(iVar12 + (int)puVar19);
                        *(undefined4 *)(iVar12 + (int)puVar19) =
                             *(undefined4 *)(iVar12 + (int)puVar10);
                        *(undefined4 *)(iVar12 + (int)puVar10) = uVar1;
                        uVar21 = *puVar19;
                        *puVar19 = *puVar10;
                        *puVar10 = uVar21;
                        puVar10 = puVar10 + -1;
                        puVar19 = puVar19 + 1;
                        local_40 = local_40 + -1;
                      } while (local_40 != 0);
                    }
                    if (local_44 != 0) {
                      FUN_1001daa0((int)local_34,(int)puVar14,0,iVar17);
                    }
                  }
                  else if (local_44 < 0x32) {
                    FUN_1001daa0((int)local_34,(int)puVar14,0,iVar17);
                  }
                  else {
                    FUN_1001db40((int)local_34,(int)puVar14,0,iVar17 + -1);
                  }
                }
              }
            }
            local_48 = local_48 + 1;
          } while (local_48 < param_1[7]);
        }
        iVar17 = param_1[7] - uVar5;
        local_34 = (undefined4 *)(piVar3[1] + uVar5 * 4);
        local_38 = (uint *)(piVar8 + uVar5);
        if (1 < iVar17) {
          iVar12 = 0;
          local_44 = 0;
          if (1 < iVar17) {
            local_40 = iVar17 + -1;
            puVar14 = local_38;
            do {
              if (*puVar14 <= puVar14[1]) {
                iVar12 = iVar12 + 1;
              }
              local_40 = local_40 + -1;
              puVar14 = puVar14 + 1;
              local_44 = iVar12;
            } while (local_40 != 0);
          }
          if (local_44 + 1 != iVar17) {
            if (local_44 < iVar17 / 3) {
              local_40 = iVar17 / 2;
              if (0 < local_40) {
                puVar14 = local_38 + iVar17 + -1;
                iVar12 = (int)local_34 - (int)local_38;
                puVar10 = local_38;
                do {
                  uVar1 = *(undefined4 *)(iVar12 + (int)puVar10);
                  *(undefined4 *)(iVar12 + (int)puVar10) = *(undefined4 *)(iVar12 + (int)puVar14);
                  *(undefined4 *)(iVar12 + (int)puVar14) = uVar1;
                  uVar5 = *puVar10;
                  *puVar10 = *puVar14;
                  *puVar14 = uVar5;
                  puVar14 = puVar14 + -1;
                  puVar10 = puVar10 + 1;
                  local_40 = local_40 + -1;
                } while (local_40 != 0);
              }
              if (local_44 != 0) {
                FUN_1001daa0((int)local_34,(int)local_38,0,iVar17);
              }
            }
            else if (local_44 < 0x32) {
              FUN_1001daa0((int)local_34,(int)local_38,0,iVar17);
            }
            else {
              FUN_1001db40((int)local_34,(int)local_38,0,iVar17 + -1);
            }
          }
        }
        iVar17 = 0;
        *puVar7 = 0;
        uVar5 = 1;
        if (1 < param_1[7]) {
          piVar2 = piVar8;
          do {
            piVar9 = piVar2 + 1;
            if (*piVar9 != *piVar2) {
              iVar17 = iVar17 + 1;
            }
            *(int *)(((int)puVar7 - (int)piVar8) + (int)piVar9) = iVar17;
            uVar5 = uVar5 + 1;
            piVar2 = piVar9;
          } while (uVar5 < param_1[7]);
        }
        local_50 = local_50 + 1;
        local_4c = local_4c + 1;
      } while (local_50 < param_1[8]);
    }
    FUN_100212a4(piVar8);
  }
  local_4c = (uint *)srHeap::allocate((srHeap *)srHeap_exref,*param_1 * 0xc);
  if (local_4c == (uint *)0x0) {
    local_4c = (uint *)0x0;
  }
  piVar8 = (int *)operator_new(param_1[7] << 2);
  local_2c = piVar8;
  pvVar4 = operator_new(param_1[7] << 2);
  uVar5 = 0;
  if (param_1[7] != 0) {
    do {
      piVar8[*(int *)(piVar3[1] + uVar5 * 4)] = uVar5;
      *(undefined4 *)((int)pvVar4 + uVar5 * 4) = 0;
      uVar5 = uVar5 + 1;
    } while (uVar5 < param_1[7]);
  }
  uVar5 = 0;
  puVar14 = local_4c;
  if (*param_1 != 0) {
    do {
      uVar21 = 0;
      do {
        *puVar14 = piVar8[*(int *)(param_1[2] + (uVar21 + *(int *)(*piVar3 + uVar5 * 4) * 3) * 4)];
        uVar21 = uVar21 + 1;
        puVar14 = (uint *)((int *)puVar14 + 1);
      } while (uVar21 < 3);
      uVar5 = uVar5 + 1;
    } while (uVar5 < *param_1);
  }
  uVar5 = 0;
  iVar17 = 0;
  param_2 = 0;
  puVar14 = local_4c;
  if (*param_1 != 0) {
    do {
      iVar12 = 3;
      do {
        if (*(int *)((int)pvVar4 + *puVar14 * 4) == 0) {
          iVar17 = iVar17 + 1;
          *(int *)((int)pvVar4 + *puVar14 * 4) = iVar17;
        }
        puVar14 = (uint *)((int *)puVar14 + 1);
        iVar12 = iVar12 + -1;
      } while (iVar12 != 0);
      param_2 = param_2 + 1;
    } while (param_2 < *param_1);
  }
  param_2 = 1;
  if (1 < param_1[7]) {
    do {
      if (puVar7[param_2] != puVar7[param_2 - 1]) {
        iVar12 = param_2 - uVar5;
        local_38 = (uint *)(uVar5 * 4 + (int)pvVar4);
        iVar17 = piVar3[1] + uVar5 * 4;
        uVar5 = param_2;
        if (1 < iVar12) {
          iVar18 = 0;
          local_44 = 0;
          if (1 < iVar12) {
            iVar13 = iVar12 + -1;
            puVar14 = local_38;
            do {
              if (*puVar14 <= puVar14[1]) {
                iVar18 = iVar18 + 1;
              }
              iVar13 = iVar13 + -1;
              puVar14 = puVar14 + 1;
              local_44 = iVar18;
            } while (iVar13 != 0);
          }
          if (local_44 + 1 != iVar12) {
            if (local_44 < iVar12 / 3) {
              local_40 = iVar12 / 2;
              if (0 < local_40) {
                puVar14 = local_38 + iVar12 + -1;
                iVar18 = iVar17 - (int)local_38;
                puVar10 = local_38;
                do {
                  uVar1 = *(undefined4 *)(iVar18 + (int)puVar10);
                  *(undefined4 *)(iVar18 + (int)puVar10) = *(undefined4 *)(iVar18 + (int)puVar14);
                  *(undefined4 *)(iVar18 + (int)puVar14) = uVar1;
                  uVar21 = *puVar10;
                  *puVar10 = *puVar14;
                  *puVar14 = uVar21;
                  puVar14 = puVar14 + -1;
                  puVar10 = puVar10 + 1;
                  local_40 = local_40 + -1;
                } while (local_40 != 0);
              }
              if (local_44 != 0) {
LAB_1001bf5f:
                local_44 = 1;
                if (1 < iVar12) {
                  local_34 = (undefined4 *)(iVar17 + 4);
                  puVar14 = local_38;
                  do {
                    uStack_30 = *local_34;
                    uVar21 = puVar14[1];
                    iVar18 = local_44;
                    if (uVar21 < *puVar14) {
                      puVar10 = puVar14;
                      puVar15 = local_34;
                      do {
                        puVar10[1] = *puVar10;
                        *puVar15 = *(undefined4 *)((int)puVar10 + (iVar17 - (int)local_38));
                        iVar18 = iVar18 + -1;
                        puVar15 = puVar15 + -1;
                        puVar10 = puVar10 + -1;
                        if (iVar18 == 0) break;
                      } while (uVar21 < *puVar10);
                    }
                    local_38[iVar18] = uVar21;
                    *(undefined4 *)(iVar17 + iVar18 * 4) = uStack_30;
                    local_44 = local_44 + 1;
                    local_34 = local_34 + 1;
                    puVar14 = puVar14 + 1;
                  } while (local_44 < iVar12);
                }
              }
            }
            else {
              if (local_44 < 0x32) goto LAB_1001bf5f;
              FUN_1001db40(iVar17,(int)local_38,0,iVar12 + -1);
            }
          }
        }
      }
      param_2 = param_2 + 1;
    } while (param_2 < param_1[7]);
  }
  iVar17 = param_1[7] - uVar5;
  puVar15 = (undefined4 *)(piVar3[1] + uVar5 * 4);
  local_38 = (uint *)(uVar5 * 4 + (int)pvVar4);
  if (1 < iVar17) {
    puVar22 = (undefined4 *)0x0;
    local_34 = (undefined4 *)0x0;
    if (1 < iVar17) {
      iVar12 = iVar17 + -1;
      puVar14 = local_38;
      do {
        if (*puVar14 <= puVar14[1]) {
          puVar22 = (undefined4 *)((int)puVar22 + 1);
        }
        iVar12 = iVar12 + -1;
        puVar14 = puVar14 + 1;
        local_34 = puVar22;
      } while (iVar12 != 0);
    }
    if ((int)local_34 + 1 != iVar17) {
      if ((int)local_34 < iVar17 / 3) {
        param_2 = iVar17 / 2;
        if (0 < (int)param_2) {
          puVar14 = local_38 + iVar17 + -1;
          iVar12 = (int)puVar15 - (int)local_38;
          puVar10 = local_38;
          do {
            uVar1 = *(undefined4 *)(iVar12 + (int)puVar10);
            *(undefined4 *)(iVar12 + (int)puVar10) = *(undefined4 *)(iVar12 + (int)puVar14);
            *(undefined4 *)(iVar12 + (int)puVar14) = uVar1;
            uVar5 = *puVar10;
            *puVar10 = *puVar14;
            *puVar14 = uVar5;
            puVar14 = puVar14 + -1;
            puVar10 = puVar10 + 1;
            param_2 = param_2 - 1;
          } while (param_2 != 0);
        }
        if (local_34 != (undefined4 *)0x0) {
LAB_1001c0e6:
          local_34 = (undefined4 *)0x1;
          puVar14 = local_38;
          puVar22 = puVar15;
          if (1 < iVar17) {
            do {
              puVar22 = puVar22 + 1;
              uStack_30 = *puVar22;
              uVar5 = puVar14[1];
              puVar16 = local_34;
              if (uVar5 < *puVar14) {
                puVar10 = puVar14;
                puVar20 = puVar22;
                do {
                  puVar10[1] = *puVar10;
                  *puVar20 = *(undefined4 *)((int)puVar10 + ((int)puVar15 - (int)local_38));
                  puVar16 = (undefined4 *)((int)puVar16 + -1);
                  puVar20 = puVar20 + -1;
                  puVar10 = puVar10 + -1;
                  if (puVar16 == (undefined4 *)0x0) break;
                } while (uVar5 < *puVar10);
              }
              local_38[(int)puVar16] = uVar5;
              puVar15[(int)puVar16] = uStack_30;
              local_34 = (undefined4 *)((int)local_34 + 1);
              puVar14 = puVar14 + 1;
            } while ((int)local_34 < iVar17);
          }
        }
      }
      else {
        if ((int)local_34 < 0x32) goto LAB_1001c0e6;
        FUN_1001db40((int)puVar15,(int)local_38,0,iVar17 + -1);
      }
    }
  }
  srHeap::free(local_4c);
  FUN_100212a4(local_2c);
  FUN_100212a4(pvVar4);
LAB_1001c1b0:
  FUN_100212a4(puVar6);
  FUN_100212a4(puVar7);
  ExceptionList = local_c;
  return piVar3;
}



uint __thiscall FUN_1001c1f0(void *this,int *param_1,int param_2)

{
  uint *puVar1;
  uint uVar2;
  uint uVar3;
  uint uVar4;
  
                    // WARNING: Load size is inaccurate
  puVar1 = (uint *)(*this + *param_1 * 0xc);
  if (param_2 == 0) {
    uVar2 = puVar1[1];
    uVar4 = *puVar1;
    uVar3 = uVar2;
    if (uVar2 < uVar4) {
      uVar3 = uVar4;
      uVar4 = uVar2;
    }
  }
  else if (param_2 == 1) {
    uVar3 = puVar1[2];
    uVar4 = puVar1[1];
    if (uVar3 < uVar4) {
      return uVar4 << 8 ^ uVar3;
    }
  }
  else {
    uVar3 = *puVar1;
    uVar4 = puVar1[2];
    if (uVar3 < uVar4) {
      return uVar4 << 8 ^ uVar3;
    }
  }
  return uVar3 << 8 ^ uVar4;
}



int __thiscall FUN_1001c250(void *this,int param_1)

{
  int *piVar1;
  uint *puVar2;
  int iVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  int local_8;
  uint local_4;
  
  piVar1 = (int *)(*(int *)((int)this + 4) + param_1 * 8);
                    // WARNING: Load size is inaccurate
  local_8 = 0;
  local_4 = 0;
  puVar2 = (uint *)(*this + *piVar1 * 0xc);
  do {
    if (local_4 == 0) {
      uVar5 = puVar2[1];
      uVar6 = *puVar2;
      uVar4 = uVar5;
      if (uVar5 < uVar6) goto LAB_1001c2a3;
    }
    else if (local_4 == 1) {
      uVar5 = puVar2[2];
      uVar6 = puVar2[1];
      uVar4 = uVar5;
      if (uVar5 < uVar6) goto LAB_1001c2a3;
    }
    else {
      uVar5 = *puVar2;
      uVar6 = puVar2[2];
      uVar4 = uVar5;
      if (uVar5 < uVar6) {
LAB_1001c2a3:
        uVar5 = uVar6;
        uVar6 = uVar4;
      }
    }
    uVar6 = uVar5 << 8 ^ uVar6;
    iVar7 = *(int *)(*(int *)((int)this + 0xc) +
                    (((uVar6 >> 10 ^ uVar6) >> 10 ^ uVar6) & *(int *)((int)this + 0x18) - 1U) * 4);
    if (iVar7 != -1) {
      do {
        if (*(uint *)(*(int *)((int)this + 0x10) + 4 + iVar7 * 0xc) == uVar6) {
          if (iVar7 != -1) {
            goto LAB_1001c2ed;
          }
          break;
        }
        iVar7 = *(int *)(*(int *)((int)this + 0x10) + iVar7 * 0xc);
      } while (iVar7 != -1);
    }
LAB_1001c328:
    local_4 = local_4 + 1;
    if (2 < local_4) {
      return local_8;
    }
  } while( true );
LAB_1001c2ed:
  if (*(int **)(iVar7 * 0xc + 8 + *(int *)((int)this + 0x10)) != piVar1) {
    local_8 = local_8 + 1;
    goto LAB_1001c328;
  }
  iVar3 = *(int *)((int)this + 0x10);
  iVar7 = *(int *)(iVar3 + iVar7 * 0xc);
  while( true ) {
    if (iVar7 == -1) goto LAB_1001c328;
    if (*(uint *)(iVar3 + 4 + iVar7 * 0xc) == uVar6) break;
    iVar7 = *(int *)(iVar3 + iVar7 * 0xc);
  }
  if (iVar7 == -1) goto LAB_1001c328;
  goto LAB_1001c2ed;
}



void __fastcall FUN_1001c350(int param_1)

{
  FUN_100212a4(*(void **)(param_1 + 4));
  if (*(void **)(param_1 + 0xc) != (void *)0x0) {
    FUN_100212a4(*(void **)(param_1 + 0xc));
  }
  if (*(void **)(param_1 + 0x10) != (void *)0x0) {
    FUN_100212a4(*(void **)(param_1 + 0x10));
  }
  return;
}



void __fastcall FUN_1001c390(int *param_1)

{
  int *piVar1;
  uint *puVar2;
  int iVar3;
  int iVar4;
  int iVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  uint uVar9;
  uint local_10;
  
  piVar1 = param_1 + 3;
  if (param_1[6] != 0) {
    FUN_100212a4((void *)*piVar1);
    FUN_100212a4((void *)param_1[4]);
  }
  param_1[6] = 0;
  *piVar1 = 0;
  param_1[4] = 0;
  param_1[5] = -1;
  FUN_1001d7b0(piVar1);
  local_10 = 0;
  if (param_1[2] != 0) {
    do {
      iVar4 = param_1[1];
      iVar3 = local_10 * 8;
      iVar5 = *(int *)(iVar3 + iVar4);
      uVar6 = *(uint *)(*param_1 + 4 + iVar5 * 0xc);
      puVar2 = (uint *)(*param_1 + iVar5 * 0xc);
      uVar7 = *puVar2;
      uVar9 = uVar7;
      if (uVar6 < uVar7) {
        uVar9 = uVar6;
        uVar6 = uVar7;
      }
      uVar9 = uVar6 << 8 ^ uVar9;
      if (param_1[5] == -1) {
        FUN_1001d7b0(piVar1);
      }
      iVar5 = param_1[5];
      iVar8 = iVar5 * 0xc;
      param_1[5] = *(int *)(iVar8 + param_1[4]);
      uVar6 = ((uVar9 >> 10 ^ uVar9) >> 10 ^ uVar9) & param_1[6] - 1U;
      ((int *)(iVar8 + param_1[4]))[1] = uVar9;
      *(int **)(iVar8 + 8 + param_1[4]) = (int *)(iVar3 + iVar4);
      *(undefined4 *)(iVar8 + param_1[4]) = *(undefined4 *)(*piVar1 + uVar6 * 4);
      *(int *)(*piVar1 + uVar6 * 4) = iVar5;
      iVar4 = param_1[1];
      uVar6 = puVar2[2];
      uVar7 = puVar2[1];
      uVar9 = uVar7;
      if (uVar6 < uVar7) {
        uVar9 = uVar6;
        uVar6 = uVar7;
      }
      uVar9 = uVar6 << 8 ^ uVar9;
      if (param_1[5] == -1) {
        FUN_1001d7b0(piVar1);
      }
      iVar5 = param_1[5];
      iVar8 = iVar5 * 0xc;
      param_1[5] = *(int *)(param_1[4] + iVar8);
      uVar6 = ((uVar9 >> 10 ^ uVar9) >> 10 ^ uVar9) & param_1[6] - 1U;
      ((int *)(param_1[4] + iVar8))[1] = uVar9;
      *(int *)(param_1[4] + 8 + iVar8) = iVar3 + iVar4;
      *(undefined4 *)(iVar8 + param_1[4]) = *(undefined4 *)(*piVar1 + uVar6 * 4);
      *(int *)(*piVar1 + uVar6 * 4) = iVar5;
      uVar6 = *puVar2;
      iVar4 = param_1[1];
      uVar7 = puVar2[2];
      uVar9 = uVar6;
      if (uVar6 < uVar7) {
        uVar9 = uVar7;
        uVar7 = uVar6;
      }
      uVar7 = uVar9 << 8 ^ uVar7;
      if (param_1[5] == -1) {
        FUN_1001d7b0(piVar1);
      }
      iVar5 = param_1[5];
      iVar8 = iVar5 * 0xc;
      param_1[5] = *(int *)(param_1[4] + iVar8);
      uVar6 = ((uVar7 >> 10 ^ uVar7) >> 10 ^ uVar7) & param_1[6] - 1U;
      ((int *)(param_1[4] + iVar8))[1] = uVar7;
      *(int *)(param_1[4] + 8 + iVar8) = iVar3 + iVar4;
      *(undefined4 *)(param_1[4] + iVar8) = *(undefined4 *)(*piVar1 + uVar6 * 4);
      *(int *)(*piVar1 + uVar6 * 4) = iVar5;
      local_10 = local_10 + 1;
    } while (local_10 < (uint)param_1[2]);
  }
  return;
}



void __thiscall FUN_1001c5a0(void *this,undefined4 *param_1,uint *param_2)

{
  undefined4 *puVar1;
  undefined4 local_8;
  undefined4 local_4;
  
  puVar1 = (undefined4 *)FUN_1001c770(this,&local_8,param_2);
  local_4 = CONCAT31(local_4._1_3_,(char)puVar1[1]);
  *param_1 = *puVar1;
  param_1[1] = local_4;
  return;
}



void * __thiscall FUN_1001c5d0(void *this,undefined *param_1,undefined param_2,undefined *param_3)

{
  undefined uVar1;
  
  *(undefined *)this = *param_3;
  uVar1 = *param_1;
  *(undefined *)((int)this + 8) = param_2;
  *(undefined *)((int)this + 1) = uVar1;
  FUN_1001ce60((int)this);
  return this;
}



undefined4 * __thiscall FUN_1001c600(void *this,undefined4 *param_1,int *param_2,int *param_3)

{
  int *piVar1;
  int *piVar2;
  _Lockit local_14 [4];
  undefined4 local_10;
  void *local_c;
  undefined *puStack_8;
  int local_4;
  
  piVar2 = param_3;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10024082;
  local_c = ExceptionList;
  if (((*(int *)((int)this + 0xc) == 0) || (param_2 != (int *)**(int **)((int)this + 4))) ||
     (param_3 != *(int **)((int)this + 4))) {
    ExceptionList = &local_c;
    if (param_2 != param_3) {
      do {
        piVar1 = param_2;
        FUN_1001cf10((int *)&param_2);
        FUN_1001c8a0(this,&local_10,piVar1);
      } while (param_2 != piVar2);
    }
    *param_1 = param_2;
  }
  else {
    ExceptionList = &local_c;
    std::_Lockit::_Lockit(local_14);
    piVar2 = *(int **)(*(int *)((int)this + 4) + 4);
    local_4 = 0;
    std::_Lockit::_Lockit((_Lockit *)&param_3);
    local_4._0_1_ = 1;
    if (piVar2 != DAT_10038510) {
      do {
        FUN_1001cdd0((int *)piVar2[2]);
        piVar1 = (int *)*piVar2;
        FUN_100212a4(piVar2);
        piVar2 = piVar1;
      } while (piVar1 != DAT_10038510);
    }
    local_4 = (uint)local_4._1_3_ << 8;
    std::_Lockit::~_Lockit((_Lockit *)&param_3);
    *(int **)(*(int *)((int)this + 4) + 4) = DAT_10038510;
    *(undefined4 *)((int)this + 0xc) = 0;
    *(undefined4 *)*(undefined4 *)((int)this + 4) = *(undefined4 *)((int)this + 4);
    *(int *)(*(int *)((int)this + 4) + 8) = *(int *)((int)this + 4);
    *param_1 = **(undefined4 **)((int)this + 4);
    local_4 = 0xffffffff;
    std::_Lockit::~_Lockit(local_14);
  }
  ExceptionList = local_c;
  return param_1;
}



void __thiscall FUN_1001c720(void *this,int *param_1,uint *param_2)

{
  undefined4 *puVar1;
  
  puVar1 = FUN_1001d2d0(this,param_2);
  if ((puVar1 != *(undefined4 **)((int)this + 4)) && ((uint)puVar1[3] <= *param_2)) {
    *param_1 = (int)puVar1;
    return;
  }
  *param_1 = (int)*(undefined4 **)((int)this + 4);
  return;
}



void __thiscall FUN_1001c770(void *this,undefined4 *param_1,uint *param_2)

{
  undefined4 *puVar1;
  int *piVar2;
  int *piVar3;
  bool bVar4;
  int *piStack_c;
  _Lockit local_8 [4];
  uint uStack_4;
  
  piVar3 = *(int **)((int)this + 4);
  piVar2 = (int *)piVar3[1];
  bVar4 = true;
  std::_Lockit::_Lockit(local_8);
  if (piVar2 != DAT_10038510) {
    do {
      piVar3 = piVar2;
      bVar4 = *param_2 < (uint)piVar3[3];
      if (bVar4) {
        piVar2 = (int *)*piVar3;
      }
      else {
        piVar2 = (int *)piVar3[2];
      }
    } while (piVar2 != DAT_10038510);
  }
  std::_Lockit::~_Lockit(local_8);
  if (*(char *)((int)this + 8) != '\0') {
    puVar1 = FUN_1001cfc0(this,&param_2,(int)piVar2,piVar3,param_2);
    *param_1 = *puVar1;
    uStack_4 = CONCAT31(uStack_4._1_3_,1);
    param_1[1] = uStack_4;
    return;
  }
  piStack_c = piVar3;
  if (bVar4) {
    if (piVar3 == (int *)**(int **)((int)this + 4)) {
      puVar1 = FUN_1001cfc0(this,&param_2,(int)piVar2,piVar3,param_2);
      uStack_4 = CONCAT31(uStack_4._1_3_,1);
      *param_1 = *puVar1;
      param_1[1] = uStack_4;
      return;
    }
    FUN_1001d320((int *)&piStack_c);
  }
  if ((uint)piStack_c[3] < *param_2) {
    puVar1 = FUN_1001cfc0(this,&param_2,(int)piVar2,piVar3,param_2);
    *param_1 = *puVar1;
    uStack_4 = CONCAT31(uStack_4._1_3_,1);
    param_1[1] = uStack_4;
    return;
  }
  uStack_4 = uStack_4 & 0xffffff00;
  *param_1 = piStack_c;
  param_1[1] = uStack_4;
  return;
}



undefined4 * __thiscall FUN_1001c8a0(void *this,undefined4 *param_1,int *param_2)

{
  int iVar1;
  int *piVar2;
  int *piVar3;
  int *piVar4;
  int *piVar5;
  int *piVar6;
  int *local_1c;
  _Lockit local_18 [4];
  _Lockit local_14 [4];
  _Lockit local_10 [4];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  piVar6 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10024099;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_1001cf10((int *)&param_2);
  local_1c = piVar6;
  std::_Lockit::_Lockit(local_10);
  piVar5 = (int *)*piVar6;
  local_4 = 0;
  if (piVar5 == DAT_10038510) {
    piVar5 = (int *)piVar6[2];
  }
  else {
    piVar2 = (int *)piVar6[2];
    if (piVar2 != DAT_10038510) {
      std::_Lockit::_Lockit(local_18);
      local_1c = piVar2;
      for (piVar5 = (int *)*piVar2; piVar5 != DAT_10038510; piVar5 = (int *)*piVar5) {
        local_1c = piVar5;
      }
      std::_Lockit::~_Lockit(local_18);
      piVar5 = (int *)local_1c[2];
      if (local_1c != piVar6) {
        *(int **)(*piVar6 + 4) = local_1c;
        *local_1c = *piVar6;
        if (local_1c == (int *)piVar6[2]) {
          piVar5[1] = (int)local_1c;
        }
        else {
          piVar5[1] = local_1c[1];
          *(int **)local_1c[1] = piVar5;
          local_1c[2] = piVar6[2];
          *(int **)(piVar6[2] + 4) = local_1c;
        }
        if (*(int **)(*(int *)((int)this + 4) + 4) == piVar6) {
          *(int **)(*(int *)((int)this + 4) + 4) = local_1c;
        }
        else {
          piVar2 = (int *)piVar6[1];
          if ((int *)*piVar2 == piVar6) {
            *piVar2 = (int)local_1c;
          }
          else {
            piVar2[2] = (int)local_1c;
          }
        }
        local_1c[1] = piVar6[1];
        iVar1 = local_1c[5];
        local_1c[5] = piVar6[5];
        piVar6[5] = iVar1;
        local_1c = piVar6;
        goto LAB_1001ca55;
      }
    }
  }
  piVar5[1] = local_1c[1];
  if (*(int **)(*(int *)((int)this + 4) + 4) == piVar6) {
    *(int **)(*(int *)((int)this + 4) + 4) = piVar5;
  }
  else {
    piVar2 = (int *)piVar6[1];
    if ((int *)*piVar2 == piVar6) {
      *piVar2 = (int)piVar5;
    }
    else {
      piVar2[2] = (int)piVar5;
    }
  }
  if ((int *)**(int **)((int)this + 4) == piVar6) {
    if ((int *)piVar6[2] == DAT_10038510) {
      **(int **)((int)this + 4) = piVar6[1];
    }
    else {
      std::_Lockit::_Lockit(local_14);
      piVar3 = (int *)*piVar5;
      piVar2 = piVar5;
      while (piVar4 = piVar3, piVar4 != DAT_10038510) {
        piVar2 = piVar4;
        piVar3 = (int *)*piVar4;
      }
      std::_Lockit::~_Lockit(local_14);
      **(undefined4 **)((int)this + 4) = piVar2;
    }
  }
  if (*(int **)(*(int *)((int)this + 4) + 8) == piVar6) {
    if ((int *)*piVar6 == DAT_10038510) {
      *(int *)(*(int *)((int)this + 4) + 8) = piVar6[1];
    }
    else {
      std::_Lockit::_Lockit(local_14);
      piVar2 = (int *)piVar5[2];
      piVar6 = piVar5;
      while (piVar3 = piVar2, piVar3 != DAT_10038510) {
        piVar6 = piVar3;
        piVar2 = (int *)piVar3[2];
      }
      std::_Lockit::~_Lockit(local_14);
      *(int **)(*(int *)((int)this + 4) + 8) = piVar6;
    }
  }
LAB_1001ca55:
  if (local_1c[5] == 1) {
    if (piVar5 != *(int **)(*(int *)((int)this + 4) + 4)) {
      do {
        if (piVar5[5] != 1) break;
        piVar6 = (int *)piVar5[1];
        if (piVar5 == (int *)*piVar6) {
          piVar6 = (int *)piVar6[2];
          if (piVar6[5] == 0) {
            piVar6[5] = 1;
            *(undefined4 *)(piVar5[1] + 0x14) = 0;
            iVar1 = piVar5[1];
            std::_Lockit::_Lockit(local_14);
            piVar6 = *(int **)(iVar1 + 8);
            *(int *)(iVar1 + 8) = *piVar6;
            if ((int *)*piVar6 != DAT_10038510) {
              ((int *)*piVar6)[1] = iVar1;
            }
            piVar6[1] = *(int *)(iVar1 + 4);
            if (iVar1 == *(int *)(*(int *)((int)this + 4) + 4)) {
              *(int **)(*(int *)((int)this + 4) + 4) = piVar6;
            }
            else {
              piVar2 = *(int **)(iVar1 + 4);
              if (iVar1 == *piVar2) {
                *piVar2 = (int)piVar6;
              }
              else {
                piVar2[2] = (int)piVar6;
              }
            }
            *piVar6 = iVar1;
            *(int **)(iVar1 + 4) = piVar6;
            std::_Lockit::~_Lockit(local_14);
            piVar6 = *(int **)(piVar5[1] + 8);
          }
          if ((*(int *)(*piVar6 + 0x14) != 1) || (*(int *)(piVar6[2] + 0x14) != 1)) {
            if (*(int *)(piVar6[2] + 0x14) == 1) {
              *(undefined4 *)(*piVar6 + 0x14) = 1;
              piVar6[5] = 0;
              std::_Lockit::_Lockit(local_14);
              iVar1 = *piVar6;
              *piVar6 = *(int *)(iVar1 + 8);
              if (*(int **)(iVar1 + 8) != DAT_10038510) {
                (*(int **)(iVar1 + 8))[1] = (int)piVar6;
              }
              *(int *)(iVar1 + 4) = piVar6[1];
              if (piVar6 == *(int **)(*(int *)((int)this + 4) + 4)) {
                *(int *)(*(int *)((int)this + 4) + 4) = iVar1;
              }
              else {
                piVar2 = (int *)piVar6[1];
                if (piVar6 == (int *)piVar2[2]) {
                  piVar2[2] = iVar1;
                }
                else {
                  *piVar2 = iVar1;
                }
              }
              *(int **)(iVar1 + 8) = piVar6;
              piVar6[1] = iVar1;
              std::_Lockit::~_Lockit(local_14);
              piVar6 = *(int **)(piVar5[1] + 8);
            }
            piVar6[5] = *(int *)(piVar5[1] + 0x14);
            *(undefined4 *)(piVar5[1] + 0x14) = 1;
            *(undefined4 *)(piVar6[2] + 0x14) = 1;
            iVar1 = piVar5[1];
            std::_Lockit::_Lockit(local_14);
            piVar6 = *(int **)(iVar1 + 8);
            *(int *)(iVar1 + 8) = *piVar6;
            if ((int *)*piVar6 != DAT_10038510) {
              ((int *)*piVar6)[1] = iVar1;
            }
            piVar6[1] = *(int *)(iVar1 + 4);
            if (iVar1 == *(int *)(*(int *)((int)this + 4) + 4)) {
              *(int **)(*(int *)((int)this + 4) + 4) = piVar6;
              *piVar6 = iVar1;
              *(int **)(iVar1 + 4) = piVar6;
            }
            else {
              piVar2 = *(int **)(iVar1 + 4);
              if (iVar1 == *piVar2) {
                *piVar2 = (int)piVar6;
                *piVar6 = iVar1;
                *(int **)(iVar1 + 4) = piVar6;
              }
              else {
                piVar2[2] = (int)piVar6;
                *piVar6 = iVar1;
                *(int **)(iVar1 + 4) = piVar6;
              }
            }
LAB_1001cd70:
            std::_Lockit::~_Lockit(local_14);
            break;
          }
        }
        else {
          piVar6 = (int *)*piVar6;
          if (piVar6[5] == 0) {
            piVar6[5] = 1;
            *(undefined4 *)(piVar5[1] + 0x14) = 0;
            piVar6 = (int *)piVar5[1];
            std::_Lockit::_Lockit(local_18);
            iVar1 = *piVar6;
            *piVar6 = *(int *)(iVar1 + 8);
            if (*(int **)(iVar1 + 8) != DAT_10038510) {
              (*(int **)(iVar1 + 8))[1] = (int)piVar6;
            }
            *(int *)(iVar1 + 4) = piVar6[1];
            if (piVar6 == *(int **)(*(int *)((int)this + 4) + 4)) {
              *(int *)(*(int *)((int)this + 4) + 4) = iVar1;
            }
            else {
              piVar2 = (int *)piVar6[1];
              if (piVar6 == (int *)piVar2[2]) {
                piVar2[2] = iVar1;
              }
              else {
                *piVar2 = iVar1;
              }
            }
            *(int **)(iVar1 + 8) = piVar6;
            piVar6[1] = iVar1;
            std::_Lockit::~_Lockit(local_18);
            piVar6 = *(int **)piVar5[1];
          }
          if ((*(int *)(piVar6[2] + 0x14) != 1) || (*(int *)(*piVar6 + 0x14) != 1)) {
            if (*(int *)(*piVar6 + 0x14) == 1) {
              *(undefined4 *)(piVar6[2] + 0x14) = 1;
              piVar6[5] = 0;
              std::_Lockit::_Lockit(local_14);
              piVar2 = (int *)piVar6[2];
              piVar6[2] = *piVar2;
              if ((int *)*piVar2 != DAT_10038510) {
                ((int *)*piVar2)[1] = (int)piVar6;
              }
              piVar2[1] = piVar6[1];
              if (piVar6 == *(int **)(*(int *)((int)this + 4) + 4)) {
                *(int **)(*(int *)((int)this + 4) + 4) = piVar2;
              }
              else {
                piVar3 = (int *)piVar6[1];
                if (piVar6 == (int *)*piVar3) {
                  *piVar3 = (int)piVar2;
                }
                else {
                  piVar3[2] = (int)piVar2;
                }
              }
              *piVar2 = (int)piVar6;
              piVar6[1] = (int)piVar2;
              std::_Lockit::~_Lockit(local_14);
              piVar6 = *(int **)piVar5[1];
            }
            piVar6[5] = *(int *)(piVar5[1] + 0x14);
            *(undefined4 *)(piVar5[1] + 0x14) = 1;
            *(undefined4 *)(*piVar6 + 0x14) = 1;
            piVar6 = (int *)piVar5[1];
            std::_Lockit::_Lockit(local_14);
            iVar1 = *piVar6;
            *piVar6 = *(int *)(iVar1 + 8);
            if (*(int **)(iVar1 + 8) != DAT_10038510) {
              (*(int **)(iVar1 + 8))[1] = (int)piVar6;
            }
            *(int *)(iVar1 + 4) = piVar6[1];
            if (piVar6 == *(int **)(*(int *)((int)this + 4) + 4)) {
              *(int *)(*(int *)((int)this + 4) + 4) = iVar1;
            }
            else {
              piVar2 = (int *)piVar6[1];
              if (piVar6 == (int *)piVar2[2]) {
                piVar2[2] = iVar1;
              }
              else {
                *piVar2 = iVar1;
              }
            }
            *(int **)(iVar1 + 8) = piVar6;
            piVar6[1] = iVar1;
            goto LAB_1001cd70;
          }
        }
        piVar6[5] = 0;
        piVar5 = (int *)piVar5[1];
      } while (piVar5 != *(int **)(*(int *)((int)this + 4) + 4));
    }
    piVar5[5] = 1;
  }
  FUN_100212a4(local_1c);
  *(int *)((int)this + 0xc) = *(int *)((int)this + 0xc) + -1;
  *param_1 = param_2;
  local_4 = 0xffffffff;
  std::_Lockit::~_Lockit(local_10);
  ExceptionList = pvStack_c;
  return param_1;
}



void FUN_1001cdd0(int *param_1)

{
  int *piVar1;
  _Lockit local_10 [4];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_100240b9;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  std::_Lockit::_Lockit(local_10);
  uStack_4 = 0;
  if (param_1 != DAT_10038510) {
    do {
      FUN_1001cdd0((int *)param_1[2]);
      piVar1 = (int *)*param_1;
      FUN_100212a4(param_1);
      param_1 = piVar1;
    } while (piVar1 != DAT_10038510);
  }
  uStack_4 = 0xffffffff;
  std::_Lockit::~_Lockit(local_10);
  ExceptionList = pvStack_c;
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __fastcall FUN_1001ce60(int param_1)

{
  undefined4 *puVar1;
  void *pvVar2;
  int local_10;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_100240d9;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  local_10 = param_1;
  std::_Lockit::_Lockit((_Lockit *)&local_10);
  uStack_4 = 0;
  if (DAT_10038510 == (undefined4 *)0x0) {
    DAT_10038510 = (undefined4 *)operator_new(0x18);
    DAT_10038510[1] = 0;
    DAT_10038510[5] = 1;
    *DAT_10038510 = 0;
    DAT_10038510[2] = 0;
  }
  puVar1 = DAT_10038510;
  _DAT_10038514 = _DAT_10038514 + 1;
  pvVar2 = operator_new(0x18);
  *(undefined4 **)((int)pvVar2 + 4) = puVar1;
  *(undefined4 *)((int)pvVar2 + 0x14) = 0;
  *(void **)(param_1 + 4) = pvVar2;
  *(undefined4 *)(param_1 + 0xc) = 0;
  *(void **)pvVar2 = pvVar2;
  *(int *)(*(int *)(param_1 + 4) + 8) = *(int *)(param_1 + 4);
  uStack_4 = 0xffffffff;
  std::_Lockit::~_Lockit((_Lockit *)&local_10);
  ExceptionList = pvStack_c;
  return;
}



void __fastcall FUN_1001cf10(int *param_1)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  int iVar3;
  _Lockit local_14 [4];
  _Lockit local_10 [4];
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_100240f9;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  std::_Lockit::_Lockit(local_10);
  puVar1 = *(undefined4 **)(*param_1 + 8);
  local_4 = 0;
  if (puVar1 == DAT_10038510) {
    iVar3 = *(int *)(*param_1 + 4);
    if (*param_1 == *(int *)(iVar3 + 8)) {
      do {
        *param_1 = iVar3;
        iVar3 = *(int *)(iVar3 + 4);
      } while (*param_1 == *(int *)(iVar3 + 8));
    }
    if (*(int *)(*param_1 + 8) != iVar3) {
      *param_1 = iVar3;
    }
  }
  else {
    std::_Lockit::_Lockit(local_14);
    for (puVar2 = (undefined4 *)*puVar1; puVar2 != DAT_10038510; puVar2 = (undefined4 *)*puVar2) {
      puVar1 = puVar2;
    }
    std::_Lockit::~_Lockit(local_14);
    *param_1 = (int)puVar1;
  }
  local_4 = 0xffffffff;
  std::_Lockit::~_Lockit(local_10);
  ExceptionList = local_c;
  return;
}



undefined4 * __thiscall
FUN_1001cfc0(void *this,undefined4 *param_1,int param_2,int *param_3,uint *param_4)

{
  int *piVar1;
  int *piVar2;
  int iVar3;
  uint *puVar4;
  int *piVar5;
  _Lockit *this_00;
  int *piVar6;
  _Lockit a_Stack_14 [4];
  _Lockit local_10 [4];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10024119;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  std::_Lockit::_Lockit(local_10);
  uStack_4 = 0;
  piVar5 = (int *)operator_new(0x18);
  puVar4 = param_4;
  piVar6 = param_3;
  piVar5[5] = 0;
  piVar5[1] = (int)param_3;
  *piVar5 = DAT_10038510;
  piVar5[2] = DAT_10038510;
  FUN_1001d790(piVar5 + 3,param_4);
  *(int *)((int)this + 0xc) = *(int *)((int)this + 0xc) + 1;
  if (((piVar6 == *(int **)((int)this + 4)) || (param_2 != DAT_10038510)) ||
     (*puVar4 < (uint)piVar6[3])) {
    *piVar6 = (int)piVar5;
    piVar1 = *(int **)((int)this + 4);
    if (piVar6 == piVar1) {
      piVar1[1] = (int)piVar5;
      *(int **)(*(int *)((int)this + 4) + 8) = piVar5;
    }
    else if (piVar6 == (int *)*piVar1) {
      *piVar1 = (int)piVar5;
    }
  }
  else {
    piVar6[2] = (int)piVar5;
    if (piVar6 == *(int **)(*(int *)((int)this + 4) + 8)) {
      *(int **)(*(int *)((int)this + 4) + 8) = piVar5;
    }
  }
  piVar6 = piVar5;
  if (piVar5 != *(int **)(*(int *)((int)this + 4) + 4)) {
    do {
      piVar1 = (int *)piVar6[1];
      if (piVar1[5] != 0) break;
      piVar2 = (int *)piVar1[1];
      if (piVar1 == (int *)*piVar2) {
        iVar3 = piVar2[2];
        if (*(int *)(iVar3 + 0x14) == 0) {
          piVar1[5] = 1;
          *(undefined4 *)(iVar3 + 0x14) = 1;
          *(undefined4 *)(*(int *)(piVar6[1] + 4) + 0x14) = 0;
          piVar6 = *(int **)(piVar6[1] + 4);
        }
        else {
          if (piVar6 == (int *)piVar1[2]) {
            std::_Lockit::_Lockit((_Lockit *)&param_3);
            piVar6 = (int *)piVar1[2];
            piVar1[2] = *piVar6;
            if (*piVar6 != DAT_10038510) {
              *(int **)(*piVar6 + 4) = piVar1;
            }
            piVar6[1] = piVar1[1];
            if (piVar1 == *(int **)(*(int *)((int)this + 4) + 4)) {
              *(int **)(*(int *)((int)this + 4) + 4) = piVar6;
            }
            else {
              piVar2 = (int *)piVar1[1];
              if (piVar1 == (int *)*piVar2) {
                *piVar2 = (int)piVar6;
              }
              else {
                piVar2[2] = (int)piVar6;
              }
            }
            *piVar6 = (int)piVar1;
            piVar1[1] = (int)piVar6;
            std::_Lockit::~_Lockit((_Lockit *)&param_3);
            piVar6 = piVar1;
          }
          *(undefined4 *)(piVar6[1] + 0x14) = 1;
          *(undefined4 *)(*(int *)(piVar6[1] + 4) + 0x14) = 0;
          piVar1 = *(int **)(piVar6[1] + 4);
          std::_Lockit::_Lockit((_Lockit *)&param_4);
          iVar3 = *piVar1;
          *piVar1 = *(int *)(iVar3 + 8);
          if (*(int *)(iVar3 + 8) != DAT_10038510) {
            *(int **)(*(int *)(iVar3 + 8) + 4) = piVar1;
          }
          *(int *)(iVar3 + 4) = piVar1[1];
          if (piVar1 == *(int **)(*(int *)((int)this + 4) + 4)) {
            *(int *)(*(int *)((int)this + 4) + 4) = iVar3;
            *(int **)(iVar3 + 8) = piVar1;
            piVar1[1] = iVar3;
            this_00 = (_Lockit *)&param_4;
          }
          else {
            piVar2 = (int *)piVar1[1];
            if (piVar1 == (int *)piVar2[2]) {
              piVar2[2] = iVar3;
              *(int **)(iVar3 + 8) = piVar1;
              piVar1[1] = iVar3;
              this_00 = (_Lockit *)&param_4;
            }
            else {
              *piVar2 = iVar3;
              *(int **)(iVar3 + 8) = piVar1;
              piVar1[1] = iVar3;
              this_00 = (_Lockit *)&param_4;
            }
          }
LAB_1001d276:
          std::_Lockit::~_Lockit(this_00);
        }
      }
      else {
        iVar3 = *piVar2;
        if (*(int *)(iVar3 + 0x14) != 0) {
          if (piVar6 == (int *)*piVar1) {
            std::_Lockit::_Lockit((_Lockit *)&param_2);
            iVar3 = *piVar1;
            *piVar1 = *(int *)(iVar3 + 8);
            if (*(int *)(iVar3 + 8) != DAT_10038510) {
              *(int **)(*(int *)(iVar3 + 8) + 4) = piVar1;
            }
            *(int *)(iVar3 + 4) = piVar1[1];
            if (piVar1 == *(int **)(*(int *)((int)this + 4) + 4)) {
              *(int *)(*(int *)((int)this + 4) + 4) = iVar3;
            }
            else {
              piVar6 = (int *)piVar1[1];
              if (piVar1 == (int *)piVar6[2]) {
                piVar6[2] = iVar3;
              }
              else {
                *piVar6 = iVar3;
              }
            }
            *(int **)(iVar3 + 8) = piVar1;
            piVar1[1] = iVar3;
            std::_Lockit::~_Lockit((_Lockit *)&param_2);
            piVar6 = piVar1;
          }
          *(undefined4 *)(piVar6[1] + 0x14) = 1;
          *(undefined4 *)(*(int *)(piVar6[1] + 4) + 0x14) = 0;
          iVar3 = *(int *)(piVar6[1] + 4);
          std::_Lockit::_Lockit(a_Stack_14);
          piVar1 = *(int **)(iVar3 + 8);
          *(int *)(iVar3 + 8) = *piVar1;
          if (*piVar1 != DAT_10038510) {
            *(int *)(*piVar1 + 4) = iVar3;
          }
          piVar1[1] = *(int *)(iVar3 + 4);
          if (iVar3 == *(int *)(*(int *)((int)this + 4) + 4)) {
            *(int **)(*(int *)((int)this + 4) + 4) = piVar1;
          }
          else {
            piVar2 = *(int **)(iVar3 + 4);
            if (iVar3 == *piVar2) {
              *piVar2 = (int)piVar1;
            }
            else {
              piVar2[2] = (int)piVar1;
            }
          }
          *piVar1 = iVar3;
          *(int **)(iVar3 + 4) = piVar1;
          this_00 = a_Stack_14;
          goto LAB_1001d276;
        }
        piVar1[5] = 1;
        *(undefined4 *)(iVar3 + 0x14) = 1;
        *(undefined4 *)(*(int *)(piVar6[1] + 4) + 0x14) = 0;
        piVar6 = *(int **)(piVar6[1] + 4);
      }
    } while (piVar6 != *(int **)(*(int *)((int)this + 4) + 4));
  }
  *(undefined4 *)(*(int *)(*(int *)((int)this + 4) + 4) + 0x14) = 1;
  *param_1 = piVar5;
  uStack_4 = 0xffffffff;
  std::_Lockit::~_Lockit(local_10);
  ExceptionList = pvStack_c;
  return param_1;
}



undefined4 * __thiscall FUN_1001d2d0(void *this,uint *param_1)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  void *local_4;
  
  local_4 = this;
  std::_Lockit::_Lockit((_Lockit *)&local_4);
  puVar3 = *(undefined4 **)((int)this + 4);
  if ((undefined4 *)puVar3[1] != DAT_10038510) {
    puVar1 = (undefined4 *)puVar3[1];
    do {
      if ((uint)puVar1[3] < *param_1) {
        puVar2 = (undefined4 *)puVar1[2];
      }
      else {
        puVar2 = (undefined4 *)*puVar1;
        puVar3 = puVar1;
      }
      puVar1 = puVar2;
    } while (puVar2 != DAT_10038510);
  }
  std::_Lockit::~_Lockit((_Lockit *)&local_4);
  return puVar3;
}



void __fastcall FUN_1001d320(int *param_1)

{
  int iVar1;
  int iVar2;
  int *piVar3;
  _Lockit local_14 [4];
  _Lockit local_10 [4];
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10024139;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  std::_Lockit::_Lockit(local_10);
  piVar3 = (int *)*param_1;
  local_4 = 0;
  if ((piVar3[5] == 0) && (*(int **)(piVar3[1] + 4) == piVar3)) {
    *param_1 = piVar3[2];
  }
  else {
    iVar1 = *piVar3;
    if (iVar1 == DAT_10038510) {
      piVar3 = (int *)piVar3[1];
      if (*param_1 == *piVar3) {
        do {
          *param_1 = (int)piVar3;
          piVar3 = (int *)piVar3[1];
        } while (*param_1 == *piVar3);
      }
      *param_1 = (int)piVar3;
    }
    else {
      std::_Lockit::_Lockit(local_14);
      for (iVar2 = *(int *)(iVar1 + 8); iVar2 != DAT_10038510; iVar2 = *(int *)(iVar2 + 8)) {
        iVar1 = iVar2;
      }
      std::_Lockit::~_Lockit(local_14);
      *param_1 = iVar1;
    }
  }
  local_4 = 0xffffffff;
  std::_Lockit::~_Lockit(local_10);
  ExceptionList = local_c;
  return;
}



float10 __cdecl FUN_1001d3e0(float *param_1,float *param_2)

{
  return (float10)*param_1 * (float10)*param_2 +
         (float10)param_1[1] * (float10)param_2[1] + (float10)param_1[2] * (float10)param_2[2];
}



void __fastcall FUN_1001d400(undefined4 *param_1)

{
  if ((void *)*param_1 != (void *)0x0) {
    FUN_100212a4((void *)*param_1);
  }
  if ((void *)param_1[1] != (void *)0x0) {
    FUN_100212a4((void *)param_1[1]);
  }
  return;
}



int __thiscall FUN_1001d430(void *this,uint *param_1,int param_2)

{
  uint uVar1;
  int iVar2;
  int iVar3;
  
  if (param_2 == -1) {
    uVar1 = *param_1;
                    // WARNING: Load size is inaccurate
    iVar2 = *(int *)(*this + (((uVar1 >> 10 ^ uVar1) >> 10 ^ uVar1) & *(int *)((int)this + 0xc) - 1U
                             ) * 4);
    if (iVar2 != -1) {
      do {
        if (*(uint *)(*(int *)((int)this + 4) + 4 + iVar2 * 0xc) == uVar1) {
          return iVar2;
        }
        iVar2 = *(int *)(*(int *)((int)this + 4) + iVar2 * 0xc);
      } while (iVar2 != -1);
      return -1;
    }
  }
  else {
    iVar2 = *(int *)((int)this + 4);
    iVar3 = *(int *)(iVar2 + param_2 * 0xc);
    if (iVar3 != -1) {
      do {
        if (*(uint *)(iVar2 + 4 + iVar3 * 0xc) == *param_1) {
          return iVar3;
        }
        iVar3 = *(int *)(iVar2 + iVar3 * 0xc);
      } while (iVar3 != -1);
    }
  }
  return -1;
}



void __cdecl FUN_1001d4b0(undefined4 *param_1,uint *param_2,int param_3)

{
  undefined4 uVar1;
  undefined4 uVar2;
  uint uVar3;
  uint uVar4;
  uint *puVar5;
  undefined4 *puVar6;
  uint *puVar7;
  int iVar8;
  undefined4 *puVar9;
  uint *puVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  uint *puVar14;
  int local_c;
  
  puVar7 = param_2;
  if (1 < param_3) {
    iVar11 = 0;
    local_c = 0;
    if (1 < param_3) {
      iVar8 = param_3 + -1;
      puVar10 = param_2;
      do {
        if (*puVar10 <= puVar10[1]) {
          iVar11 = iVar11 + 1;
        }
        iVar8 = iVar8 + -1;
        puVar10 = puVar10 + 1;
        local_c = iVar11;
      } while (iVar8 != 0);
    }
    if (local_c + 1 != param_3) {
      if (local_c < param_3 / 3) {
        if (0 < param_3 / 2) {
          puVar10 = param_2 + param_3 + -1;
          puVar6 = param_1 + param_3 * 2 + -2;
          puVar9 = param_1;
          puVar14 = param_2;
          param_2 = (uint *)(param_3 / 2);
          do {
            uVar1 = puVar9[1];
            uVar2 = *puVar9;
            *puVar9 = *puVar6;
            puVar9[1] = puVar6[1];
            *puVar6 = uVar2;
            puVar6[1] = uVar1;
            uVar3 = *puVar14;
            *puVar14 = *puVar10;
            *puVar10 = uVar3;
            puVar6 = puVar6 + -2;
            puVar10 = puVar10 + -1;
            puVar9 = puVar9 + 2;
            puVar14 = puVar14 + 1;
            param_2 = (uint *)((int)param_2 + -1);
          } while (param_2 != (uint *)0x0);
        }
        if (local_c == 0) {
          return;
        }
      }
      else if (0x31 < local_c) {
        iVar11 = param_3 + -1;
        if (8 < iVar11) {
          uVar3 = param_2[iVar11];
          iVar12 = -1;
          iVar8 = iVar11;
          do {
            puVar7 = param_2 + iVar12;
            do {
              iVar13 = iVar12;
              iVar12 = iVar13 + 1;
              puVar7 = puVar7 + 1;
              if (iVar11 <= iVar12) break;
            } while (*puVar7 < uVar3);
            puVar7 = param_2 + iVar8;
            do {
              iVar8 = iVar8 + -1;
              puVar7 = puVar7 + -1;
              if (iVar8 < 1) break;
            } while (uVar3 < *puVar7);
            uVar1 = param_1[iVar12 * 2 + 1];
            uVar2 = param_1[iVar12 * 2];
            param_1[iVar12 * 2] = param_1[iVar8 * 2];
            param_1[iVar12 * 2 + 1] = param_1[iVar8 * 2 + 1];
            param_1[iVar8 * 2] = uVar2;
            param_1[iVar8 * 2 + 1] = uVar1;
            uVar4 = param_2[iVar12];
            param_2[iVar12] = param_2[iVar8];
            param_2[iVar8] = uVar4;
            if (iVar8 <= iVar12) {
              param_1[iVar8 * 2] = param_1[iVar12 * 2];
              param_1[iVar8 * 2 + 1] = param_1[iVar12 * 2 + 1];
              param_1[iVar12 * 2] = param_1[iVar11 * 2];
              param_1[iVar12 * 2 + 1] = param_1[iVar11 * 2 + 1];
              param_1[iVar11 * 2] = uVar2;
              param_1[iVar11 * 2 + 1] = uVar1;
              param_2[iVar8] = param_2[iVar12];
              param_2[iVar12] = param_2[iVar11];
              param_2[iVar11] = uVar4;
              if (0 < iVar13) {
                FUN_1001d900((int)param_1,(int)param_2,0,iVar13);
              }
              if (iVar11 <= iVar13 + 2) {
                return;
              }
              FUN_1001d900((int)param_1,(int)param_2,iVar13 + 2,iVar11);
              return;
            }
          } while( true );
        }
        param_2 = (uint *)0x1;
        puVar10 = puVar7;
        puVar6 = param_1;
        if (param_3 < 2) {
          return;
        }
        do {
          puVar9 = puVar6 + 2;
          uVar1 = puVar6[3];
          uVar3 = puVar10[1];
          uVar2 = *puVar9;
          uVar4 = *puVar10;
          puVar14 = puVar10;
          puVar6 = puVar9;
          puVar5 = param_2;
          while (uVar3 < uVar4) {
            puVar14[1] = *puVar14;
            *puVar6 = puVar6[-2];
            puVar6[1] = puVar6[-1];
            puVar5 = (uint *)((int)puVar5 + -1);
            puVar6 = puVar6 + -2;
            puVar14 = puVar14 + -1;
            if (puVar5 == (uint *)0x0) break;
            uVar4 = *puVar14;
          }
          puVar7[(int)puVar5] = uVar3;
          param_1[(int)puVar5 * 2] = uVar2;
          param_1[(int)puVar5 * 2 + 1] = uVar1;
          param_2 = (uint *)((int)param_2 + 1);
          puVar10 = puVar10 + 1;
          puVar6 = puVar9;
          if (param_3 <= (int)param_2) {
            return;
          }
        } while( true );
      }
      local_c = 1;
      puVar10 = puVar7;
      puVar6 = param_1;
      if (1 < param_3) {
        do {
          puVar9 = puVar6 + 2;
          uVar1 = *puVar9;
          uVar3 = puVar10[1];
          uVar2 = puVar6[3];
          uVar4 = *puVar10;
          puVar6 = puVar9;
          puVar14 = puVar10;
          iVar11 = local_c;
          while (uVar3 < uVar4) {
            puVar14[1] = *puVar14;
            *puVar6 = puVar6[-2];
            puVar6[1] = puVar6[-1];
            iVar11 = iVar11 + -1;
            puVar6 = puVar6 + -2;
            puVar14 = puVar14 + -1;
            if (iVar11 == 0) break;
            uVar4 = *puVar14;
          }
          puVar7[iVar11] = uVar3;
          param_1[iVar11 * 2] = uVar1;
          param_1[iVar11 * 2 + 1] = uVar2;
          local_c = local_c + 1;
          puVar10 = puVar10 + 1;
          puVar6 = puVar9;
          if (param_3 <= local_c) {
            return;
          }
        } while( true );
      }
    }
  }
  return;
}



void __cdecl FUN_1001d790(undefined4 *param_1,undefined4 *param_2)

{
  if (param_1 != (undefined4 *)0x0) {
    *param_1 = *param_2;
    param_1[1] = param_2[1];
  }
  return;
}



void __fastcall FUN_1001d7b0(int *param_1)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  int iVar4;
  undefined4 *puVar5;
  int iVar6;
  int *piVar7;
  uint uVar8;
  int iVar9;
  uint local_14;
  int local_10;
  
  local_14 = param_1[3] << 1;
  if (local_14 < 4) {
    local_14 = 4;
  }
  puVar1 = (undefined4 *)operator_new(local_14 * 0xc);
  puVar2 = (undefined4 *)operator_new(local_14 * 4);
  iVar9 = 0;
  puVar3 = puVar1;
  puVar5 = puVar2;
  uVar8 = local_14;
  if (0 < (int)local_14) {
    do {
      *puVar3 = 0xffffffff;
      *puVar5 = 0xffffffff;
      uVar8 = uVar8 - 1;
      puVar3 = puVar3 + 3;
      puVar5 = puVar5 + 1;
    } while (uVar8 != 0);
  }
  if (param_1[3] != 0) {
    local_10 = 0;
    if (0 < param_1[3]) {
      do {
        iVar6 = *(int *)(*param_1 + local_10 * 4);
        if (iVar6 != -1) {
          iVar4 = param_1[1];
          puVar3 = puVar1 + iVar9 * 3 + 2;
          do {
            iVar6 = iVar6 * 0xc;
            uVar8 = *(uint *)(iVar4 + 4 + iVar6);
            puVar3[-1] = uVar8;
            uVar8 = ((uVar8 >> 10 ^ uVar8) >> 10 ^ uVar8) & local_14 - 1;
            *puVar3 = *(undefined4 *)(param_1[1] + 8 + iVar6);
            puVar3[-2] = puVar2[uVar8];
            puVar2[uVar8] = iVar9;
            iVar4 = param_1[1];
            iVar6 = *(int *)(iVar4 + iVar6);
            iVar9 = iVar9 + 1;
            puVar3 = puVar3 + 3;
          } while (iVar6 != -1);
        }
        local_10 = local_10 + 1;
      } while (local_10 < param_1[3]);
    }
    FUN_100212a4((void *)*param_1);
    FUN_100212a4((void *)param_1[1]);
  }
  if (iVar9 < (int)local_14) {
    piVar7 = puVar1 + iVar9 * 3;
    iVar6 = iVar9;
    do {
      iVar6 = iVar6 + 1;
      *piVar7 = iVar6;
      piVar7 = piVar7 + 3;
    } while (iVar6 < (int)local_14);
  }
  puVar1[local_14 * 3 + -3] = 0xffffffff;
  param_1[2] = iVar9;
  *param_1 = (int)puVar2;
  param_1[3] = local_14;
  param_1[1] = (int)puVar1;
  return;
}



void __cdecl FUN_1001d900(int param_1,int param_2,int param_3,int param_4)

{
  uint uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  uint uVar5;
  int iVar6;
  uint *puVar7;
  undefined4 *puVar8;
  undefined4 *puVar9;
  uint *puVar10;
  int iVar11;
  int iVar12;
  
  iVar11 = -param_3;
  while (8 < param_4 + iVar11) {
    uVar1 = *(uint *)(param_2 + param_4 * 4);
    iVar11 = param_3 + -1;
    iVar6 = param_4;
    do {
      puVar7 = (uint *)(param_2 + iVar11 * 4);
      do {
        iVar12 = iVar11;
        iVar11 = iVar12 + 1;
        puVar7 = puVar7 + 1;
        if (param_4 <= iVar11) break;
      } while (*puVar7 < uVar1);
      puVar7 = (uint *)(param_2 + iVar6 * 4);
      do {
        iVar6 = iVar6 + -1;
        puVar7 = puVar7 + -1;
        if (iVar6 < 1) break;
      } while (uVar1 < *puVar7);
      uVar2 = *(undefined4 *)(param_1 + 4 + iVar11 * 8);
      uVar3 = *(undefined4 *)(param_1 + iVar11 * 8);
      *(undefined4 *)(param_1 + iVar11 * 8) = *(undefined4 *)(param_1 + iVar6 * 8);
      *(undefined4 *)(param_1 + 4 + iVar11 * 8) = *(undefined4 *)(param_1 + 4 + iVar6 * 8);
      *(undefined4 *)(param_1 + iVar6 * 8) = uVar3;
      *(undefined4 *)(param_1 + 4 + iVar6 * 8) = uVar2;
      uVar4 = *(undefined4 *)(param_2 + iVar11 * 4);
      *(undefined4 *)(param_2 + iVar11 * 4) = *(undefined4 *)(param_2 + iVar6 * 4);
      *(undefined4 *)(param_2 + iVar6 * 4) = uVar4;
    } while (iVar11 < iVar6);
    *(undefined4 *)(param_1 + iVar6 * 8) = *(undefined4 *)(param_1 + iVar11 * 8);
    *(undefined4 *)(param_1 + 4 + iVar6 * 8) = *(undefined4 *)(param_1 + 4 + iVar11 * 8);
    *(undefined4 *)(param_1 + iVar11 * 8) = *(undefined4 *)(param_1 + param_4 * 8);
    *(undefined4 *)(param_1 + 4 + iVar11 * 8) = *(undefined4 *)(param_1 + 4 + param_4 * 8);
    *(undefined4 *)(param_1 + param_4 * 8) = uVar3;
    *(undefined4 *)(param_1 + 4 + param_4 * 8) = uVar2;
    *(undefined4 *)(param_2 + iVar6 * 4) = *(undefined4 *)(param_2 + iVar11 * 4);
    *(undefined4 *)(param_2 + iVar11 * 4) = *(undefined4 *)(param_2 + param_4 * 4);
    *(undefined4 *)(param_2 + param_4 * 4) = uVar4;
    if (param_3 < iVar12) {
      FUN_1001d900(param_1,param_2,param_3,iVar12);
    }
    param_3 = iVar12 + 2;
    if (param_4 <= param_3) {
      return;
    }
    iVar11 = -param_3;
  }
  iVar11 = param_3 + 1;
  if (iVar11 < param_4 + 1) {
    puVar8 = (undefined4 *)(param_1 + iVar11 * 8);
    puVar7 = (uint *)(param_2 + -4 + iVar11 * 4);
    do {
      uVar2 = *puVar8;
      uVar1 = puVar7[1];
      uVar3 = puVar8[1];
      uVar5 = *puVar7;
      puVar9 = puVar8;
      puVar10 = puVar7;
      iVar6 = iVar11;
      while (uVar1 < uVar5) {
        puVar10[1] = *puVar10;
        *puVar9 = puVar9[-2];
        puVar9[1] = puVar9[-1];
        iVar6 = iVar6 + -1;
        puVar9 = puVar9 + -2;
        puVar10 = puVar10 + -1;
        if (iVar6 == param_3) break;
        uVar5 = *puVar10;
      }
      *(uint *)(param_2 + iVar6 * 4) = uVar1;
      *(undefined4 *)(param_1 + iVar6 * 8) = uVar2;
      *(undefined4 *)(param_1 + 4 + iVar6 * 8) = uVar3;
      iVar11 = iVar11 + 1;
      puVar8 = puVar8 + 2;
      puVar7 = puVar7 + 1;
    } while (iVar11 < param_4 + 1);
  }
  return;
}



void __cdecl FUN_1001daa0(int param_1,int param_2,int param_3,int param_4)

{
  undefined4 uVar1;
  uint uVar2;
  uint *puVar3;
  int iVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  int iVar7;
  uint *puVar8;
  
  iVar7 = param_3 + 1;
  if (iVar7 < param_4) {
    puVar5 = (undefined4 *)(param_1 + iVar7 * 4);
    puVar8 = (uint *)(param_2 + -4 + iVar7 * 4);
    do {
      uVar1 = *puVar5;
      uVar2 = puVar8[1];
      iVar4 = iVar7;
      if (uVar2 < *puVar8) {
        puVar3 = puVar8;
        puVar6 = puVar5;
        do {
          puVar3[1] = *puVar3;
          *puVar6 = *(undefined4 *)((param_1 - param_2) + (int)puVar3);
          iVar4 = iVar4 + -1;
          puVar6 = puVar6 + -1;
          puVar3 = puVar3 + -1;
          if (iVar4 == param_3) break;
        } while (uVar2 < *puVar3);
      }
      *(uint *)(param_2 + iVar4 * 4) = uVar2;
      *(undefined4 *)(param_1 + iVar4 * 4) = uVar1;
      iVar7 = iVar7 + 1;
      puVar5 = puVar5 + 1;
      puVar8 = puVar8 + 1;
    } while (iVar7 < param_4);
  }
  return;
}



void __cdecl FUN_1001db40(int param_1,int param_2,int param_3,int param_4)

{
  uint uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  int iVar4;
  uint *puVar5;
  uint *puVar6;
  undefined4 *puVar7;
  undefined4 *puVar8;
  int iVar9;
  int iVar10;
  
  iVar4 = -param_3;
  while (8 < param_4 + iVar4) {
    uVar1 = *(uint *)(param_2 + param_4 * 4);
    iVar9 = param_3 + -1;
    iVar4 = param_4;
    do {
      puVar5 = (uint *)(param_2 + iVar9 * 4);
      do {
        iVar10 = iVar9;
        iVar9 = iVar10 + 1;
        puVar5 = puVar5 + 1;
        if (param_4 <= iVar9) break;
      } while (*puVar5 < uVar1);
      puVar5 = (uint *)(param_2 + iVar4 * 4);
      do {
        iVar4 = iVar4 + -1;
        puVar5 = puVar5 + -1;
        if (iVar4 < 1) break;
      } while (uVar1 < *puVar5);
      uVar2 = *(undefined4 *)(param_1 + iVar9 * 4);
      *(undefined4 *)(param_1 + iVar9 * 4) = *(undefined4 *)(param_1 + iVar4 * 4);
      *(undefined4 *)(param_1 + iVar4 * 4) = uVar2;
      uVar3 = *(undefined4 *)(param_2 + iVar9 * 4);
      *(undefined4 *)(param_2 + iVar9 * 4) = *(undefined4 *)(param_2 + iVar4 * 4);
      *(undefined4 *)(param_2 + iVar4 * 4) = uVar3;
    } while (iVar9 < iVar4);
    *(undefined4 *)(param_1 + iVar4 * 4) = *(undefined4 *)(param_1 + iVar9 * 4);
    *(undefined4 *)(param_1 + iVar9 * 4) = *(undefined4 *)(param_1 + param_4 * 4);
    *(undefined4 *)(param_1 + param_4 * 4) = uVar2;
    *(undefined4 *)(param_2 + iVar4 * 4) = *(undefined4 *)(param_2 + iVar9 * 4);
    *(undefined4 *)(param_2 + iVar9 * 4) = *(undefined4 *)(param_2 + param_4 * 4);
    *(undefined4 *)(param_2 + param_4 * 4) = uVar3;
    if (param_3 < iVar10) {
      FUN_1001db40(param_1,param_2,param_3,iVar10);
    }
    param_3 = iVar10 + 2;
    if (param_4 <= param_3) {
      return;
    }
    iVar4 = -param_3;
  }
  iVar4 = param_3 + 1;
  if (iVar4 < param_4 + 1) {
    puVar8 = (undefined4 *)(param_1 + iVar4 * 4);
    puVar5 = (uint *)(param_2 + -4 + iVar4 * 4);
    do {
      uVar1 = puVar5[1];
      uVar2 = *puVar8;
      iVar9 = iVar4;
      if (uVar1 < *puVar5) {
        puVar6 = puVar5;
        puVar7 = puVar8;
        do {
          puVar6[1] = *puVar6;
          *puVar7 = *(undefined4 *)((int)puVar6 + (param_1 - param_2));
          iVar9 = iVar9 + -1;
          puVar7 = puVar7 + -1;
          puVar6 = puVar6 + -1;
          if (iVar9 == param_3) break;
        } while (uVar1 < *puVar6);
      }
      *(uint *)(param_2 + iVar9 * 4) = uVar1;
      *(undefined4 *)(param_1 + iVar9 * 4) = uVar2;
      iVar4 = iVar4 + 1;
      puVar8 = puVar8 + 1;
      puVar5 = puVar5 + 1;
    } while (iVar4 < param_4 + 1);
  }
  return;
}



// public: __thiscall Emissive_ARGB_Model::Emissive_ARGB_Model(long,long)

Emissive_ARGB_Model * __thiscall
Emissive_ARGB_Model::Emissive_ARGB_Model(Emissive_ARGB_Model *this,long param_1,long param_2)

{
  srRegistry *this_00;
  srRegistry *this_01;
  srRegistry *this_02;
  ClassNode *pCVar1;
  Coeff_Info *this_03;
  undefined4 uVar2;
  ulong uVar3;
  int iVar4;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0x1dca0  8  ??0Emissive_ARGB_Model@@QAE@JJ@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_10024184;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  srMeshModel::srMeshModel((srMeshModel *)this,0,0);
  *(undefined ***)this = &PTR_FUN_10025f54;
  this_00 = *(srRegistry **)(srCore_exref + 0x2c);
  uStack_4 = 0;
  pCVar1 = srRegistry::getClassNode(this_00,0x18888);
  if (pCVar1 == (ClassNode *)0x0) {
    this_01 = *(srRegistry **)(srCore_exref + 0x2c);
    pCVar1 = srRegistry::getClassNode(this_01,0x2010);
    if (pCVar1 == (ClassNode *)0x0) {
      this_02 = *(srRegistry **)(srCore_exref + 0x2c);
      pCVar1 = srRegistry::getClassNode(this_02,0x2000);
      if (pCVar1 == (ClassNode *)0x0) {
        iVar4 = 1;
        uVar3 = 0x2000;
        pCVar1 = srClass::sGetClassNode();
        pCVar1 = srRegistry::registerClass(this_02,s_srModel_10038204,pCVar1,uVar3,iVar4);
      }
      pCVar1 = srRegistry::registerClass(this_01,s_srMeshModel_100381f8,pCVar1,0x2010,0);
    }
    pCVar1 = srRegistry::registerClass(this_00,s_srMeshModel_100381f8,pCVar1,0x18888,0);
  }
  srRegistry::registerInstance(this_00,pCVar1,(srRuntimeClass *)this);
  uStack_4 = 1;
  *(undefined4 *)(this + 0x39c) = 0;
  FUN_100216a3(this + 0x3a0,8,4,&LAB_1001e580);
  uStack_4._0_1_ = 2;
  *(undefined ***)this = &PTR_FUN_10025ef4;
  this_03 = (Coeff_Info *)operator_new(0x18);
  uStack_4._0_1_ = 3;
  if (this_03 == (Coeff_Info *)0x0) {
    uVar2 = 0;
  }
  else {
    uVar2 = Housemarque::Threedee_Engine::Coeff_Info::Coeff_Info(this_03);
  }
  uStack_4 = CONCAT31(uStack_4._1_3_,2);
  *(undefined4 *)(this + 0x398) = uVar2;
  srMeshModel::reset((srMeshModel *)this,param_1,param_2);
  ExceptionList = pvStack_c;
  return this;
}



// public: virtual __thiscall Emissive_ARGB_Model::~Emissive_ARGB_Model(void)

void __thiscall Emissive_ARGB_Model::~Emissive_ARGB_Model(Emissive_ARGB_Model *this)

{
  srRegistry *this_00;
  srRegistry *this_01;
  srRegistry *this_02;
  ClassNode *pCVar1;
  ulong uVar2;
  int iVar3;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0x1de10  31  ??1Emissive_ARGB_Model@@UAE@XZ
  puStack_8 = &LAB_1002419f;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined ***)this = &PTR_FUN_10025ef4;
  local_4 = 0;
  FUN_100212a4(*(void **)(this + 0x398));
  FUN_100215af(this + 0x3a0,8,4,FUN_1001f590);
  *(undefined ***)this = &PTR_FUN_10025f54;
  this_00 = *(srRegistry **)(srCore_exref + 0x2c);
  local_4 = 1;
  pCVar1 = srRegistry::getClassNode(this_00,0x18888);
  if (pCVar1 == (ClassNode *)0x0) {
    this_01 = *(srRegistry **)(srCore_exref + 0x2c);
    pCVar1 = srRegistry::getClassNode(this_01,0x2010);
    if (pCVar1 == (ClassNode *)0x0) {
      this_02 = *(srRegistry **)(srCore_exref + 0x2c);
      pCVar1 = srRegistry::getClassNode(this_02,0x2000);
      if (pCVar1 == (ClassNode *)0x0) {
        iVar3 = 1;
        uVar2 = 0x2000;
        pCVar1 = srClass::sGetClassNode();
        pCVar1 = srRegistry::registerClass(this_02,s_srModel_10038204,pCVar1,uVar2,iVar3);
      }
      pCVar1 = srRegistry::registerClass(this_01,s_srMeshModel_100381f8,pCVar1,0x2010,0);
    }
    pCVar1 = srRegistry::registerClass(this_00,s_srMeshModel_100381f8,pCVar1,0x18888,0);
  }
  srRegistry::unregisterInstance(this_00,pCVar1,(srRuntimeClass *)this);
  local_4 = 0xffffffff;
  srMeshModel::~srMeshModel((srMeshModel *)this);
  ExceptionList = pvStack_c;
  return;
}



undefined4 FUN_1001df40(void)

{
  Emissive_ARGB_Model *this;
  undefined4 uVar1;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_100241b4;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  this = (Emissive_ARGB_Model *)srHeap::allocate((srHeap *)srHeap_exref,0x3c0);
  uStack_4 = 0;
  if (this != (Emissive_ARGB_Model *)0x0) {
    uVar1 = Emissive_ARGB_Model::Emissive_ARGB_Model(this,0,0);
    ExceptionList = pvStack_c;
    return uVar1;
  }
  ExceptionList = pvStack_c;
  return 0;
}



void * __thiscall FUN_1001dfb0(void *this,srMeshModel *param_1)

{
  uint uVar1;
  undefined4 *puVar2;
  undefined4 *puVar3;
  int iVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  int *piVar8;
  
  if ((srMeshModel *)this == param_1) {
    return this;
  }
  srMeshModel::operator=((srMeshModel *)this,param_1);
  iVar7 = (int)param_1 - (int)this;
  piVar8 = (int *)((int)this + 0x3a0);
  param_1 = (srMeshModel *)0x4;
  do {
    if ((int *)((int)piVar8 + iVar7) != piVar8) {
      if ((void *)*piVar8 != (void *)0x0) {
        srHeap::free((void *)*piVar8);
      }
      *piVar8 = 0;
      piVar8[1] = 0;
      uVar1 = *(uint *)((int)piVar8 + iVar7 + 4);
      if (uVar1 != 0) {
        if (uVar1 == 0) {
          FUN_1001f7b0(piVar8);
        }
        else {
          puVar2 = (undefined4 *)srHeap::allocate((srHeap *)srHeap_exref,uVar1 * 4);
          iVar6 = *piVar8;
          if ((iVar6 != 0) && (uVar5 = piVar8[1], uVar5 != 0)) {
            if (uVar1 < uVar5) {
              uVar5 = uVar1;
            }
            if ((uVar5 != 0) && (uVar5 != 0)) {
              puVar3 = puVar2;
              do {
                *puVar3 = *(undefined4 *)((iVar6 - (int)puVar2) + (int)puVar3);
                puVar3 = puVar3 + 1;
                uVar5 = uVar5 - 1;
              } while (uVar5 != 0);
            }
          }
          FUN_1001f7b0(piVar8);
          *piVar8 = (int)puVar2;
          piVar8[1] = uVar1;
        }
        iVar6 = piVar8[1];
        puVar2 = (undefined4 *)*piVar8;
        if ((iVar6 != 0) && (iVar6 != 0)) {
          iVar4 = *(int *)((int)piVar8 + iVar7) - (int)puVar2;
          do {
            *puVar2 = *(undefined4 *)(iVar4 + (int)puVar2);
            puVar2 = puVar2 + 1;
            iVar6 = iVar6 + -1;
          } while (iVar6 != 0);
        }
      }
    }
    piVar8 = piVar8 + 2;
    param_1 = param_1 + -1;
  } while (param_1 != (srMeshModel *)0x0);
  return this;
}



// public: class srARGB * __fastcall Emissive_ARGB_Model::getVertexARGB(long,int)

srARGB * __fastcall Emissive_ARGB_Model::getVertexARGB(long param_1,int param_2)

{
  int *piVar1;
  int iVar2;
  void *pvVar3;
  uint uVar4;
  srARGB *psVar5;
  int in_stack_00000004;
  
                    // 0x1e4a0  157  ?getVertexARGB@Emissive_ARGB_Model@@QAIPAVsrARGB@@JH@Z
  psVar5 = *(srARGB **)(param_1 + 0x3a0 + param_2 * 8);
  piVar1 = (int *)(param_1 + 0x3a0 + param_2 * 8);
  if (psVar5 == (srARGB *)0x0) {
    if (in_stack_00000004 != 0) {
      iVar2 = *(int *)(param_1 + 0x22c);
      if (piVar1[1] != iVar2) {
        if (iVar2 == 0) {
          FUN_1001f7b0(piVar1);
        }
        else {
          pvVar3 = srHeap::allocate((srHeap *)srHeap_exref,iVar2 * 4);
          FUN_1001f7b0(piVar1);
          *piVar1 = (int)pvVar3;
          piVar1[1] = iVar2;
        }
      }
      uVar4 = 0;
      if (piVar1[1] != 0) {
        do {
          *(undefined4 *)(*piVar1 + uVar4 * 4) = 0xffffffff;
          uVar4 = uVar4 + 1;
        } while (uVar4 < (uint)piVar1[1]);
      }
    }
    psVar5 = (srARGB *)*piVar1;
  }
  return psVar5;
}



void __thiscall FUN_1001e520(void *this,undefined4 param_1)

{
  *(undefined4 *)((int)this + 0x39c) = param_1;
  return;
}



void __fastcall FUN_1001e530(int param_1,undefined param_2,undefined4 param_3)

{
  Housemarque::Threedee_Engine::Coeff_Info::operator=(*(Coeff_Info **)(param_1 + 0x398));
  return;
}



void * __thiscall FUN_1001e550(void *this,byte param_1)

{
  Emissive_ARGB_Model::~Emissive_ARGB_Model((Emissive_ARGB_Model *)this);
  if ((param_1 & 1) != 0) {
    srHeap::free(this);
  }
  return this;
}



undefined4 __cdecl FUN_1001e590(int param_1)

{
  int iVar1;
  undefined4 *this;
  srVertexPipe *this_00;
  undefined4 uVar2;
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10024216;
  local_c = ExceptionList;
  if (param_1 == 0) {
    return 0;
  }
  ExceptionList = &local_c;
  if (*(int *)pipe_exref == 0) {
    ExceptionList = &local_c;
    this = (undefined4 *)operator_new(0xac);
    local_4 = 0;
    if (this == (undefined4 *)0x0) {
      this = (undefined4 *)0x0;
    }
    else {
      this[1] = 0;
      this[2] = 0;
      FUN_1001f5f0(this + 1,0);
      local_4._0_1_ = 1;
      this[3] = 0;
      this[4] = 0;
      FUN_1001f6a0(this + 3,0);
      local_4._0_1_ = 2;
      this[10] = 0;
      FUN_10014400(this + 0x1d);
      this[0x25] = 0;
      this[0x26] = 0;
      this[0x27] = 0;
      this[0x28] = 0;
      this[0x29] = 0;
      this[0x2a] = 0;
      local_4._0_1_ = 5;
      *this = &PTR_LAB_10025fb4;
      this_00 = (srVertexPipe *)operator_new(0x9c);
      local_4._0_1_ = 6;
      if (this_00 == (srVertexPipe *)0x0) {
        uVar2 = 0;
      }
      else {
        uVar2 = srVertexPipe::srVertexPipe(this_00);
      }
      local_4 = CONCAT31(local_4._1_3_,5);
      this[0x24] = uVar2;
      this[0x23] = 0;
      FUN_1001e740(this,0);
      FUN_1001e860(this);
    }
    *(undefined4 **)pipe_exref = this;
  }
  local_4 = 0xffffffff;
  iVar1 = *(int *)pipe_exref;
  *(int *)(iVar1 + 0x88) = param_1;
  *(undefined4 *)(iVar1 + 0x84) = 0;
  *(undefined4 *)(iVar1 + 0x28) = 0;
  *(undefined4 *)(iVar1 + 0x28) = 1;
  *(undefined4 *)(iVar1 + 0x28) = 3;
  *(undefined4 *)(iVar1 + 0x1c) = 0;
  *(undefined4 *)(iVar1 + 0x2c) = 0;
  *(undefined4 *)(iVar1 + 0x30) = 0;
  *(undefined4 *)(iVar1 + 0x34) = 0;
  *(undefined4 *)(iVar1 + 0x20) = 0;
  *(undefined4 *)(iVar1 + 0x38) = 0;
  *(undefined4 *)(iVar1 + 0x3c) = 0;
  *(undefined4 *)(iVar1 + 0x6c) = 0;
  *(undefined4 *)(iVar1 + 0x40) = 0;
  *(undefined4 *)(iVar1 + 0x74) = 0x100241b;
  *(undefined4 *)(iVar1 + 0x78) = 0;
  *(undefined4 *)(iVar1 + 0x7c) = 0;
  *(undefined4 *)(iVar1 + 0x80) = *(undefined4 *)(srCore_exref + 0x170);
  FUN_1001e890(iVar1);
  ExceptionList = local_c;
  return *(undefined4 *)pipe_exref;
}



void __thiscall FUN_1001e730(void *this,undefined4 param_1)

{
  *(undefined4 *)((int)this + 0x74) = param_1;
  *(undefined4 *)(*(int *)((int)this + 0x18) + 8) = param_1;
  return;
}



void __thiscall FUN_1001e740(void *this,undefined4 param_1)

{
  uint uVar1;
  uint uVar2;
  
  *(undefined4 *)((int)this + 0x84) = 0;
  *(undefined4 *)((int)this + 0x88) = param_1;
  *(undefined4 *)((int)this + 0x28) = 0;
  *(undefined4 *)((int)this + 0x28) = 1;
  *(undefined4 *)((int)this + 0x28) = 3;
  *(undefined4 *)((int)this + 0x1c) = 0;
  *(undefined4 *)((int)this + 0x2c) = 0;
  *(undefined4 *)((int)this + 0x30) = 0;
  *(undefined4 *)((int)this + 0x34) = 0;
  *(undefined4 *)((int)this + 0x20) = 0;
  *(undefined4 *)((int)this + 0x38) = 0;
  *(undefined4 *)((int)this + 0x3c) = 0;
  *(undefined4 *)((int)this + 0x6c) = 0;
  *(undefined4 *)((int)this + 0x40) = 0;
  *(undefined4 *)((int)this + 0x74) = 0x100241b;
  uVar1 = *(uint *)((int)this + 0x84);
  *(undefined4 *)((int)this + 0x78) = 0;
  *(undefined4 *)((int)this + 0x7c) = 0;
  *(undefined4 *)((int)this + 0x80) = *(undefined4 *)(srCore_exref + 0x170);
  if (*(uint *)((int)this + 0x98) <= uVar1) {
    FUN_1001f880((int *)((int)this + 0x94),*(uint *)((int)this + 0x98) + 8 + uVar1);
  }
  uVar2 = *(uint *)((int)this + 0x84);
  *(uint *)((int)this + 0x14) = *(int *)((int)this + 0x94) + uVar1 * 0x5c;
  if (*(uint *)((int)this + 0xa0) <= uVar2) {
    FUN_1001f9d0((int *)((int)this + 0x9c),*(uint *)((int)this + 0xa0) + 8 + uVar2);
  }
  *(uint *)((int)this + 0x18) = uVar2 * 0x20 + *(int *)((int)this + 0x9c);
  **(undefined4 **)((int)this + 0x14) = 0;
  *(undefined4 *)(*(int *)((int)this + 0x14) + 4) = 0;
  *(undefined4 *)(*(int *)((int)this + 0x14) + 8) = *(undefined4 *)((int)this + 0x80);
  **(undefined4 **)((int)this + 0x18) = *(undefined4 *)((int)this + 0x78);
  *(undefined4 *)(*(int *)((int)this + 0x18) + 4) = *(undefined4 *)((int)this + 0x7c);
  *(undefined4 *)(*(int *)((int)this + 0x18) + 8) = *(undefined4 *)((int)this + 0x74);
  *(undefined4 *)(*(int *)((int)this + 0x18) + 0xc) = 0;
  *(undefined4 *)(*(int *)((int)this + 0x18) + 0x10) = 0;
  *(undefined4 *)(*(int *)((int)this + 0x18) + 0x14) = 0;
  *(undefined4 *)(*(int *)((int)this + 0x18) + 0x18) = 0;
  *(undefined4 *)(*(int *)((int)this + 0x18) + 0x1c) = 0;
  return;
}



void __fastcall FUN_1001e860(undefined4 *param_1)

{
  param_1[0x23] = 1;
  if (param_1[0x21] != 0) {
    (**(code **)*param_1)();
  }
  param_1[0x23] = 0;
  return;
}



void __fastcall FUN_1001e890(int param_1)

{
  uint uVar1;
  uint uVar2;
  
  uVar1 = *(uint *)(param_1 + 0x84);
  if (*(uint *)(param_1 + 0x98) <= uVar1) {
    FUN_1001f880((int *)(param_1 + 0x94),*(uint *)(param_1 + 0x98) + 8 + uVar1);
  }
  uVar2 = *(uint *)(param_1 + 0x84);
  *(uint *)(param_1 + 0x14) = *(int *)(param_1 + 0x94) + uVar1 * 0x5c;
  if (*(uint *)(param_1 + 0xa0) <= uVar2) {
    FUN_1001f9d0((int *)(param_1 + 0x9c),*(uint *)(param_1 + 0xa0) + 8 + uVar2);
  }
  *(uint *)(param_1 + 0x18) = uVar2 * 0x20 + *(int *)(param_1 + 0x9c);
  **(undefined4 **)(param_1 + 0x14) = 0;
  *(undefined4 *)(*(int *)(param_1 + 0x14) + 4) = 0;
  *(undefined4 *)(*(int *)(param_1 + 0x14) + 8) = *(undefined4 *)(param_1 + 0x80);
  **(undefined4 **)(param_1 + 0x18) = *(undefined4 *)(param_1 + 0x78);
  *(undefined4 *)(*(int *)(param_1 + 0x18) + 4) = *(undefined4 *)(param_1 + 0x7c);
  *(undefined4 *)(*(int *)(param_1 + 0x18) + 8) = *(undefined4 *)(param_1 + 0x74);
  *(undefined4 *)(*(int *)(param_1 + 0x18) + 0xc) = 0;
  *(undefined4 *)(*(int *)(param_1 + 0x18) + 0x10) = 0;
  *(undefined4 *)(*(int *)(param_1 + 0x18) + 0x14) = 0;
  *(undefined4 *)(*(int *)(param_1 + 0x18) + 0x18) = 0;
  *(undefined4 *)(*(int *)(param_1 + 0x18) + 0x1c) = 0;
  return;
}



void __thiscall FUN_1001f3a0(void *this,double param_1,double param_2,double param_3)

{
  *(float *)this = (float)param_1;
  *(float *)((int)this + 4) = (float)param_2;
  *(float *)((int)this + 8) = (float)param_3;
  return;
}



void * __fastcall FUN_1001f480(srMeshModel *param_1)

{
  void *this;
  
  this = (void *)(**(code **)(*(int *)param_1 + 0x20))();
  FUN_1001dfb0(this,param_1);
  return this;
}



void __fastcall FUN_1001f4a0(srMeshModel *param_1)

{
  srRegistry *this;
  srRegistry *this_00;
  srRegistry *this_01;
  ClassNode *pCVar1;
  ulong uVar2;
  int iVar3;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_10024229;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  *(undefined ***)param_1 = &PTR_FUN_10025f54;
  this = *(srRegistry **)(srCore_exref + 0x2c);
  local_4 = 0;
  pCVar1 = srRegistry::getClassNode(this,0x18888);
  if (pCVar1 == (ClassNode *)0x0) {
    this_00 = *(srRegistry **)(srCore_exref + 0x2c);
    pCVar1 = srRegistry::getClassNode(this_00,0x2010);
    if (pCVar1 == (ClassNode *)0x0) {
      this_01 = *(srRegistry **)(srCore_exref + 0x2c);
      pCVar1 = srRegistry::getClassNode(this_01,0x2000);
      if (pCVar1 == (ClassNode *)0x0) {
        iVar3 = 1;
        uVar2 = 0x2000;
        pCVar1 = srClass::sGetClassNode();
        pCVar1 = srRegistry::registerClass(this_01,s_srModel_10038204,pCVar1,uVar2,iVar3);
      }
      pCVar1 = srRegistry::registerClass(this_00,s_srMeshModel_100381f8,pCVar1,0x2010,0);
    }
    pCVar1 = srRegistry::registerClass(this,s_srMeshModel_100381f8,pCVar1,0x18888,0);
  }
  srRegistry::unregisterInstance(this,pCVar1,(srRuntimeClass *)param_1);
  local_4 = 0xffffffff;
  srMeshModel::~srMeshModel(param_1);
  ExceptionList = pvStack_c;
  return;
}



void __fastcall FUN_1001f590(undefined4 *param_1)

{
  if ((void *)*param_1 != (void *)0x0) {
    srHeap::free((void *)*param_1);
  }
  *param_1 = 0;
  param_1[1] = 0;
  return;
}



void __fastcall FUN_1001f5c0(undefined4 *param_1)

{
  if ((void *)*param_1 != (void *)0x0) {
    srHeap::free((void *)*param_1);
  }
  *param_1 = 0;
  param_1[1] = 0;
  return;
}



void * __thiscall FUN_1001f5f0(void *this,uint param_1)

{
  void *pvVar1;
  
                    // WARNING: Load size is inaccurate
  pvVar1 = *this;
  if (*(uint *)((int)this + 4) < param_1) {
    if (pvVar1 != (void *)0x0) {
      srHeap::free(pvVar1);
    }
    *(undefined4 *)this = 0;
    *(undefined4 *)((int)this + 4) = 0;
    if (param_1 != 0) {
      param_1 = _ftol();
    }
    *(uint *)((int)this + 4) = param_1;
    if (param_1 != 0) {
      pvVar1 = srHeap::allocate((srHeap *)srHeap_exref,param_1 * 4);
      *(void **)this = pvVar1;
    }
                    // WARNING: Load size is inaccurate
    pvVar1 = *this;
  }
  return pvVar1;
}



void __fastcall FUN_1001f670(undefined4 *param_1)

{
  if ((void *)*param_1 != (void *)0x0) {
    srHeap::free((void *)*param_1);
  }
  *param_1 = 0;
  param_1[1] = 0;
  return;
}



void * __thiscall FUN_1001f6a0(void *this,uint param_1)

{
  void *pvVar1;
  
                    // WARNING: Load size is inaccurate
  pvVar1 = *this;
  if (*(uint *)((int)this + 4) < param_1) {
    if (pvVar1 != (void *)0x0) {
      srHeap::free(pvVar1);
    }
    *(undefined4 *)this = 0;
    *(undefined4 *)((int)this + 4) = 0;
    if (param_1 != 0) {
      param_1 = _ftol();
    }
    *(uint *)((int)this + 4) = param_1;
    if (param_1 != 0) {
      pvVar1 = srHeap::allocate((srHeap *)srHeap_exref,param_1 * 4);
      *(void **)this = pvVar1;
    }
                    // WARNING: Load size is inaccurate
    pvVar1 = *this;
  }
  return pvVar1;
}



void __fastcall FUN_1001f720(undefined4 *param_1)

{
  FUN_100212a4((void *)*param_1);
  *param_1 = 0;
  param_1[1] = 0;
  return;
}



void __fastcall FUN_1001f740(undefined4 *param_1)

{
  FUN_100212a4((void *)*param_1);
  *param_1 = 0;
  param_1[1] = 0;
  return;
}



void __fastcall FUN_1001f760(undefined4 *param_1)

{
  FUN_100212a4((void *)*param_1);
  *param_1 = 0;
  param_1[1] = 0;
  return;
}



void * __thiscall FUN_1001f780(void *this,byte param_1)

{
  FUN_1001f4a0((srMeshModel *)this);
  if ((param_1 & 1) != 0) {
    srHeap::free(this);
  }
  return this;
}



void __fastcall FUN_1001f7b0(undefined4 *param_1)

{
  if ((void *)*param_1 != (void *)0x0) {
    srHeap::free((void *)*param_1);
  }
  *param_1 = 0;
  param_1[1] = 0;
  return;
}



void FUN_1001f7e0(int param_1)

{
  srHeap::allocate((srHeap *)srHeap_exref,param_1 * 4);
  return;
}



void FUN_1001f800(int param_1)

{
  srHeap::allocate((srHeap *)srHeap_exref,param_1 * 4);
  return;
}



int __thiscall FUN_1001f820(void *this,uint param_1)

{
  if (*(uint *)((int)this + 4) <= param_1) {
    FUN_1001f880(this,*(uint *)((int)this + 4) + 8 + param_1);
  }
                    // WARNING: Load size is inaccurate
  return *this + param_1 * 0x5c;
}



int __thiscall FUN_1001f850(void *this,uint param_1)

{
  if (*(uint *)((int)this + 4) <= param_1) {
    FUN_1001f9d0(this,*(uint *)((int)this + 4) + 8 + param_1);
  }
                    // WARNING: Load size is inaccurate
  return param_1 * 0x20 + *this;
}



void __thiscall FUN_1001f880(void *this,uint param_1)

{
  undefined4 *puVar1;
  undefined4 *puVar2;
  uint uVar3;
  int iVar4;
  int iVar5;
  void *pvVar6;
  int iVar7;
  uint local_10;
  
  if (*(uint *)((int)this + 4) != param_1) {
    pvVar6 = (void *)0x0;
    if (param_1 != 0) {
      pvVar6 = operator_new(param_1 * 0x5c);
      if (pvVar6 == (void *)0x0) {
        pvVar6 = (void *)0x0;
      }
      else if (-1 < (int)(param_1 - 1)) {
        puVar2 = (undefined4 *)((int)pvVar6 + 4);
        uVar3 = param_1;
        do {
          puVar2[-1] = 0;
          *puVar2 = 0;
          puVar2[2] = 0;
          puVar2[3] = 2;
          puVar2 = puVar2 + 0x17;
          uVar3 = uVar3 - 1;
        } while (uVar3 != 0);
      }
                    // WARNING: Load size is inaccurate
      if ((*this != 0) && (local_10 = *(uint *)((int)this + 4), local_10 != 0)) {
        if (param_1 <= local_10) {
          local_10 = param_1;
        }
        if (local_10 != 0) {
          puVar2 = (undefined4 *)((int)pvVar6 + 4);
          do {
                    // WARNING: Load size is inaccurate
            iVar4 = *this + (-4 - (int)pvVar6);
            puVar2[-1] = *(undefined4 *)((int)puVar2 + *this + (-4 - (int)pvVar6));
            *puVar2 = *(undefined4 *)((int)puVar2 + iVar4 + 4);
            puVar2[1] = *(undefined4 *)((int)puVar2 + iVar4 + 8);
            puVar2[2] = *(undefined4 *)((int)puVar2 + iVar4 + 0xc);
            puVar2[3] = *(undefined4 *)((int)puVar2 + iVar4 + 0x10);
            puVar2[4] = *(undefined4 *)((int)puVar2 + iVar4 + 0x14);
            puVar2[5] = *(undefined4 *)((int)puVar2 + iVar4 + 0x18);
            puVar2[6] = *(undefined4 *)((int)puVar2 + iVar4 + 0x1c);
            puVar1 = puVar2 + 7;
            iVar7 = (int)puVar2 + (iVar4 - (int)(puVar2 + -1));
            iVar5 = 2;
            do {
              *puVar1 = *(undefined4 *)(iVar7 + (int)puVar1);
              puVar1 = puVar1 + 1;
              iVar5 = iVar5 + -1;
            } while (iVar5 != 0);
            puVar2[9] = *(undefined4 *)((int)puVar2 + iVar4 + 0x28);
            puVar1 = puVar2 + 10;
            iVar4 = 0xc;
            do {
              *puVar1 = *(undefined4 *)((int)puVar1 + iVar7);
              puVar1 = puVar1 + 1;
              iVar4 = iVar4 + -1;
            } while (iVar4 != 0);
            puVar2 = puVar2 + 0x17;
            local_10 = local_10 - 1;
          } while (local_10 != 0);
        }
      }
    }
                    // WARNING: Load size is inaccurate
    FUN_100212a4(*this);
    *(void **)this = pvVar6;
    *(uint *)((int)this + 4) = param_1;
  }
  return;
}



void __thiscall FUN_1001f9d0(void *this,uint param_1)

{
  int iVar1;
  undefined4 *puVar2;
  uint uVar3;
  undefined4 *puVar4;
  undefined4 *puVar5;
  int iVar6;
  int iVar7;
  uint local_c;
  
  if (*(uint *)((int)this + 4) != param_1) {
    puVar5 = (undefined4 *)0x0;
    if (param_1 != 0) {
      puVar5 = (undefined4 *)operator_new(param_1 << 5);
      if (puVar5 == (undefined4 *)0x0) {
        puVar5 = (undefined4 *)0x0;
      }
      else if (-1 < (int)(param_1 - 1)) {
        puVar2 = puVar5 + 2;
        uVar3 = param_1;
        do {
          *puVar2 = 0x100241b;
          puVar2 = puVar2 + 8;
          uVar3 = uVar3 - 1;
        } while (uVar3 != 0);
      }
                    // WARNING: Load size is inaccurate
      if ((*this != 0) && (local_c = *(uint *)((int)this + 4), local_c != 0)) {
        if (param_1 <= local_c) {
          local_c = param_1;
        }
        if (local_c != 0) {
          iVar6 = 0;
          puVar2 = puVar5;
          do {
                    // WARNING: Load size is inaccurate
            iVar1 = iVar6 + *this;
            iVar7 = 2;
            puVar4 = puVar2;
            do {
              *puVar4 = *(undefined4 *)((iVar1 - (int)puVar2) + (int)puVar4);
              puVar4 = puVar4 + 1;
              iVar7 = iVar7 + -1;
            } while (iVar7 != 0);
            puVar2[2] = *(undefined4 *)(iVar1 + 8);
            puVar4 = puVar2 + 3;
            iVar7 = 2;
            do {
              *puVar4 = *(undefined4 *)((int)puVar4 + (iVar1 - (int)puVar2));
              puVar4 = puVar4 + 1;
              iVar7 = iVar7 + -1;
            } while (iVar7 != 0);
            puVar2[5] = *(undefined4 *)(iVar1 + 0x14);
            puVar2[6] = *(undefined4 *)(iVar1 + 0x18);
            puVar2[7] = *(undefined4 *)(iVar1 + 0x1c);
            iVar6 = iVar6 + 0x20;
            puVar2 = puVar2 + 8;
            local_c = local_c - 1;
          } while (local_c != 0);
        }
      }
    }
                    // WARNING: Load size is inaccurate
    FUN_100212a4(*this);
    *(undefined4 **)this = puVar5;
    *(uint *)((int)this + 4) = param_1;
  }
  return;
}



void __thiscall FUN_1001fad0(void *this,uint param_1)

{
  int iVar1;
  int iVar2;
  uint uVar3;
  void *pvVar4;
  undefined4 *puVar5;
  undefined4 *puVar6;
  
  if (*(uint *)((int)this + 4) != param_1) {
    pvVar4 = (void *)0x0;
                    // WARNING: Load size is inaccurate
    if (((param_1 != 0) && (pvVar4 = operator_new(param_1 << 5), *this != 0)) &&
       (uVar3 = *(uint *)((int)this + 4), uVar3 != 0)) {
      if (param_1 <= uVar3) {
        uVar3 = param_1;
      }
      if (uVar3 != 0) {
        iVar1 = 0;
        do {
                    // WARNING: Load size is inaccurate
          puVar5 = (undefined4 *)(iVar1 + *this);
          puVar6 = (undefined4 *)(iVar1 + (int)pvVar4);
          iVar1 = iVar1 + 0x20;
          uVar3 = uVar3 - 1;
          for (iVar2 = 8; iVar2 != 0; iVar2 = iVar2 + -1) {
            *puVar6 = *puVar5;
            puVar5 = puVar5 + 1;
            puVar6 = puVar6 + 1;
          }
        } while (uVar3 != 0);
      }
    }
                    // WARNING: Load size is inaccurate
    FUN_100212a4(*this);
    *(void **)this = pvVar4;
    *(uint *)((int)this + 4) = param_1;
  }
  return;
}



void FUN_1001fb60(void)

{
                    // WARNING: Could not recover jumptable at 0x1001fb65. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::Init((Init *)&DAT_10038519);
  return;
}



void FUN_1001fb70(void)

{
  FUN_100214c2(FUN_1001fb80);
  return;
}



void FUN_1001fb80(void)

{
                    // WARNING: Could not recover jumptable at 0x1001fb85. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ios_base::Init::~Init((Init *)&DAT_10038519);
  return;
}



void FUN_1001fba0(void)

{
                    // WARNING: Could not recover jumptable at 0x1001fba5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::_Winit((_Winit *)&DAT_10038518);
  return;
}



void FUN_1001fbb0(void)

{
  FUN_100214c2(FUN_1001fbc0);
  return;
}



void FUN_1001fbc0(void)

{
                    // WARNING: Could not recover jumptable at 0x1001fbc5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Winit::~_Winit((_Winit *)&DAT_10038518);
  return;
}



// public: __thiscall
// Housemarque::Threedee_Engine::Memory_Texture_Config::Memory_Texture_Config(class
// std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &)

Memory_Texture_Config * __thiscall
Housemarque::Threedee_Engine::Memory_Texture_Config::Memory_Texture_Config
          (Memory_Texture_Config *this,basic_string<> *param_1)

{
  Config_Base *this_00;
  basic_string<> *this_01;
  int iVar1;
  undefined4 *puVar2;
  basic_ostream<> *pbVar3;
  char *pcVar4;
  uint uVar5;
  code *_Filename;
  char *pcVar6;
  char cVar7;
  undefined uStack_c9;
  basic_string<> abStack_c8 [16];
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  undefined4 uStack_b0;
  basic_string<> *pbStack_a8;
  void *local_a4;
  Memory_Texture_Config *local_a0;
  code *pcStack_9c;
  byte abStack_98 [4];
  basic_streambuf<> abStack_94 [84];
  basic_ios<> abStack_40 [52];
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
                    // 0x1fbd0  14
                    // ??0Memory_Texture_Config@Threedee_Engine@Housemarque@@QAE@ABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@Z
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_1002435f;
  pvStack_c = ExceptionList;
  this_00 = (Config_Base *)(this + 4);
  local_a4 = (void *)0x0;
  ExceptionList = &pvStack_c;
  local_a0 = this;
  Housemarque::Template_Library::Config_Base::Config_Base(this_00);
  uStack_4 = 0;
  std::basic_string<>::_Tidy(abStack_c8,false);
  uVar5 = 0xffffffff;
  pcVar4 = s_emissive_coeff_10038270;
  do {
    if (uVar5 == 0) break;
    uVar5 = uVar5 - 1;
    cVar7 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar7 != '\0');
  std::basic_string<>::assign(abStack_c8,s_emissive_coeff_10038270,~uVar5 - 1);
  *(undefined4 *)(this + 0x10) = 0;
  *(undefined4 *)(this + 0x14) = 0;
  uStack_4._0_1_ = 1;
  uStack_b8 = 0;
  uStack_b4 = 0;
  uStack_b0 = 0;
  *(undefined4 *)(this + 0x18) = 0;
  pbStack_a8 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 2;
  if (pbStack_a8 != (basic_string<> *)0x0) {
    FUN_100155a0(pbStack_a8,abStack_c8,this_00,this + 0x10);
  }
  uStack_4._0_1_ = 0;
  std::basic_string<>::_Tidy(abStack_c8,true);
  std::basic_string<>::_Tidy(abStack_c8,false);
  uVar5 = 0xffffffff;
  pcVar4 = s_specular_coeff_10038280;
  do {
    if (uVar5 == 0) break;
    uVar5 = uVar5 - 1;
    cVar7 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar7 != '\0');
  std::basic_string<>::assign(abStack_c8,s_specular_coeff_10038280,~uVar5 - 1);
  uStack_b8 = 0;
  *(undefined4 *)(this + 0x1c) = 0;
  uStack_b4 = 0;
  uStack_b0 = 0;
  *(undefined4 *)(this + 0x20) = 0;
  uStack_4._0_1_ = 3;
  *(undefined4 *)(this + 0x24) = 0;
  pbStack_a8 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 4;
  if (pbStack_a8 != (basic_string<> *)0x0) {
    FUN_100155a0(pbStack_a8,abStack_c8,this_00,this + 0x1c);
  }
  uStack_4._0_1_ = 0;
  std::basic_string<>::_Tidy(abStack_c8,true);
  std::basic_string<>::_Tidy(abStack_c8,false);
  uVar5 = 0xffffffff;
  pcVar4 = s_diffuse_coeff_10038290;
  do {
    if (uVar5 == 0) break;
    uVar5 = uVar5 - 1;
    cVar7 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar7 != '\0');
  std::basic_string<>::assign(abStack_c8,s_diffuse_coeff_10038290,~uVar5 - 1);
  uStack_b8 = 0x3f800000;
  *(undefined4 *)(this + 0x28) = 0x3f800000;
  uStack_b4 = 0x3f800000;
  uStack_b0 = 0x3f800000;
  *(undefined4 *)(this + 0x2c) = 0x3f800000;
  uStack_4._0_1_ = 5;
  *(undefined4 *)(this + 0x30) = 0x3f800000;
  pbStack_a8 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 6;
  if (pbStack_a8 != (basic_string<> *)0x0) {
    FUN_100155a0(pbStack_a8,abStack_c8,this_00,this + 0x28);
  }
  uStack_4._0_1_ = 0;
  std::basic_string<>::_Tidy(abStack_c8,true);
  std::basic_string<>::_Tidy(abStack_c8,false);
  uVar5 = 0xffffffff;
  pcVar4 = s_ambient_coeff_100382a0;
  do {
    if (uVar5 == 0) break;
    uVar5 = uVar5 - 1;
    cVar7 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar7 != '\0');
  std::basic_string<>::assign(abStack_c8,s_ambient_coeff_100382a0,~uVar5 - 1);
  uStack_b8 = 0x3f800000;
  *(undefined4 *)(this + 0x34) = 0x3f800000;
  uStack_b4 = 0x3f800000;
  uStack_b0 = 0x3f800000;
  *(undefined4 *)(this + 0x38) = 0x3f800000;
  uStack_4._0_1_ = 7;
  *(undefined4 *)(this + 0x3c) = 0x3f800000;
  pbStack_a8 = (basic_string<> *)operator_new(0x20);
  uStack_4._0_1_ = 8;
  if (pbStack_a8 != (basic_string<> *)0x0) {
    FUN_100155a0(pbStack_a8,abStack_c8,this_00,this + 0x34);
  }
  uStack_4._0_1_ = 0;
  std::basic_string<>::_Tidy(abStack_c8,true);
  std::basic_string<>::_Tidy(abStack_c8,false);
  uVar5 = 0xffffffff;
  pcVar4 = s_shininess_100382b0;
  do {
    if (uVar5 == 0) break;
    uVar5 = uVar5 - 1;
    cVar7 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar7 != '\0');
  std::basic_string<>::assign(abStack_c8,s_shininess_100382b0,~uVar5 - 1);
  uStack_4._0_1_ = 9;
  local_a4 = (void *)0x3f800000;
  FUN_100145a0(this + 0x40,this_00,abStack_c8,&local_a4);
  uStack_4._0_1_ = 0;
  std::basic_string<>::_Tidy(abStack_c8,true);
  uStack_b8 = CONCAT31(uStack_b8._1_3_,uStack_c9);
  std::basic_string<>::_Tidy((basic_string<> *)&uStack_b8,false);
  uVar5 = 0xffffffff;
  pcVar4 = &DAT_1003851c;
  do {
    if (uVar5 == 0) break;
    uVar5 = uVar5 - 1;
    cVar7 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar7 != '\0');
  std::basic_string<>::assign((basic_string<> *)&uStack_b8,&DAT_1003851c,~uVar5 - 1);
  uStack_4._0_1_ = 10;
  std::basic_string<>::_Tidy(abStack_c8,false);
  uVar5 = 0xffffffff;
  pcVar4 = s_diffuse_illumination_100382bc;
  do {
    if (uVar5 == 0) break;
    uVar5 = uVar5 - 1;
    cVar7 = *pcVar4;
    pcVar4 = pcVar4 + 1;
  } while (cVar7 != '\0');
  std::basic_string<>::assign(abStack_c8,s_diffuse_illumination_100382bc,~uVar5 - 1);
  this_01 = (basic_string<> *)(this + 0x44);
  uStack_4._0_1_ = 0xb;
  pbStack_a8 = this_01;
  std::basic_string<>::basic_string<>(this_01,(basic_string<> *)&uStack_b8);
  uStack_4._0_1_ = 0xc;
  local_a4 = operator_new(0x20);
  uStack_4._0_1_ = 0xd;
  if (local_a4 != (void *)0x0) {
    FUN_10014030(local_a4,abStack_c8,this_00,this_01);
  }
  uStack_4._0_1_ = 0x10;
  std::basic_string<>::_Tidy(abStack_c8,true);
  uStack_4._0_1_ = 0xf;
  std::basic_string<>::_Tidy((basic_string<> *)&uStack_b8,true);
  *(undefined ***)this = &PTR_FUN_10025fec;
  _Filename = _C_exref;
  if (*(code **)(param_1 + 4) != (code *)0x0) {
    _Filename = *(code **)(param_1 + 4);
  }
  pcStack_9c = _vbtable__exref;
  std::basic_ios<>::basic_ios<>(abStack_40);
  local_a4 = (void *)0x2;
  uStack_4 = CONCAT31(uStack_4._1_3_,0x11);
  std::basic_istream<>::basic_istream<>((basic_istream<> *)&pcStack_9c,abStack_94,false);
  uStack_4 = 0x12;
  std::basic_filebuf<>::basic_filebuf<>((basic_filebuf<> *)abStack_94,(_iobuf *)0x0);
  *(code **)(abStack_98 + *(int *)(pcStack_9c + 4) + -4) = _vftable__exref;
  uStack_4 = CONCAT31(uStack_4._1_3_,0x13);
  iVar1 = std::basic_filebuf<>::open((char *)_Filename,1);
  if (iVar1 == 0) {
    std::basic_ios<>::setstate((basic_ios<> *)(abStack_98 + *(int *)(pcStack_9c + 4) + -4),2,false);
  }
  uStack_4._0_1_ = 0x14;
  if ((abStack_98[*(int *)(pcStack_9c + 4)] & 6) != 0) {
    local_a4 = (void *)0x3;
    puVar2 = (undefined4 *)Housemarque::Kernel::Error::Tmp_Str_Inserter(&pbStack_a8);
    uStack_4 = CONCAT31(uStack_4._1_3_,0x15);
    pbVar3 = std::operator<<((basic_ostream<> *)*puVar2,s_Memory_Texture_cannot_open_file__10038300)
    ;
    cVar7 = '\0';
    pbVar3 = std::operator<<(pbVar3,param_1);
    std::operator<<(pbVar3,cVar7);
    iVar1 = 0x2d;
    pcVar6 = s_E__work_ht_3de_Stream_Texture_Su_100382d4;
    pcVar4 = Housemarque::Kernel::Error::Tmp_Str();
    Housemarque::Kernel::Throw_Error(s__filee_fail___10038324,pcVar4,pcVar6,iVar1);
  }
  uStack_4 = 0x14;
  if (((uint)local_a4 & 1) != 0) {
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)&pbStack_a8);
  }
  Housemarque::Template_Library::operator>>((basic_istream<> *)&pcStack_9c,this_00);
  uStack_4 = CONCAT31(uStack_4._1_3_,0xf);
  std::basic_ifstream<>::~basic_ifstream<>((basic_ifstream<> *)abStack_40);
  std::basic_ios<>::~basic_ios<>(abStack_40);
  ExceptionList = pvStack_c;
  return this;
}



// public: virtual __thiscall
// Housemarque::Threedee_Engine::Memory_Texture_Config::~Memory_Texture_Config(void)

void __thiscall
Housemarque::Threedee_Engine::Memory_Texture_Config::~Memory_Texture_Config
          (Memory_Texture_Config *this)

{
  char *pcVar1;
  char cVar2;
  
                    // 0x20160  35  ??1Memory_Texture_Config@Threedee_Engine@Housemarque@@UAE@XZ
  *(undefined ***)this = &PTR_FUN_10025fec;
  if (*(int *)(this + 0x48) != 0) {
    pcVar1 = (char *)(*(int *)(this + 0x48) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(this + 0x48) = 0;
  *(undefined4 *)(this + 0x4c) = 0;
  *(undefined4 *)(this + 0x50) = 0;
  Housemarque::Template_Library::Config_Base::~Config_Base((Config_Base *)(this + 4));
  return;
}



// public: __thiscall Housemarque::Threedee_Engine::Memory_Texture::Memory_Texture(class
// Housemarque::Threedee_Engine::Memory_Texture_Config const &)

Memory_Texture * __thiscall
Housemarque::Threedee_Engine::Memory_Texture::Memory_Texture
          (Memory_Texture *this,Memory_Texture_Config *param_1)

{
                    // 0x201b0  13
                    // ??0Memory_Texture@Threedee_Engine@Housemarque@@QAE@ABVMemory_Texture_Config@12@@Z
  *(undefined4 *)(this + 4) = 0;
  *(undefined4 *)(this + 8) = 0;
  *(undefined4 *)(this + 0xc) = 0;
  *(undefined4 *)(this + 0x10) = 0;
  *(undefined4 *)(this + 0x14) = 0;
  *(undefined ***)this = &PTR_FUN_10025ff4;
  FUN_100202c0(this,(int)param_1,(srTextureMap *)0x0);
  return this;
}



// public: __thiscall Housemarque::Threedee_Engine::Memory_Texture::Memory_Texture(class
// std::basic_string<char,struct std::char_traits<char>,class std::allocator<char> > const &)

Memory_Texture * __thiscall
Housemarque::Threedee_Engine::Memory_Texture::Memory_Texture
          (Memory_Texture *this,basic_string<> *param_1)

{
  Memory_Texture_Config local_60 [84];
  void *local_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0x201e0  12
                    // ??0Memory_Texture@Threedee_Engine@Housemarque@@QAE@ABV?$basic_string@DU?$char_traits@D@std@@V?$allocator@D@2@@std@@@Z
  local_4 = 0xffffffff;
  puStack_8 = &LAB_10024371;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  *(undefined4 *)(this + 4) = 0;
  *(undefined4 *)(this + 8) = 0;
  *(undefined4 *)(this + 0xc) = 0;
  *(undefined4 *)(this + 0x10) = 0;
  *(undefined4 *)(this + 0x14) = 0;
  *(undefined ***)this = &PTR_FUN_10025ff4;
  Memory_Texture_Config::Memory_Texture_Config(local_60,param_1);
  local_4 = 0;
  FUN_100202c0(this,(int)local_60,(srTextureMap *)0x0);
  local_4 = 0xffffffff;
  Memory_Texture_Config::~Memory_Texture_Config(local_60);
  ExceptionList = local_c;
  return this;
}



// public: virtual __thiscall Housemarque::Threedee_Engine::Memory_Texture::~Memory_Texture(void)

void __thiscall Housemarque::Threedee_Engine::Memory_Texture::~Memory_Texture(Memory_Texture *this)

{
  void *pvVar1;
  srColorSurfaceIFace *this_00;
  
                    // 0x20260  34  ??1Memory_Texture@Threedee_Engine@Housemarque@@UAE@XZ
  pvVar1 = *(void **)(this + 0x14);
  *(undefined ***)this = &PTR_FUN_10025ff4;
  if (pvVar1 != (void *)0x0) {
    FUN_10020940((int)pvVar1);
    FUN_100212a4(pvVar1);
  }
  if (*(srClass **)(this + 8) != (srClass *)0x0) {
    srClass::release(*(srClass **)(this + 8));
  }
  if (*(srClass **)(this + 4) != (srClass *)0x0) {
    srClass::release(*(srClass **)(this + 4));
  }
  if (*(srClass **)(this + 0xc) != (srClass *)0x0) {
    srClass::release(*(srClass **)(this + 0xc));
  }
  if (*(srTextureMap **)(this + 0x10) != (srTextureMap *)0x0) {
    this_00 = srTextureMap::getSurfacePtr(*(srTextureMap **)(this + 0x10));
    if (this_00 != (srColorSurfaceIFace *)0x0) {
      srClass::release((srClass *)this_00);
    }
    srClass::release(*(srClass **)(this + 0x10));
  }
  return;
}



void __thiscall FUN_100202c0(void *this,int param_1,srTextureMap *param_2)

{
  int *piVar1;
  float fVar2;
  char cVar3;
  int iVar4;
  void *pvVar5;
  srClass *psVar6;
  int iVar7;
  code *pcVar8;
  srColorSurfaceIFace *this_00;
  srColorSurface *psVar9;
  srTextureMap *psVar10;
  uint uVar11;
  char *pcVar12;
  allocator<char> aStack_21;
  srClass *psStack_20;
  undefined4 uStack_1c;
  undefined4 uStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_100243c3;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (param_2 == (srTextureMap *)0x0) {
    ExceptionList = &pvStack_c;
    param_2 = (srTextureMap *)srCore::getSurfaceIOManager((srCore *)srCore_exref);
  }
  pvVar5 = operator_new(0x3c);
  psStack_20 = (srClass *)pvVar5;
  if (pvVar5 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    uStack_4._0_1_ = 1;
    uStack_4._1_3_ = 0;
    std::basic_string<>::basic_string<>((basic_string<> *)((int)pvVar5 + 8),&aStack_21);
    uStack_4 = CONCAT31(uStack_4._1_3_,2);
    FUN_10005d90((int)pvVar5 + 0x28,4,2,&LAB_10020980);
  }
  *(void **)((int)this + 0x14) = pvVar5;
  uStack_4 = 0xffffffff;
  psVar6 = (srClass *)srHeap::allocate((srHeap *)srHeap_exref,0x78);
  uStack_4 = 3;
  psStack_20 = psVar6;
  if (psVar6 == (srClass *)0x0) {
    psVar6 = (srClass *)0x0;
  }
  else {
    FUN_10014b40(psVar6);
    uStack_4 = CONCAT31(uStack_4._1_3_,4);
    FUN_10014440((int)(psVar6 + 0x18));
    *(undefined4 *)(psVar6 + 0x6c) = 0;
    *(code **)psVar6 = _vftable__exref;
    srMaterial::reset((srMaterial *)psVar6);
    *(undefined ***)psVar6 = &PTR_FUN_10025d5c;
  }
  uStack_4 = 0xffffffff;
  *(srClass **)((int)this + 8) = psVar6;
  uStack_1c = 0;
  uStack_18 = 0;
  uStack_14 = 0;
  uStack_10 = 0;
  srMaterial::setVector
            ((srMaterial *)psVar6,(srVector4T<float> *)((srMaterial *)psVar6 + 0x28),
             (srVector4T<float> *)&uStack_1c);
  uStack_1c = 0;
  uStack_18 = 0;
  uStack_14 = 0;
  uStack_10 = 0;
  srMaterial::setVector
            (*(srMaterial **)((int)this + 8),
             (srVector4T<float> *)(*(srMaterial **)((int)this + 8) + 0x54),
             (srVector4T<float> *)&uStack_1c);
  uStack_1c = 0;
  uStack_18 = 0;
  uStack_14 = 0;
  uStack_10 = 0;
  srMaterial::setVector
            (*(srMaterial **)((int)this + 8),
             (srVector4T<float> *)(*(srMaterial **)((int)this + 8) + 0x18),
             (srVector4T<float> *)&uStack_1c);
  uStack_1c = 0;
  uStack_18 = 0;
  uStack_14 = 0;
  uStack_10 = 0;
  srMaterial::setVector
            (*(srMaterial **)((int)this + 8),
             (srVector4T<float> *)(*(srMaterial **)((int)this + 8) + 0x38),
             (srVector4T<float> *)&uStack_1c);
  iVar4 = param_1;
  fVar2 = *(float *)(param_1 + 0x40);
  iVar7 = *(int *)((int)this + 8);
  *(float *)(iVar7 + 0x4c) = fVar2;
  if (fVar2 < 1.0) {
    *(undefined4 *)(iVar7 + 0x4c) = 0x3f800000;
  }
  if (127.0 < *(float *)(iVar7 + 0x4c)) {
    *(undefined4 *)(iVar7 + 0x4c) = 0x42fe0000;
  }
  *(undefined4 *)(iVar7 + 0x74) = 1;
  *(undefined4 *)(*(int *)((int)this + 0x14) + 0x18) = *(undefined4 *)((int)this + 8);
  *(undefined4 *)(*(int *)((int)this + 0x14) + 0x20) = *(undefined4 *)((int)this + 0xc);
  *(uint *)(*(int *)((int)this + 0x14) + 0x28) =
       *(uint *)(*(int *)((int)this + 0x14) + 0x28) & 0xfffff3ff;
  *(uint *)(*(int *)((int)this + 0x14) + 0x28) =
       *(uint *)(*(int *)((int)this + 0x14) + 0x28) | 0x8000;
  *(undefined4 *)(*(int *)((int)this + 0x14) + 0x30) = 1;
  uVar11 = 0xffffffff;
  pcVar12 = &DAT_10038520;
  do {
    if (uVar11 == 0) break;
    uVar11 = uVar11 - 1;
    cVar3 = *pcVar12;
    pcVar12 = pcVar12 + 1;
  } while (cVar3 != '\0');
  iVar7 = std::basic_string<>::compare
                    ((basic_string<> *)(param_1 + 0x44),0,*(uint *)(param_1 + 0x4c),&DAT_10038520,
                     ~uVar11 - 1);
  if (iVar7 != 0) {
    param_1 = 0;
    pcVar8 = _C_exref;
    if (*(code **)(iVar4 + 0x48) != (code *)0x0) {
      pcVar8 = *(code **)(iVar4 + 0x48);
    }
    this_00 = srSurfaceIOManager::importSurface
                        ((srSurfaceIOManager *)param_2,(char *)pcVar8,(ImportInfo *)&param_1);
    srRuntimeClass::setName((srRuntimeClass *)this_00,&DAT_10038334);
    psVar9 = (srColorSurface *)srHeap::allocate((srHeap *)srHeap_exref,0x5c);
    uStack_4 = 5;
    param_2 = (srTextureMap *)psVar9;
    if (psVar9 == (srColorSurface *)0x0) {
      psVar9 = (srColorSurface *)0x0;
    }
    else {
      srColorSurface::srColorSurface(psVar9,0xe,0x100,0x100);
      *(undefined ***)psVar9 = &PTR_FUN_10025890;
    }
    uStack_4 = 0xffffffff;
    srRuntimeClass::setName((srRuntimeClass *)psVar9,s_specsurf_1003833c);
    *(code **)(this_00 + 0x2c) = srBSplineFilter_exref;
    uVar11 = *(uint *)(this_00 + 0x28);
    *(uint *)(this_00 + 0x28) = uVar11 & 0xfffffffd;
    *(uint *)(this_00 + 0x28) = uVar11 & 0xfffffffc;
    (**(code **)(*(int *)psVar9 + 0x88))(this_00);
    srClass::release((srClass *)this_00);
    psVar10 = (srTextureMap *)srHeap::allocate((srHeap *)srHeap_exref,0x5c);
    uStack_4 = 6;
    param_2 = psVar10;
    if (psVar10 == (srTextureMap *)0x0) {
      psVar10 = (srTextureMap *)0x0;
    }
    else {
      srTextureMap::srTextureMap(psVar10,(srColorSurfaceIFace *)psVar9);
      *(undefined ***)psVar10 = &PTR_FUN_10025ffc;
    }
    uStack_4 = 0xffffffff;
    *(srTextureMap **)((int)this + 0x10) = psVar10;
    srRuntimeClass::setName((srRuntimeClass *)psVar10,s_diffuse_illumination_10038348);
    srTexture::setMipmap(*(srTexture **)((int)this + 0x10),0);
    *(undefined4 *)(*(int *)((int)this + 0x14) + 0x1c) = *(undefined4 *)((int)this + 8);
    *(undefined4 *)(*(int *)((int)this + 0x14) + 0x24) = *(undefined4 *)((int)this + 0x10);
    *(uint *)(*(int *)((int)this + 0x14) + 0x2c) =
         *(uint *)(*(int *)((int)this + 0x14) + 0x2c) & 0xfffff3ff;
    *(uint *)(*(int *)((int)this + 0x14) + 0x2c) =
         *(uint *)(*(int *)((int)this + 0x14) + 0x2c) | 0x8000;
    *(uint *)(*(int *)((int)this + 0x14) + 0x2c) =
         *(uint *)(*(int *)((int)this + 0x14) + 0x2c) & 0xffff9fff;
    *(uint *)(*(int *)((int)this + 0x14) + 0x2c) =
         *(uint *)(*(int *)((int)this + 0x14) + 0x2c) & 0xffffff5f | 0x40;
    piVar1 = (int *)(*(int *)((int)this + 0x14) + 0x30);
    *piVar1 = *piVar1 + 1;
  }
  ExceptionList = pvStack_c;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Memory_Texture::Set_Buffer(class
// Housemarque::Threedee_Engine::Trimesh *,void *,int,int)

void __fastcall
Housemarque::Threedee_Engine::Memory_Texture::Set_Buffer
          (Trimesh *param_1,void *param_2,int param_3,int param_4)

{
  uint uVar1;
  srColorSurface *this;
  srTextureMap *this_00;
  srModel *psVar2;
  srMeshModel *this_01;
  int iVar3;
  srRuntimeClass *psVar4;
  ulong in_stack_0000000c;
  void *pvStack_c;
  undefined *puStack_8;
  undefined4 local_4;
  
                    // 0x20690  130
                    // ?Set_Buffer@Memory_Texture@Threedee_Engine@Housemarque@@QAIXPAVTrimesh@23@PAXHH@Z
  local_4 = 0xffffffff;
  puStack_8 = &LAB_100243e3;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  this = (srColorSurface *)srHeap::allocate((srHeap *)srHeap_exref,0x5c);
  local_4 = 0;
  psVar4 = (srRuntimeClass *)0x0;
  if (this != (srColorSurface *)0x0) {
    srColorSurface::srColorSurface(this,7,(void *)param_3,param_4,in_stack_0000000c,param_4 * 2);
    *(undefined ***)this = &PTR_FUN_10025890;
    psVar4 = (srRuntimeClass *)this;
  }
  local_4 = 0xffffffff;
  *(srRuntimeClass **)(param_1 + 4) = psVar4;
  srRuntimeClass::setName(psVar4,s_Set_Buffer__new_surface_10038360);
  *(uint *)(*(int *)(param_1 + 4) + 0x28) = *(uint *)(*(int *)(param_1 + 4) + 0x28) | 2;
  *(uint *)(*(int *)(param_1 + 4) + 0x28) = *(uint *)(*(int *)(param_1 + 4) + 0x28) | 1;
  psVar4 = *(srRuntimeClass **)(param_1 + 0xc);
  this_00 = (srTextureMap *)srHeap::allocate((srHeap *)srHeap_exref,0x5c);
  local_4 = 1;
  if (this_00 == (srTextureMap *)0x0) {
    this_00 = (srTextureMap *)0x0;
  }
  else {
    srTextureMap::srTextureMap(this_00,*(srColorSurfaceIFace **)(param_1 + 4));
    *(undefined ***)this_00 = &PTR_FUN_10025ffc;
  }
  local_4 = 0xffffffff;
  *(srTextureMap **)(param_1 + 0xc) = this_00;
  srRuntimeClass::setName((srRuntimeClass *)this_00,s_Set_Buffer__new_diffuse_color_10038378);
  srTexture::setMipmap(*(srTexture **)(param_1 + 0xc),0);
  psVar2 = Trimesh::Get_Sr_Model((int)param_2);
  this_01 = (srMeshModel *)
            __RTDynamicCast(psVar2,0,&srModel::RTTI_Type_Descriptor,
                            &srMeshModel::RTTI_Type_Descriptor,0);
  if (this_01 == (srMeshModel *)0x0) {
    iVar3 = __RTDynamicCast(psVar2,0,&srModel::RTTI_Type_Descriptor,
                            &srStaticIlluminationMeshModel::RTTI_Type_Descriptor,0);
    this_01 = *(srMeshModel **)(iVar3 + 0x1c);
  }
  srMeshModel::setTexture(this_01,*(srTextureIFace **)(param_1 + 0xc),0,0);
  if (((byte)this_01[0x390] & 8) == 0) {
    uVar1 = *(uint *)(this_01 + 0x390);
    *(uint *)(this_01 + 0x390) = uVar1 | 8;
    *(uint *)(this_01 + 0x390) = uVar1 | 8;
  }
  if (psVar4 != (srRuntimeClass *)0x0) {
    srRuntimeClass::setName(psVar4,s_Old_diffuse_10038398);
    srClass::release((srClass *)psVar4);
  }
  ExceptionList = pvStack_c;
  return;
}



// public: void __fastcall Housemarque::Threedee_Engine::Memory_Texture::Update_Texture(void)

void __fastcall Housemarque::Threedee_Engine::Memory_Texture::Update_Texture(void)

{
  int in_ECX;
  
                    // 0x20820  154
                    // ?Update_Texture@Memory_Texture@Threedee_Engine@Housemarque@@QAIXXZ
                    // WARNING: Could not recover jumptable at 0x10020825. Too many branches
                    // WARNING: Treating indirect jump as call
  (**(code **)(**(int **)(in_ECX + 0xc) + 0x44))();
  return;
}



// public: class Housemarque::Threedee_Engine::Material_Node * __fastcall
// Housemarque::Threedee_Engine::Memory_Texture::Get_Material(void)const 

Material_Node * __fastcall Housemarque::Threedee_Engine::Memory_Texture::Get_Material(void)

{
  int in_ECX;
  
                    // 0x20830  80
                    // ?Get_Material@Memory_Texture@Threedee_Engine@Housemarque@@QBIPAVMaterial_Node@23@XZ
  return *(Material_Node **)(in_ECX + 0x14);
}



// public: void __fastcall Housemarque::Threedee_Engine::Memory_Texture::Copy_UVs(class
// Housemarque::Threedee_Engine::Trimesh *,class Housemarque::Template_Library::Vector_3<float>
// const &)

void __fastcall
Housemarque::Threedee_Engine::Memory_Texture::Copy_UVs(Trimesh *param_1,Vector_3<float> *param_2)

{
  uint uVar1;
  srModel *psVar2;
  srMeshModel *this;
  int iVar3;
  srVector2T<float> *psVar4;
  srVector2T<float> *psVar5;
  srVector2T<float> *psVar6;
  float *in_stack_00000004;
  
                    // 0x20840  61
                    // ?Copy_UVs@Memory_Texture@Threedee_Engine@Housemarque@@QAIXPAVTrimesh@23@ABV?$Vector_3@M@Template_Library@3@@Z
  psVar2 = Trimesh::Get_Sr_Model((int)param_2);
  this = (srMeshModel *)
         __RTDynamicCast(psVar2,0,&srModel::RTTI_Type_Descriptor,&srMeshModel::RTTI_Type_Descriptor,
                         0);
  if (this == (srMeshModel *)0x0) {
    iVar3 = __RTDynamicCast(psVar2,0,&srModel::RTTI_Type_Descriptor,
                            &srStaticIlluminationMeshModel::RTTI_Type_Descriptor,0);
    this = *(srMeshModel **)(iVar3 + 0x1c);
  }
  psVar4 = srMeshModel::getVertexTexCoords(this,0,0,1);
  psVar5 = srMeshModel::getVertexTexCoords(this,1,0,1);
  iVar3 = *(int *)(this + 0x22c);
  if (iVar3 != 0) {
    psVar6 = psVar5;
    do {
      *(float *)psVar6 = *(float *)(psVar6 + ((int)psVar4 - (int)psVar5));
      *(float *)(psVar6 + 4) = *(float *)(psVar6 + ((int)psVar4 - (int)psVar5) + 4);
      iVar3 = iVar3 + -1;
      *(float *)psVar6 = *(float *)psVar6 * *in_stack_00000004;
      *(float *)(psVar6 + 4) = *(float *)(psVar6 + 4) * in_stack_00000004[1];
      psVar6 = psVar6 + 8;
    } while (iVar3 != 0);
  }
  if (((byte)this[0x390] & 8) == 0) {
    uVar1 = *(uint *)(this + 0x390);
    *(uint *)(this + 0x390) = uVar1 | 8;
    *(uint *)(this + 0x390) = uVar1 | 8;
  }
  return;
}



void * __thiscall FUN_10020900(void *this,byte param_1)

{
  Housemarque::Threedee_Engine::Memory_Texture_Config::~Memory_Texture_Config
            ((Memory_Texture_Config *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void * __thiscall FUN_10020920(void *this,byte param_1)

{
  Housemarque::Threedee_Engine::Memory_Texture::~Memory_Texture((Memory_Texture *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void __fastcall FUN_10020940(int param_1)

{
  char *pcVar1;
  char cVar2;
  
  if (*(int *)(param_1 + 0xc) != 0) {
    pcVar1 = (char *)(*(int *)(param_1 + 0xc) + -1);
    cVar2 = *pcVar1;
    if ((cVar2 == '\0') || (cVar2 == -1)) {
      FUN_100212a4(pcVar1);
    }
    else {
      *pcVar1 = cVar2 + -1;
    }
  }
  *(undefined4 *)(param_1 + 0xc) = 0;
  *(undefined4 *)(param_1 + 0x10) = 0;
  *(undefined4 *)(param_1 + 0x14) = 0;
  return;
}



void * __thiscall FUN_10020990(void *this,byte param_1)

{
  srTextureMap::~srTextureMap((srTextureMap *)this);
  if ((param_1 & 1) != 0) {
    srHeap::free(this);
  }
  return this;
}



srTextureMap * __fastcall FUN_10020a80(srTextureMap *param_1)

{
  srTextureMap *this;
  
  this = (srTextureMap *)(**(code **)(*(int *)param_1 + 0x20))();
  srTextureMap::operator=(this,param_1);
  return this;
}



void __thiscall srColorSurfaceIFace::copy(srColorSurfaceIFace *this,srColorSurfaceIFace *param_1)

{
                    // WARNING: Could not recover jumptable at 0x10020bcc. Too many branches
                    // WARNING: Treating indirect jump as call
  copy(this,param_1);
  return;
}



ulong __thiscall srColorSurface::getPixelRaw(srColorSurface *this,long param_1,long param_2)

{
  ulong uVar1;
  
                    // WARNING: Could not recover jumptable at 0x10020c50. Too many branches
                    // WARNING: Treating indirect jump as call
  uVar1 = getPixelRaw(this,param_1,param_2);
  return uVar1;
}



void __thiscall
srColorSurfaceIFace::setPixel(srColorSurfaceIFace *this,long param_1,long param_2,ulong param_3)

{
                    // WARNING: Could not recover jumptable at 0x10020c56. Too many branches
                    // WARNING: Treating indirect jump as call
  setPixel(this,param_1,param_2,param_3);
  return;
}



ulong __thiscall srColorSurfaceIFace::getPixel(srColorSurfaceIFace *this,long param_1,long param_2)

{
  ulong uVar1;
  
                    // WARNING: Could not recover jumptable at 0x10020c5c. Too many branches
                    // WARNING: Treating indirect jump as call
  uVar1 = getPixel(this,param_1,param_2);
  return uVar1;
}



void __thiscall srTexture::getDimensions(srTexture *this,Dimensions *param_1)

{
                    // WARNING: Could not recover jumptable at 0x10020efc. Too many branches
                    // WARNING: Treating indirect jump as call
  getDimensions(this,param_1);
  return;
}



void __fastcall Housemarque::Grandma::To_String(void)

{
                    // WARNING: Could not recover jumptable at 0x10021250. Too many branches
                    // WARNING: Treating indirect jump as call
  To_String();
  return;
}



void __fastcall Housemarque::Grandma::To_String(void)

{
                    // WARNING: Could not recover jumptable at 0x10021256. Too many branches
                    // WARNING: Treating indirect jump as call
  To_String();
  return;
}



void __cdecl FUN_100212a4(void *param_1)

{
  free(param_1);
  return;
}



void __cdecl std::operator+(void)

{
                    // WARNING: Could not recover jumptable at 0x10021388. Too many branches
                    // WARNING: Treating indirect jump as call
  operator+();
  return;
}



void __cdecl std::operator+(void)

{
                    // WARNING: Could not recover jumptable at 0x1002138e. Too many branches
                    // WARNING: Treating indirect jump as call
  operator+();
  return;
}



basic_ostream<> * __cdecl std::operator<<(basic_ostream<> *param_1,basic_string<> *param_2)

{
  basic_ostream<> *pbVar1;
  
                    // WARNING: Could not recover jumptable at 0x1002139a. Too many branches
                    // WARNING: Treating indirect jump as call
  pbVar1 = operator<<(param_1,param_2);
  return pbVar1;
}



void __cdecl std::operator+(void)

{
                    // WARNING: Could not recover jumptable at 0x100213f4. Too many branches
                    // WARNING: Treating indirect jump as call
  operator+();
  return;
}



void __cdecl std::operator+(void)

{
                    // WARNING: Could not recover jumptable at 0x10021406. Too many branches
                    // WARNING: Treating indirect jump as call
  operator+();
  return;
}



void __cdecl FUN_10021496(_onexit_t param_1)

{
  if (DAT_10038534 == -1) {
    _onexit(param_1);
    return;
  }
  __dllonexit(param_1,&DAT_10038534,&DAT_10038530);
  return;
}



int __cdecl FUN_100214c2(_onexit_t param_1)

{
  int iVar1;
  
  iVar1 = FUN_10021496(param_1);
  return (iVar1 != 0) - 1;
}



void __CxxFrameHandler(void)

{
                    // WARNING: Could not recover jumptable at 0x100214d4. Too many branches
                    // WARNING: Treating indirect jump as call
  __CxxFrameHandler();
  return;
}



void * __cdecl operator_new(uint param_1)

{
  void *pvVar1;
  
                    // WARNING: Could not recover jumptable at 0x100214e0. Too many branches
                    // WARNING: Treating indirect jump as call
  pvVar1 = operator_new(param_1);
  return pvVar1;
}



void * __thiscall FUN_100214f1(void *this,byte param_1)

{
  type_info::~type_info((type_info *)this);
  if ((param_1 & 1) != 0) {
    FUN_100212a4(this);
  }
  return this;
}



void _ftol(void)

{
                    // WARNING: Could not recover jumptable at 0x10021548. Too many branches
                    // WARNING: Treating indirect jump as call
  _ftol();
  return;
}



void __cdecl free(void *_Memory)

{
                    // WARNING: Could not recover jumptable at 0x10021560. Too many branches
                    // WARNING: Treating indirect jump as call
  free(_Memory);
  return;
}



void _CxxThrowException(void *pExceptionObject,ThrowInfo *pThrowInfo)

{
                    // WARNING: Could not recover jumptable at 0x10021566. Too many branches
                    // WARNING: Subroutine does not return
                    // WARNING: Treating indirect jump as call
  _CxxThrowException(pExceptionObject,pThrowInfo);
  return;
}



// WARNING: Unable to track spacebase fully for stack

void FUN_10021580(void)

{
  uint in_EAX;
  undefined *puVar1;
  undefined4 unaff_retaddr;
  
  puVar1 = &stack0x00000004;
  if (0xfff < in_EAX) {
    do {
      puVar1 = puVar1 + -0x1000;
      in_EAX = in_EAX - 0x1000;
    } while (0xfff < in_EAX);
  }
  *(undefined4 *)(puVar1 + (-4 - in_EAX)) = unaff_retaddr;
  return;
}



void FUN_100215af(undefined4 param_1,undefined4 param_2,int param_3,undefined *param_4)

{
  void *local_14;
  undefined *puStack_10;
  undefined *puStack_c;
  undefined4 local_8;
  
  puStack_c = &DAT_10026058;
  puStack_10 = &DAT_10021880;
  local_14 = ExceptionList;
  local_8 = 0;
  ExceptionList = &local_14;
  while( true ) {
    param_3 = param_3 + -1;
    if (param_3 < 0) break;
    (*(code *)param_4)();
  }
  local_8 = 0xffffffff;
  FUN_10021617();
  ExceptionList = local_14;
  return;
}



void FUN_10021617(void)

{
  int unaff_EBP;
  
  if (*(int *)(unaff_EBP + -0x1c) == 0) {
    FUN_1002162f(*(undefined4 *)(unaff_EBP + 8),*(undefined4 *)(unaff_EBP + 0xc),
                 *(int *)(unaff_EBP + 0x10),*(undefined **)(unaff_EBP + 0x14));
  }
  return;
}



void FUN_1002162f(undefined4 param_1,undefined4 param_2,int param_3,undefined *param_4)

{
  void *local_14;
  undefined *puStack_10;
  undefined *puStack_c;
  undefined4 local_8;
  
  puStack_c = &DAT_10026068;
  puStack_10 = &DAT_10021880;
  local_14 = ExceptionList;
  local_8 = 0;
  ExceptionList = &local_14;
  while( true ) {
    param_3 = param_3 + -1;
    if (param_3 < 0) break;
    (*(code *)param_4)();
  }
  ExceptionList = local_14;
  return;
}



void FUN_100216a3(undefined4 param_1,undefined4 param_2,int param_3,undefined *param_4)

{
  int local_20;
  void *local_14;
  undefined *puStack_10;
  undefined *puStack_c;
  undefined4 local_8;
  
  puStack_c = &DAT_10026078;
  puStack_10 = &DAT_10021880;
  local_14 = ExceptionList;
  local_8 = 0;
  ExceptionList = &local_14;
  for (local_20 = 0; local_20 < param_3; local_20 = local_20 + 1) {
    (*(code *)param_4)();
  }
  local_8 = 0xffffffff;
  FUN_1002170d();
  ExceptionList = local_14;
  return;
}



void FUN_1002170d(void)

{
  int unaff_EBP;
  
  if (*(int *)(unaff_EBP + -0x20) == 0) {
    FUN_1002162f(*(undefined4 *)(unaff_EBP + 8),*(undefined4 *)(unaff_EBP + 0xc),
                 *(int *)(unaff_EBP + -0x1c),*(undefined **)(unaff_EBP + 0x18));
  }
  return;
}



void __RTDynamicCast(void)

{
                    // WARNING: Could not recover jumptable at 0x10021726. Too many branches
                    // WARNING: Treating indirect jump as call
  __RTDynamicCast();
  return;
}



// WARNING: Globals starting with '_' overlap smaller symbols at the same address

undefined4 FUN_1002172c(undefined4 param_1,int param_2)

{
  undefined4 uVar1;
  undefined4 *_Memory;
  undefined4 *puVar2;
  
  if (param_2 == 0) {
    if (0 < DAT_10038524) {
      DAT_10038524 = DAT_10038524 + -1;
      goto LAB_10021742;
    }
LAB_1002176a:
    uVar1 = 0;
  }
  else {
LAB_10021742:
    _DAT_10038528 = *(undefined4 *)_adjust_fdiv_exref;
    if (param_2 == 1) {
      DAT_10038534 = (undefined4 *)malloc(0x80);
      if (DAT_10038534 == (undefined4 *)0x0) goto LAB_1002176a;
      *DAT_10038534 = 0;
      DAT_10038530 = DAT_10038534;
      _initterm(&DAT_10035000,&DAT_1003507c);
      DAT_10038524 = DAT_10038524 + 1;
    }
    else if ((param_2 == 0) &&
            (_Memory = DAT_10038534, puVar2 = DAT_10038530, DAT_10038534 != (undefined4 *)0x0)) {
      while (puVar2 = puVar2 + -1, _Memory <= puVar2) {
        if ((code *)*puVar2 != (code *)0x0) {
          (*(code *)*puVar2)();
          _Memory = DAT_10038534;
        }
      }
      free(_Memory);
      DAT_10038534 = (undefined4 *)0x0;
    }
    uVar1 = 1;
  }
  return uVar1;
}



int entry(HMODULE param_1,int param_2,undefined4 param_3)

{
  int iVar1;
  int iVar2;
  int iVar3;
  
  iVar1 = param_2;
  iVar2 = DAT_10038524;
  if (param_2 != 0) {
    if ((param_2 != 1) && (param_2 != 2)) goto LAB_1002181f;
    if ((DAT_1003852c != (code *)0x0) &&
       (iVar2 = (*DAT_1003852c)(param_1,param_2,param_3), iVar2 == 0)) {
      return 0;
    }
    iVar2 = FUN_1002172c(param_1,param_2);
  }
  if (iVar2 == 0) {
    return 0;
  }
LAB_1002181f:
  iVar2 = FUN_10021898(param_1,param_2);
  if (param_2 == 1) {
    if (iVar2 != 0) {
      return iVar2;
    }
    FUN_1002172c(param_1,0);
  }
  if ((param_2 != 0) && (param_2 != 3)) {
    return iVar2;
  }
  iVar3 = FUN_1002172c(param_1,param_2);
  param_2 = iVar2;
  if (iVar3 == 0) {
    param_2 = 0;
  }
  if (param_2 != 0) {
    if (DAT_1003852c != (code *)0x0) {
      iVar2 = (*DAT_1003852c)(param_1,iVar1,param_3);
      return iVar2;
    }
    return param_2;
  }
  return 0;
}



void __dllonexit(void)

{
                    // WARNING: Could not recover jumptable at 0x10021874. Too many branches
                    // WARNING: Treating indirect jump as call
  __dllonexit();
  return;
}



void __thiscall type_info::~type_info(type_info *this)

{
                    // WARNING: Could not recover jumptable at 0x1002187a. Too many branches
                    // WARNING: Treating indirect jump as call
  ~type_info(this);
  return;
}



void _initterm(void)

{
                    // WARNING: Could not recover jumptable at 0x1002188c. Too many branches
                    // WARNING: Treating indirect jump as call
  _initterm();
  return;
}



undefined4 FUN_10021898(HMODULE param_1,int param_2)

{
  if ((param_2 == 1) && (DAT_1003852c == 0)) {
    DisableThreadLibraryCalls(param_1);
  }
  return 1;
}



void Unwind_100218c0(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(unaff_EBP + 4));
  return;
}



void Unwind_100218e0(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021900(void)

{
  int unaff_EBP;
  
  FUN_10003210((undefined4 *)(*(int *)(unaff_EBP + -0x10) + 4));
  return;
}



void Unwind_1002190b(void)

{
  int unaff_EBP;
  
  FUN_100012f0(*(int *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021913(void)

{
  int unaff_EBP;
  
  FUN_10001e90(*(int *)(unaff_EBP + -0x14) + 0x24);
  return;
}



void Unwind_1002191e(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021933(void)

{
  int unaff_EBP;
  
  FUN_10001e90(*(int *)(unaff_EBP + -0x14) + 0x24);
  return;
}



void Unwind_1002193e(void)

{
  int unaff_EBP;
  
  FUN_100012f0(*(int *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021946(void)

{
  int unaff_EBP;
  
  FUN_10003210((undefined4 *)(*(int *)(unaff_EBP + -0x10) + 4));
  return;
}



void Unwind_1002195b(void)

{
  return;
}



void Unwind_1002196d(void)

{
  int unaff_EBP;
  
  FUN_10001090(*(undefined4 **)(unaff_EBP + -0x18));
  return;
}



void Unwind_10021975(void)

{
  int unaff_EBP;
  
  FUN_10001090(*(undefined4 **)(unaff_EBP + -0x18));
  return;
}



void Unwind_10021987(void)

{
  return;
}



void Unwind_1002198f(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + 4));
  return;
}



void Unwind_100219a4(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x18));
  return;
}



void Unwind_100219af(void)

{
  int unaff_EBP;
  
  FUN_10003210((undefined4 *)(*(int *)(unaff_EBP + -0x18) + 4));
  return;
}



void Unwind_100219c4(void)

{
  return;
}



void Unwind_100219d9(void)

{
  return;
}



void Unwind_100219f0(void)

{
  int unaff_EBP;
  
  FUN_100012f0(*(int *)(unaff_EBP + -0x10));
  return;
}



void Unwind_100219f8(void)

{
  int unaff_EBP;
  
  FUN_10003210((undefined4 *)(*(int *)(unaff_EBP + -0x10) + 4));
  return;
}



void Unwind_10021a10(void)

{
  int unaff_EBP;
  
  FUN_100039e0(*(undefined4 **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021a22(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021a34(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021a50(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021a5b(void)

{
  int unaff_EBP;
  
  FUN_10003210((undefined4 *)(*(int *)(unaff_EBP + -0x10) + 4));
  return;
}



void Unwind_10021a66(void)

{
  int unaff_EBP;
  
  FUN_100012f0(*(int *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021a6e(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021a80(void)

{
  int unaff_EBP;
  
  FUN_100039e0(*(undefined4 **)(unaff_EBP + -0x14));
  return;
}



void Unwind_10021a88(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021a9d(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x50));
  return;
}



void Unwind_10021aa8(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x50));
  return;
}



void Unwind_10021ab3(void)

{
  int unaff_EBP;
  
  FUN_100012f0(*(int *)(unaff_EBP + -0x50));
  return;
}



void Unwind_10021abb(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(unaff_EBP + -0x54));
  return;
}



void Unwind_10021ac3(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(unaff_EBP + -0x50));
  return;
}



void Unwind_10021ad5(void)

{
  int unaff_EBP;
  
  FUN_100039e0(*(undefined4 **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021ae7(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021af2(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021afd(void)

{
  int unaff_EBP;
  
  FUN_100012f0(*(int *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021b05(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(unaff_EBP + -0x18));
  return;
}



void Unwind_10021b0d(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021b1f(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021b40(void)

{
  int unaff_EBP;
  
  FUN_10003210((undefined4 *)(*(int *)(unaff_EBP + -0x10) + 4));
  return;
}



void Unwind_10021b60(void)

{
  int unaff_EBP;
  
  FUN_100039e0(*(undefined4 **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021b72(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021b7d(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021b88(void)

{
  int unaff_EBP;
  
  FUN_100012f0(*(int *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021b90(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(unaff_EBP + -0x18));
  return;
}



void Unwind_10021b98(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021baa(void)

{
  int unaff_EBP;
  
  FUN_100039e0(*(undefined4 **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021bbc(void)

{
  int unaff_EBP;
  
  FUN_10003050(*(int *)(unaff_EBP + -0x14));
  return;
}



void Unwind_10021bc4(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021be0(void)

{
  int unaff_EBP;
  
  FUN_10007750(*(undefined4 **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021c00(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(*(int *)(unaff_EBP + -0x10) + 8));
  return;
}



void Unwind_10021c15(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(*(int *)(unaff_EBP + -0x10) + 8));
  return;
}



void Unwind_10021c2a(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021c35(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(unaff_EBP + 4));
  return;
}



void Unwind_10021c47(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021c60(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021c6b(void)

{
  int unaff_EBP;
  
  FUN_10003210((undefined4 *)(*(int *)(unaff_EBP + -0x10) + 4));
  return;
}



void Unwind_10021c76(void)

{
  int unaff_EBP;
  
  FUN_100012f0(*(int *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021c7e(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021c90(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021ca5(void)

{
  int unaff_EBP;
  
  FUN_10006050((int *)(*(int *)(unaff_EBP + -0x10) + 0xc));
  return;
}



void Unwind_10021cb0(void)

{
  int unaff_EBP;
  
  FUN_100060b0((int *)(*(int *)(unaff_EBP + -0x10) + 0x18));
  return;
}



void Unwind_10021cc5(void)

{
  int unaff_EBP;
  
  FUN_10006050((int *)(*(int *)(unaff_EBP + -0x14) + 0xc));
  return;
}



void Unwind_10021cd0(void)

{
  int unaff_EBP;
  
  FUN_100060b0((int *)(*(int *)(unaff_EBP + -0x14) + 0x18));
  return;
}



void Unwind_10021cdb(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021cf0(void)

{
  int unaff_EBP;
  
  FUN_10006050((int *)(*(int *)(unaff_EBP + -0x14) + 0xc));
  return;
}



void Unwind_10021cfb(void)

{
  int unaff_EBP;
  
  FUN_100060b0((int *)(*(int *)(unaff_EBP + -0x14) + 0x18));
  return;
}



void Unwind_10021d10(void)

{
  int unaff_EBP;
  
  FUN_10006000(unaff_EBP + -0x420);
  return;
}



void Unwind_10021d1b(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x434) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x10021d32. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x408));
    return;
  }
  return;
}



void Unwind_10021d39(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10021d3f. Too many branches
                    // WARNING: Treating indirect jump as call
  srModeler::~srModeler((srModeler *)(unaff_EBP + -0x390));
  return;
}



void Unwind_10021d45(void)

{
  int unaff_EBP;
  
  FUN_10005f30(unaff_EBP + -0x3ac);
  return;
}



void Unwind_10021d50(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + -0x424));
  return;
}



void Unwind_10021d5e(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x434) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x10021d75. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x408));
    return;
  }
  return;
}



void Unwind_10021d7c(void)

{
  int unaff_EBP;
  
  FUN_10005f30(unaff_EBP + -0x3ac);
  return;
}



void Unwind_10021d87(void)

{
  int unaff_EBP;
  
  FUN_10005f70((void *)(unaff_EBP + -0x3ec));
  return;
}



void Unwind_10021d92(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x434) & 4) != 0) {
                    // WARNING: Could not recover jumptable at 0x10021da9. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0x414));
    return;
  }
  return;
}



void Unwind_10021db0(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x434) & 8) != 0) {
                    // WARNING: Could not recover jumptable at 0x10021dc7. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x414));
    return;
  }
  return;
}



void Unwind_10021dd8(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x18) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x10021de9. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + 4));
    return;
  }
  return;
}



void Unwind_10021df0(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x18) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x10021e01. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x14));
    return;
  }
  return;
}



void Unwind_10021e12(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x18) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x10021e23. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + 4));
    return;
  }
  return;
}



void Unwind_10021e2a(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x18) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x10021e3b. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x14));
    return;
  }
  return;
}



void Unwind_10021e4c(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10021e52. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x5a4));
  return;
}



void Unwind_10021e58(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(unaff_EBP + -0x5ac));
  return;
}



void Unwind_10021e63(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x580));
  return;
}



void Unwind_10021e80(void)

{
  int unaff_EBP;
  
  FUN_10001090((undefined4 *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021ea0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10021ea3. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + -0x14));
  return;
}



void Unwind_10021ea9(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10021eac. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + 0xc));
  return;
}



void Unwind_10021ec0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10021ec3. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021ee0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10021ee3. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021f00(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10021f03. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021f20(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10021f23. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021f40(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10021f43. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021f60(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10021f63. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021f80(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10021f83. Too many branches
                    // WARNING: Treating indirect jump as call
  srSurfaceIOManager::SurfaceImporter::~SurfaceImporter(*(SurfaceImporter **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021f93(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10021f96. Too many branches
                    // WARNING: Treating indirect jump as call
  srSurfaceIOManager::SurfaceImporter::~SurfaceImporter(*(SurfaceImporter **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021fa6(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + 8));
  return;
}



void Unwind_10021fb1(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + 8));
  return;
}



void Unwind_10021fd0(void)

{
  int unaff_EBP;
  
  FUN_10007750(*(undefined4 **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021fe2(void)

{
  int unaff_EBP;
  
  FUN_10007750(*(undefined4 **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10021ff4(void)

{
  int unaff_EBP;
  
  FUN_100077e0(*(undefined4 **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10022006(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x10) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x10022017. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + 4));
    return;
  }
  return;
}



void Unwind_10022028(void)

{
  int unaff_EBP;
  
  FUN_100077e0(*(undefined4 **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10022040(void)

{
  int unaff_EBP;
  
  FUN_10009f00(*(int *)(unaff_EBP + -0x10) + 4);
  return;
}



void Unwind_10022060(void)

{
  int unaff_EBP;
  
  FUN_10009820((undefined4 *)(*(int *)(unaff_EBP + -0x14) + 4));
  return;
}



void Unwind_1002206b(void)

{
  int unaff_EBP;
  
  FUN_10009820((undefined4 *)(*(int *)(unaff_EBP + -0x14) + 0x20));
  return;
}



void Unwind_10022076(void)

{
  int unaff_EBP;
  
  FUN_10009f00(*(int *)(unaff_EBP + -0x10) + 4);
  return;
}



void Unwind_10022081(void)

{
  int unaff_EBP;
  
  FUN_10009f00(*(int *)(unaff_EBP + -0x10) + 4);
  return;
}



void Unwind_1002208c(void)

{
  int unaff_EBP;
  
  FUN_10009f00(*(int *)(unaff_EBP + -0x10) + 4);
  return;
}



void Unwind_100220a1(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x28));
  return;
}



void Unwind_100220ac(void)

{
  int unaff_EBP;
  
  FUN_100098d0(*(undefined4 **)(unaff_EBP + -0x28));
  return;
}



void Unwind_100220b4(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x28));
  return;
}



void Unwind_100220bf(void)

{
  int unaff_EBP;
  
  FUN_100098d0(*(undefined4 **)(unaff_EBP + -0x28));
  return;
}



void Unwind_100220c7(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x28));
  return;
}



void Unwind_100220d2(void)

{
  int unaff_EBP;
  
  FUN_100098d0(*(undefined4 **)(unaff_EBP + -0x28));
  return;
}



void Unwind_100220da(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x28));
  return;
}



void Unwind_100220e5(void)

{
  int unaff_EBP;
  
  FUN_100098d0(*(undefined4 **)(unaff_EBP + -0x28));
  return;
}



void Unwind_100220ed(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x28));
  return;
}



void Unwind_100220f8(void)

{
  int unaff_EBP;
  
  FUN_100098d0(*(undefined4 **)(unaff_EBP + -0x28));
  return;
}



void Unwind_10022100(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x30));
  return;
}



void Unwind_1002210b(void)

{
  int unaff_EBP;
  
  FUN_100098d0(*(undefined4 **)(unaff_EBP + -0x30));
  return;
}



void Unwind_10022113(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x30));
  return;
}



void Unwind_1002211e(void)

{
  int unaff_EBP;
  
  FUN_100098d0(*(undefined4 **)(unaff_EBP + -0x30));
  return;
}



void Unwind_10022126(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x38));
  return;
}



void Unwind_10022131(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x38));
  return;
}



void Unwind_1002213c(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x38));
  return;
}



void Unwind_10022151(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022157. Too many branches
                    // WARNING: Treating indirect jump as call
  std::locale::~locale((locale *)(unaff_EBP + -0x1b8));
  return;
}



void Unwind_1002215d(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022163. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1e8));
  return;
}



void Unwind_10022180(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022183. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + -0x60));
  return;
}



void Unwind_10022189(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -100));
  return;
}



void Unwind_10022194(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022197. Too many branches
                    // WARNING: Treating indirect jump as call
  std::ctype_base::~ctype_base(*(ctype_base **)(unaff_EBP + -100));
  return;
}



void Unwind_1002219d(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100221a0. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + 4));
  return;
}



void Unwind_100221a6(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100221a9. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Locinfo::~_Locinfo((_Locinfo *)(unaff_EBP + -0x4c));
  return;
}



void Unwind_100221af(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100221b2. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + 4));
  return;
}



void Unwind_100221d0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100221d3. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + -0x10));
  return;
}



void Unwind_100221f0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100221f6. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(*(int *)(unaff_EBP + -0x20) + 4));
  return;
}



void Unwind_100221fc(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100221ff. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10022205(void)

{
  int unaff_EBP;
  
  FUN_10013f90(*(int *)(unaff_EBP + -0x20) + 0x14);
  return;
}



void Unwind_10022210(void)

{
  int unaff_EBP;
  
  FUN_100147a0(*(int *)(unaff_EBP + -0x20) + 0x34);
  return;
}



void Unwind_10022225(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002222b. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(*(int *)(unaff_EBP + -0x10) + 4));
  return;
}



void Unwind_10022231(void)

{
  int unaff_EBP;
  
  FUN_10013f90(*(int *)(unaff_EBP + -0x10) + 0x14);
  return;
}



void Unwind_1002223c(void)

{
  int unaff_EBP;
  
  FUN_100147a0(*(int *)(unaff_EBP + -0x10) + 0x34);
  return;
}



void Unwind_10022251(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022254. Too many branches
                    // WARNING: Treating indirect jump as call
  srStringTable::~srStringTable((srStringTable *)(unaff_EBP + -0x28));
  return;
}



void Unwind_1002225a(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002225d. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_1002226d(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022270. Too many branches
                    // WARNING: Treating indirect jump as call
  srStringTable::~srStringTable((srStringTable *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022276(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022279. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x30));
  return;
}



void Unwind_1002227f(void)

{
  int unaff_EBP;
  
  FUN_10013f90(unaff_EBP + -0x20);
  return;
}



void Unwind_10022291(void)

{
  int unaff_EBP;
  
  FUN_10013f90(unaff_EBP + 4);
  return;
}



void Unwind_100222a3(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x24) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x100222b4. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0x20));
    return;
  }
  return;
}



void Unwind_100222c5(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_100222da(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x24) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x100222eb. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0x20));
    return;
  }
  return;
}



void Unwind_100222fc(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100222ff. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10022305(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022308. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x2c));
  return;
}



void Unwind_1002230e(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022311. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022321(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022324. Too many branches
                    // WARNING: Treating indirect jump as call
  srStringTable::~srStringTable((srStringTable *)(unaff_EBP + -0x38));
  return;
}



void Unwind_1002232a(void)

{
  int unaff_EBP;
  
  FUN_100147a0(unaff_EBP + -0x1c);
  return;
}



void Unwind_10022332(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022335. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x2c));
  return;
}



void Unwind_10022345(void)

{
  int unaff_EBP;
  
  FUN_100147a0(unaff_EBP + -0x1c);
  return;
}



void Unwind_1002234d(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022350. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x2c));
  return;
}



void Unwind_10022356(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x44) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x10022367. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + 4));
    return;
  }
  return;
}



void Unwind_10022378(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002237b. Too many branches
                    // WARNING: Treating indirect jump as call
  srStringTable::~srStringTable((srStringTable *)(unaff_EBP + -0x38));
  return;
}



void Unwind_10022381(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x50) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x10022392. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x4c));
    return;
  }
  return;
}



void Unwind_10022399(void)

{
  int unaff_EBP;
  
  FUN_100147a0(unaff_EBP + -0x2c);
  return;
}



void Unwind_100223a1(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100223a4. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_100223aa(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x50) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x100223bb. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0x44));
    return;
  }
  return;
}



void Unwind_100223cc(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x24) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x100223dd. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0x20));
    return;
  }
  return;
}



void Unwind_100223ee(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100223f4. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xdc));
  return;
}



void Unwind_100223fa(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022400. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xec));
  return;
}



void Unwind_10022406(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002240c. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xac));
  return;
}



void Unwind_10022412(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022418. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x120));
  return;
}



void Unwind_1002241e(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022424. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xcc));
  return;
}



void Unwind_1002242a(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022430. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xfc));
  return;
}



void Unwind_10022436(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002243c. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x10c));
  return;
}



void Unwind_10022442(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022448. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xbc));
  return;
}



void Unwind_1002244e(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x124) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x10022462. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_ios<>::~basic_ios<>((basic_ios<> *)(unaff_EBP + -0x40));
    return;
  }
  return;
}



void Unwind_10022469(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002246f. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_istream<>::~basic_istream<>((basic_istream<> *)(unaff_EBP + -0x94));
  return;
}



void Unwind_10022475(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002247b. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_filebuf<>::~basic_filebuf<>((basic_filebuf<> *)(unaff_EBP + -0x94));
  return;
}



void Unwind_10022481(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022487. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_ifstream<>::_vbase_destructor_((basic_ifstream<> *)(unaff_EBP + -0x9c));
  return;
}



void Unwind_1002248d(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x124) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x100224a1. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + 4));
    return;
  }
  return;
}



void Unwind_100224b2(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100224b5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x2c));
  return;
}



void Unwind_100224bb(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100224be. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_100224ce(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100224d1. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x2c));
  return;
}



void Unwind_100224d7(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100224da. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_100224ea(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100224ed. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x2c));
  return;
}



void Unwind_100224f3(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100224f6. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10022506(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022509. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x2c));
  return;
}



void Unwind_1002250f(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022512. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10022522(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_1002252d(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + 4));
  return;
}



void Unwind_10022542(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::Config_Base::~Config_Base(*(Config_Base **)(unaff_EBP + -0x54));
  return;
}



void Unwind_1002254a(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002254d. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022553(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022556. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x4c));
  return;
}



void Unwind_1002255c(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002255f. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x58));
  return;
}



void Unwind_10022565(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x50));
  return;
}



void Unwind_10022570(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x54) + 0xc);
  return;
}



void Unwind_1002257b(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002257e. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10022584(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022587. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x2c));
  return;
}



void Unwind_1002258d(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022590. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x50));
  return;
}



void Unwind_10022596(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x58));
  return;
}



void Unwind_100225a1(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x54) + 0x3c);
  return;
}



void Unwind_100225ac(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100225af. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_100225b5(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100225b8. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_100225be(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100225c1. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_100225c7(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x50));
  return;
}



void Unwind_100225dc(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100225e2. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x158));
  return;
}



void Unwind_100225e8(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x164));
  return;
}



void Unwind_100225f6(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x1002260d. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xec));
    return;
  }
  return;
}



void Unwind_10022614(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x1002262b. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xcc));
    return;
  }
  return;
}



void Unwind_10022632(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 4) != 0) {
                    // WARNING: Could not recover jumptable at 0x10022646. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x4c));
    return;
  }
  return;
}



void Unwind_1002264d(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022653. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x164));
  return;
}



void Unwind_10022659(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x15c));
  return;
}



void Unwind_10022667(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x15c));
  return;
}



void Unwind_10022675(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 8) != 0) {
                    // WARNING: Could not recover jumptable at 0x1002268c. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x10c));
    return;
  }
  return;
}



void Unwind_10022693(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 0x10) != 0) {
                    // WARNING: Could not recover jumptable at 0x100226aa. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xac));
    return;
  }
  return;
}



void Unwind_100226b1(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 0x20) != 0) {
                    // WARNING: Could not recover jumptable at 0x100226c8. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x8c));
    return;
  }
  return;
}



void Unwind_100226cf(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x15c));
  return;
}



void Unwind_100226dd(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 0x40) != 0) {
                    // WARNING: Could not recover jumptable at 0x100226f4. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xfc));
    return;
  }
  return;
}



void Unwind_100226fb(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 0x80) != 0) {
                    // WARNING: Could not recover jumptable at 0x10022711. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x2c));
    return;
  }
  return;
}



void Unwind_10022718(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 0x100) != 0) {
                    // WARNING: Could not recover jumptable at 0x1002272e. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x6c));
    return;
  }
  return;
}



void Unwind_10022735(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x15c));
  return;
}



void Unwind_10022743(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 0x200) != 0) {
                    // WARNING: Could not recover jumptable at 0x1002275c. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -300));
    return;
  }
  return;
}



void Unwind_10022763(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 0x400) != 0) {
                    // WARNING: Could not recover jumptable at 0x10022779. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x7c));
    return;
  }
  return;
}



void Unwind_10022780(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 0x800) != 0) {
                    // WARNING: Could not recover jumptable at 0x10022799. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x9c));
    return;
  }
  return;
}



void Unwind_100227a0(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x15c));
  return;
}



void Unwind_100227ae(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 0x1000) != 0) {
                    // WARNING: Could not recover jumptable at 0x100227c7. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x11c));
    return;
  }
  return;
}



void Unwind_100227ce(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 0x2000) != 0) {
                    // WARNING: Could not recover jumptable at 0x100227e4. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
    return;
  }
  return;
}



void Unwind_100227eb(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 0x4000) != 0) {
                    // WARNING: Could not recover jumptable at 0x10022801. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x5c));
    return;
  }
  return;
}



void Unwind_10022808(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x15c));
  return;
}



void Unwind_10022816(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 0x8000) != 0) {
                    // WARNING: Could not recover jumptable at 0x1002282f. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x148));
    return;
  }
  return;
}



void Unwind_10022836(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 0x10000) != 0) {
                    // WARNING: Could not recover jumptable at 0x1002284f. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xdc));
    return;
  }
  return;
}



void Unwind_10022856(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x168) & 0x20000) != 0) {
                    // WARNING: Could not recover jumptable at 0x1002286c. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
    return;
  }
  return;
}



void Unwind_10022873(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x164));
  return;
}



void Unwind_10022881(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022887. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier
            ((CPP_Identifier *)(unaff_EBP + -0xbc));
  return;
}



void Unwind_1002288d(void)

{
  int unaff_EBP;
  
  FUN_100140b0(*(undefined4 **)(unaff_EBP + -0x164));
  return;
}



void Unwind_100228a2(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::Config_Base::~Config_Base(*(Config_Base **)(unaff_EBP + -0xa0));
  return;
}



void Unwind_100228ad(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100228b3. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xe0));
  return;
}



void Unwind_100228b9(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100228bf. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xd0));
  return;
}



void Unwind_100228c5(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100228cb. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0xe4));
  return;
}



void Unwind_100228d1(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xe8));
  return;
}



void Unwind_100228df(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0xa0) + 0xc);
  return;
}



void Unwind_100228ed(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100228f3. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xb0));
  return;
}



void Unwind_100228f9(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100228ff. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xc0));
  return;
}



void Unwind_10022905(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002290b. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0xe4));
  return;
}



void Unwind_10022911(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xe8));
  return;
}



void Unwind_1002291f(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0xa0) + 0x3c);
  return;
}



void Unwind_1002292d(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022933. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xe0));
  return;
}



void Unwind_10022939(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002293f. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xe0));
  return;
}



void Unwind_10022945(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002294b. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xe0));
  return;
}



void Unwind_10022951(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xe4));
  return;
}



void Unwind_1002295f(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0xe8) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x10022973. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_ios<>::~basic_ios<>((basic_ios<> *)(unaff_EBP + -0x40));
    return;
  }
  return;
}



void Unwind_1002297a(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022980. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_istream<>::~basic_istream<>((basic_istream<> *)(unaff_EBP + -0x94));
  return;
}



void Unwind_10022986(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002298c. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_filebuf<>::~basic_filebuf<>((basic_filebuf<> *)(unaff_EBP + -0x94));
  return;
}



void Unwind_10022992(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022998. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_ifstream<>::_vbase_destructor_((basic_ifstream<> *)(unaff_EBP + -0x9c));
  return;
}



void Unwind_1002299e(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0xe8) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x100229b5. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0xe4));
    return;
  }
  return;
}



void Unwind_100229bc(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100229c5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_istream<>::~basic_istream<>((basic_istream<> *)(*(int *)(unaff_EBP + -0xe4) + -0x54));
  return;
}



void Unwind_100229d5(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::Config_Base::~Config_Base(*(Config_Base **)(unaff_EBP + -0x54));
  return;
}



void Unwind_100229dd(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100229e0. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_100229e6(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100229e9. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x4c));
  return;
}



void Unwind_100229ef(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100229f2. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x58));
  return;
}



void Unwind_100229f8(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x50));
  return;
}



void Unwind_10022a03(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x54) + 0xc);
  return;
}



void Unwind_10022a0e(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022a11. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10022a17(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022a1a. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x2c));
  return;
}



void Unwind_10022a20(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022a23. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x50));
  return;
}



void Unwind_10022a29(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x58));
  return;
}



void Unwind_10022a34(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x54) + 0x3c);
  return;
}



void Unwind_10022a3f(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022a42. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022a48(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022a4b. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022a51(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022a54. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022a5a(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x50));
  return;
}



void Unwind_10022a6f(void)

{
  int unaff_EBP;
  
  FUN_10006000(unaff_EBP + -0x24);
  return;
}



void Unwind_10022a77(void)

{
  int unaff_EBP;
  
  srGFInterface::~srGFInterface((srGFInterface *)(unaff_EBP + -0x50));
  return;
}



void Unwind_10022a89(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::Config_Base::~Config_Base(*(Config_Base **)(unaff_EBP + -0xa0));
  return;
}



void Unwind_10022a94(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022a9a. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xec));
  return;
}



void Unwind_10022aa0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022aa6. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xb4));
  return;
}



void Unwind_10022aac(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022ab2. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0xf0));
  return;
}



void Unwind_10022ab8(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xcc));
  return;
}



void Unwind_10022ac6(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0xa0) + 0xc);
  return;
}



void Unwind_10022ad4(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022ada. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xdc));
  return;
}



void Unwind_10022ae0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022ae6. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xc4));
  return;
}



void Unwind_10022aec(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022af2. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0xcc));
  return;
}



void Unwind_10022af8(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xf0));
  return;
}



void Unwind_10022b06(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0xa0) + 0x1c);
  return;
}



void Unwind_10022b14(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022b1a. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xec));
  return;
}



void Unwind_10022b20(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022b26. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xec));
  return;
}



void Unwind_10022b2c(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022b32. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xec));
  return;
}



void Unwind_10022b38(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022b3e. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xec));
  return;
}



void Unwind_10022b44(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022b4a. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xb4));
  return;
}



void Unwind_10022b50(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022b56. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xec));
  return;
}



void Unwind_10022b5c(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022b62. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0xcc));
  return;
}



void Unwind_10022b68(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xf0));
  return;
}



void Unwind_10022b76(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0xa0) + 0x38);
  return;
}



void Unwind_10022b84(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022b8a. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xc4));
  return;
}



void Unwind_10022b90(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022b96. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xdc));
  return;
}



void Unwind_10022b9c(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022ba2. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0xcc));
  return;
}



void Unwind_10022ba8(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xf0));
  return;
}



void Unwind_10022bb6(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0xa0) + 0x48);
  return;
}



void Unwind_10022bc4(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022bca. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xb4));
  return;
}



void Unwind_10022bd0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022bd6. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xec));
  return;
}



void Unwind_10022bdc(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022be2. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0xcc));
  return;
}



void Unwind_10022be8(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xf0));
  return;
}



void Unwind_10022bf6(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0xa0) + 0x58);
  return;
}



void Unwind_10022c04(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022c0a. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xc4));
  return;
}



void Unwind_10022c10(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022c16. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xdc));
  return;
}



void Unwind_10022c1c(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022c22. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0xcc));
  return;
}



void Unwind_10022c28(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xf0));
  return;
}



void Unwind_10022c36(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0xa0) + 0x68);
  return;
}



void Unwind_10022c44(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022c4a. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xec));
  return;
}



void Unwind_10022c50(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022c56. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xec));
  return;
}



void Unwind_10022c5c(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022c62. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xec));
  return;
}



void Unwind_10022c68(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022c6e. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xec));
  return;
}



void Unwind_10022c74(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022c7a. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xec));
  return;
}



void Unwind_10022c80(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022c86. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xb4));
  return;
}



void Unwind_10022c8c(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022c92. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xec));
  return;
}



void Unwind_10022c98(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022c9e. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0xcc));
  return;
}



void Unwind_10022ca4(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xf0));
  return;
}



void Unwind_10022cb2(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0xa0) + 0x84);
  return;
}



void Unwind_10022cc3(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022cc9. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xdc));
  return;
}



void Unwind_10022ccf(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022cd5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xc4));
  return;
}



void Unwind_10022cdb(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022ce1. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xdc));
  return;
}



void Unwind_10022ce7(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022ced. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0xcc));
  return;
}



void Unwind_10022cf3(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xf0));
  return;
}



void Unwind_10022d01(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0xa0) + 0x98);
  return;
}



void Unwind_10022d12(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022d18. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_ifstream<>::_vbase_destructor_((basic_ifstream<> *)(unaff_EBP + -0x9c));
  return;
}



void Unwind_10022d1e(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0xa4) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x10022d35. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0xf0));
    return;
  }
  return;
}



void Unwind_10022d3c(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0xa4) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x10022d53. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0xcc));
    return;
  }
  return;
}



void Unwind_10022d64(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::Config_Base::~Config_Base(*(Config_Base **)(unaff_EBP + -0x54));
  return;
}



void Unwind_10022d6c(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022d6f. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022d75(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022d78. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x4c));
  return;
}



void Unwind_10022d7e(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022d81. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x58));
  return;
}



void Unwind_10022d87(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x50));
  return;
}



void Unwind_10022d92(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x54) + 0xc);
  return;
}



void Unwind_10022d9d(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022da0. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10022da6(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022da9. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x2c));
  return;
}



void Unwind_10022daf(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022db2. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x50));
  return;
}



void Unwind_10022db8(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x58));
  return;
}



void Unwind_10022dc3(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x54) + 0x1c);
  return;
}



void Unwind_10022dce(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022dd1. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022dd7(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022dda. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022de0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022de3. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022de9(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022dec. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022df2(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022df5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x4c));
  return;
}



void Unwind_10022dfb(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022dfe. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022e04(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022e07. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x50));
  return;
}



void Unwind_10022e0d(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x58));
  return;
}



void Unwind_10022e18(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x54) + 0x38);
  return;
}



void Unwind_10022e23(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022e26. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x2c));
  return;
}



void Unwind_10022e2c(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022e2f. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10022e35(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022e38. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x50));
  return;
}



void Unwind_10022e3e(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x58));
  return;
}



void Unwind_10022e49(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x54) + 0x48);
  return;
}



void Unwind_10022e54(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022e57. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x4c));
  return;
}



void Unwind_10022e5d(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022e60. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022e66(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022e69. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x50));
  return;
}



void Unwind_10022e6f(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x58));
  return;
}



void Unwind_10022e7a(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x54) + 0x58);
  return;
}



void Unwind_10022e85(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022e88. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x2c));
  return;
}



void Unwind_10022e8e(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022e91. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10022e97(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022e9a. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x50));
  return;
}



void Unwind_10022ea0(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x58));
  return;
}



void Unwind_10022eab(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x54) + 0x68);
  return;
}



void Unwind_10022eb6(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022eb9. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022ebf(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022ec2. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022ec8(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022ecb. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022ed1(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022ed4. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022eda(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022edd. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022ee3(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022ee6. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x4c));
  return;
}



void Unwind_10022eec(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022eef. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x3c));
  return;
}



void Unwind_10022ef5(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022ef8. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x50));
  return;
}



void Unwind_10022efe(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x58));
  return;
}



void Unwind_10022f09(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x54) + 0x84);
  return;
}



void Unwind_10022f17(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022f1a. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10022f20(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022f23. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x2c));
  return;
}



void Unwind_10022f29(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022f2c. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10022f32(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x54) + 0x98);
  return;
}



void Unwind_10022f4a(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::Config_Base::~Config_Base(*(Config_Base **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10022f52(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x10) + 0xc);
  return;
}



void Unwind_10022f5d(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x10) + 0x1c);
  return;
}



void Unwind_10022f68(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x10) + 0x38);
  return;
}



void Unwind_10022f73(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x10) + 0x48);
  return;
}



void Unwind_10022f7e(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x10) + 0x58);
  return;
}



void Unwind_10022f89(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x10) + 0x68);
  return;
}



void Unwind_10022f94(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x10) + 0x84);
  return;
}



void Unwind_10022fac(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022faf. Too many branches
                    // WARNING: Treating indirect jump as call
  srModelInstance::~srModelInstance(*(srModelInstance **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10022fbf(void)

{
  int unaff_EBP;
  
  FUN_100148f0(*(srModelInstance **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10022fc7(void)

{
  int unaff_EBP;
  
  FUN_100149f0(*(int *)(unaff_EBP + -0x10) + 0x16c);
  return;
}



void Unwind_10022fd5(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10022fd8. Too many branches
                    // WARNING: Treating indirect jump as call
  srModelInstance::~srModelInstance(*(srModelInstance **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10022fe8(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10022ffd(void)

{
  return;
}



void Unwind_10023008(void)

{
  return;
}



void Unwind_1002301d(void)

{
  return;
}



void Unwind_10023028(void)

{
  return;
}



void Unwind_10023033(void)

{
  return;
}



void Unwind_1002303e(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x2c) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x1002304f. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0x1c));
    return;
  }
  return;
}



void Unwind_10023056(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x2c) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023067. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0x14));
    return;
  }
  return;
}



void Unwind_1002306e(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x2c) & 4) != 0) {
                    // WARNING: Could not recover jumptable at 0x1002307f. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0x14));
    return;
  }
  return;
}



void Unwind_10023086(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x2c) & 8) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023097. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0x1c));
    return;
  }
  return;
}



void Unwind_1002309e(void)

{
  return;
}



void Unwind_100230a6(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x2c) & 0x10) != 0) {
                    // WARNING: Could not recover jumptable at 0x100230b7. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0x14));
    return;
  }
  return;
}



void Unwind_100230be(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x2c) & 0x20) != 0) {
                    // WARNING: Could not recover jumptable at 0x100230cf. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0x14));
    return;
  }
  return;
}



void Unwind_100230e0(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_100230f5(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100230fb. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xbc));
  return;
}



void Unwind_10023101(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xf8));
  return;
}



void Unwind_1002310f(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0xf4) & 0x10) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023129. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_ios<>::~basic_ios<>((basic_ios<> *)(*(int *)(unaff_EBP + -0xf8) + 0x58));
    return;
  }
  return;
}



void Unwind_10023130(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023139. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_iostream<>::~basic_iostream<>((basic_iostream<> *)(*(int *)(unaff_EBP + -0xf8) + 0xc));
  return;
}



void Unwind_1002313f(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0xf4) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023156. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0xf8));
    return;
  }
  return;
}



void Unwind_1002315d(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023163. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xac));
  return;
}



void Unwind_10023169(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0xf4) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023180. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0xf8));
    return;
  }
  return;
}



void Unwind_10023187(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0xf4) & 0x20) != 0) {
                    // WARNING: Could not recover jumptable at 0x1002319b. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_ios<>::~basic_ios<>((basic_ios<> *)(unaff_EBP + -0x40));
    return;
  }
  return;
}



void Unwind_100231a2(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100231a8. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_istream<>::~basic_istream<>((basic_istream<> *)(unaff_EBP + -0x94));
  return;
}



void Unwind_100231ae(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100231b4. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_filebuf<>::~basic_filebuf<>((basic_filebuf<> *)(unaff_EBP + -0x94));
  return;
}



void Unwind_100231ba(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100231c0. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_ifstream<>::_vbase_destructor_((basic_ifstream<> *)(unaff_EBP + -0x9c));
  return;
}



void Unwind_100231c6(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0xf4) & 4) != 0) {
                    // WARNING: Could not recover jumptable at 0x100231dd. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0xf8));
    return;
  }
  return;
}



void Unwind_100231e4(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100231ed. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_istream<>::~basic_istream<>((basic_istream<> *)(*(int *)(unaff_EBP + -0xf4) + -0x54));
  return;
}



void Unwind_100231f3(void)

{
  int unaff_EBP;
  
  FUN_10014a80((undefined4 *)(unaff_EBP + -0xe0));
  return;
}



void Unwind_100231fe(void)

{
  int unaff_EBP;
  
  FUN_100143f0(unaff_EBP + -0xe8);
  return;
}



void Unwind_10023209(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + -0xf8));
  return;
}



void Unwind_10023217(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0xf4) & 8) != 0) {
                    // WARNING: Could not recover jumptable at 0x1002322e. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0xac));
    return;
  }
  return;
}



void Unwind_10023235(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002323b. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xac));
  return;
}



void Unwind_1002324b(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x104) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x1002325f. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0x14));
    return;
  }
  return;
}



void Unwind_10023270(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0xa8) & 4) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023284. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_ios<>::~basic_ios<>((basic_ios<> *)(unaff_EBP + -0x40));
    return;
  }
  return;
}



void Unwind_1002328b(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023291. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_istream<>::~basic_istream<>((basic_istream<> *)(unaff_EBP + -0x94));
  return;
}



void Unwind_10023297(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002329d. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_filebuf<>::~basic_filebuf<>((basic_filebuf<> *)(unaff_EBP + -0x94));
  return;
}



void Unwind_100232a3(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100232a9. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_ifstream<>::_vbase_destructor_((basic_ifstream<> *)(unaff_EBP + -0x9c));
  return;
}



void Unwind_100232af(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0xa8) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x100232c6. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0xac));
    return;
  }
  return;
}



void Unwind_100232cd(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0xa8) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x100232e4. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0xac));
    return;
  }
  return;
}



void Unwind_100232eb(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100232f4. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_istream<>::~basic_istream<>((basic_istream<> *)(*(int *)(unaff_EBP + -0xa0) + -0x54));
  return;
}



void Unwind_100232fa(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023300. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_streambuf<>::~basic_streambuf<>(*(basic_streambuf<> **)(unaff_EBP + -0xa4));
  return;
}



void Unwind_10023306(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002330f. Too many branches
                    // WARNING: Treating indirect jump as call
  std::locale::~locale((locale *)(*(int *)(unaff_EBP + -0xa4) + 0x4c));
  return;
}



void Unwind_1002331f(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023325. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Legacy::Segment::~Segment((Segment *)(unaff_EBP + -0x154));
  return;
}



void Unwind_1002332b(void)

{
  int unaff_EBP;
  
  Housemarque::Threedee_Engine::Material_Config::~Material_Config
            ((Material_Config *)(unaff_EBP + -0xb4));
  return;
}



void Unwind_10023340(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + -0x2c));
  return;
}



void Unwind_1002334b(void)

{
  int unaff_EBP;
  
  FUN_10014cd0(*(srClass **)(unaff_EBP + -0x2c));
  return;
}



void Unwind_10023353(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x4c) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023364. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0x2c));
    return;
  }
  return;
}



void Unwind_1002336b(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x4c) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x1002337c. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0x2c));
    return;
  }
  return;
}



void Unwind_10023383(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + -0x2c));
  return;
}



void Unwind_1002338e(void)

{
  int unaff_EBP;
  
  FUN_10014cd0(*(srClass **)(unaff_EBP + -0x2c));
  return;
}



void Unwind_100233a0(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x44));
  return;
}



void Unwind_100233ab(void)

{
  return;
}



void Unwind_100233b3(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x44));
  return;
}



void Unwind_100233be(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x4c) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x100233cf. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x44));
    return;
  }
  return;
}



void Unwind_100233d6(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x4c) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x100233e7. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x44));
    return;
  }
  return;
}



void Unwind_100233ee(void)

{
  int unaff_EBP;
  
  FUN_10006000(unaff_EBP + -0x3c);
  return;
}



void Unwind_10023400(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + 4));
  return;
}



void Unwind_10023420(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023423. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier
            ((CPP_Identifier *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10023429(void)

{
  int unaff_EBP;
  
  FUN_100140b0(*(undefined4 **)(unaff_EBP + -0x20));
  return;
}



void Unwind_10023440(void)

{
  int unaff_EBP;
  
  if (*(int *)(unaff_EBP + -0x10) == 0) {
    *(undefined4 *)(unaff_EBP + -4) = 0;
  }
  else {
    *(int *)(unaff_EBP + -4) = *(int *)(unaff_EBP + -0x10) + 4;
  }
  return;
}



void Unwind_10023480(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023483. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_100234a0(void)

{
  int unaff_EBP;
  
  if (*(int *)(unaff_EBP + -0x10) == 0) {
    *(undefined4 *)(unaff_EBP + -4) = 0;
  }
  else {
    *(int *)(unaff_EBP + -4) = *(int *)(unaff_EBP + -0x10) + 4;
  }
  return;
}



void Unwind_100234e0(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + 0xc));
  return;
}



void Unwind_100234eb(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100234ee. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier
            ((CPP_Identifier *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_100234f4(void)

{
  int unaff_EBP;
  
  FUN_100140b0(*(undefined4 **)(unaff_EBP + 0xc));
  return;
}



void Unwind_10023510(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + 0xc));
  return;
}



void Unwind_1002351b(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002351e. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier
            ((CPP_Identifier *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10023524(void)

{
  int unaff_EBP;
  
  FUN_100140b0(*(undefined4 **)(unaff_EBP + 0xc));
  return;
}



void Unwind_10023540(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + 0xc));
  return;
}



void Unwind_1002354b(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002354e. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier
            ((CPP_Identifier *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10023554(void)

{
  int unaff_EBP;
  
  FUN_100140b0(*(undefined4 **)(unaff_EBP + 0xc));
  return;
}



void Unwind_10023570(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023573. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x20));
  return;
}



void Unwind_10023579(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + 0xc));
  return;
}



void Unwind_10023584(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023587. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier
            ((CPP_Identifier *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_1002358d(void)

{
  int unaff_EBP;
  
  FUN_100140b0(*(undefined4 **)(unaff_EBP + 0xc));
  return;
}



void Unwind_100235a0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100235a3. Too many branches
                    // WARNING: Treating indirect jump as call
  srModelInstance::~srModelInstance(*(srModelInstance **)(unaff_EBP + -0x10));
  return;
}



void Unwind_100235c0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100235c3. Too many branches
                    // WARNING: Treating indirect jump as call
  srClass::~srClass(*(srClass **)(unaff_EBP + -0x10));
  return;
}



void Unwind_100235c9(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100235cc. Too many branches
                    // WARNING: Treating indirect jump as call
  srMaterialIFace::~srMaterialIFace(*(srMaterialIFace **)(unaff_EBP + -0x10));
  return;
}



void Unwind_100235e0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100235e3. Too many branches
                    // WARNING: Treating indirect jump as call
  srMaterialIFace::~srMaterialIFace(*(srMaterialIFace **)(unaff_EBP + -0x10));
  return;
}



void Unwind_100235e9(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100235ec. Too many branches
                    // WARNING: Treating indirect jump as call
  srClass::~srClass(*(srClass **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10023600(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023603. Too many branches
                    // WARNING: Treating indirect jump as call
  srClass::~srClass(*(srClass **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10023620(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023623. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Template_Library::CPP_Identifier::~CPP_Identifier
            ((CPP_Identifier *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10023629(void)

{
  int unaff_EBP;
  
  FUN_100140b0(*(undefined4 **)(unaff_EBP + -0x20));
  return;
}



void Unwind_10023640(void)

{
  int unaff_EBP;
  
  if (*(int *)(unaff_EBP + -0x10) == 0) {
    *(undefined4 *)(unaff_EBP + -0x14) = 0;
  }
  else {
    *(int *)(unaff_EBP + -0x14) = *(int *)(unaff_EBP + -0x10) + 4;
  }
  return;
}



void Unwind_10023680(void)

{
  int unaff_EBP;
  
  if (*(int *)(unaff_EBP + -0x10) == 0) {
    *(undefined4 *)(unaff_EBP + -0x14) = 0;
  }
  else {
    *(int *)(unaff_EBP + -0x14) = *(int *)(unaff_EBP + -0x10) + 4;
  }
  return;
}



void Unwind_100236c0(void)

{
  int unaff_EBP;
  
  if (*(int *)(unaff_EBP + -0x10) == 0) {
    *(undefined4 *)(unaff_EBP + -0x14) = 0;
  }
  else {
    *(int *)(unaff_EBP + -0x14) = *(int *)(unaff_EBP + -0x10) + 4;
  }
  return;
}



void Unwind_10023700(void)

{
  int unaff_EBP;
  
  if (*(int *)(unaff_EBP + -0x10) == 0) {
    *(undefined4 *)(unaff_EBP + -0x14) = 0;
  }
  else {
    *(int *)(unaff_EBP + -0x14) = *(int *)(unaff_EBP + -0x10) + 4;
  }
  return;
}



void Unwind_10023740(void)

{
  FUN_100166d0();
  return;
}



void Unwind_10023760(void)

{
  FUN_100166d0();
  return;
}



void Unwind_10023780(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10023795(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x20) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x100237a6. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0x1c));
    return;
  }
  return;
}



void Unwind_100237ad(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x20) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x100237be. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Error_Log_Inserter::~Error_Log_Inserter
              ((Error_Log_Inserter *)(unaff_EBP + -0x14));
    return;
  }
  return;
}



void Unwind_100237cf(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::Config_Base::~Config_Base(*(Config_Base **)(unaff_EBP + -0x132c));
  return;
}



void Unwind_100237da(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100237e0. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x133c));
  return;
}



void Unwind_100237e6(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100237ec. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1328));
  return;
}



void Unwind_100237f2(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100237f8. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x1350));
  return;
}



void Unwind_100237fe(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x1344));
  return;
}



void Unwind_1002380c(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x132c) + 0xc);
  return;
}



void Unwind_1002381a(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023820. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x130c));
  return;
}



void Unwind_10023826(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002382c. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x12fc));
  return;
}



void Unwind_10023832(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023838. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x1350));
  return;
}



void Unwind_1002383e(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x1344));
  return;
}



void Unwind_1002384c(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x132c) + 0x1c);
  return;
}



void Unwind_1002385a(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023860. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x133c));
  return;
}



void Unwind_10023866(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002386c. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1328));
  return;
}



void Unwind_10023872(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023878. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x1350));
  return;
}



void Unwind_1002387e(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x1344));
  return;
}



void Unwind_1002388c(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x132c) + 0x2c);
  return;
}



void Unwind_1002389a(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::HTSC_Table_Base::~HTSC_Table_Base
            ((HTSC_Table_Base *)(*(int *)(unaff_EBP + -0x132c) + 0x40));
  return;
}



void Unwind_100238a8(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::HTSC_Table_Base::~HTSC_Table_Base
            ((HTSC_Table_Base *)(*(int *)(unaff_EBP + -0x132c) + 0x48));
  return;
}



void Unwind_100238b6(void)

{
  int unaff_EBP;
  
  FUN_10006000(unaff_EBP + -0x1318);
  return;
}



void Unwind_100238c1(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x134c) & 0x40) != 0) {
                    // WARNING: Could not recover jumptable at 0x100238d8. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_ios<>::~basic_ios<>((basic_ios<> *)(unaff_EBP + -0x1290));
    return;
  }
  return;
}



void Unwind_100238df(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100238e5. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_istream<>::~basic_istream<>((basic_istream<> *)(unaff_EBP + -0x12e4));
  return;
}



void Unwind_100238eb(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100238f1. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_filebuf<>::~basic_filebuf<>((basic_filebuf<> *)(unaff_EBP + -0x12e4));
  return;
}



void Unwind_100238f7(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100238fd. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_ifstream<>::_vbase_destructor_((basic_ifstream<> *)(unaff_EBP + -0x12ec));
  return;
}



void Unwind_10023903(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x134c) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x1002391a. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x1340));
    return;
  }
  return;
}



void Unwind_10023921(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023927. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Legacy::Segment::~Segment((Segment *)(unaff_EBP + -0x11bc));
  return;
}



void Unwind_1002392d(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x134c) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023944. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x1340));
    return;
  }
  return;
}



void Unwind_1002394b(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023951. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Legacy::Segment::~Segment((Segment *)(unaff_EBP + -0x111c));
  return;
}



void Unwind_10023957(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002395d. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Legacy::Segment::~Segment((Segment *)(unaff_EBP + -0x125c));
  return;
}



void Unwind_10023963(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x134c) & 4) != 0) {
                    // WARNING: Could not recover jumptable at 0x1002397a. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x1340));
    return;
  }
  return;
}



void Unwind_10023981(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x134c) & 8) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023998. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x1344));
    return;
  }
  return;
}



void Unwind_1002399f(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100239a5. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Legacy::Segment::~Segment((Segment *)(unaff_EBP + -0x107c));
  return;
}



void Unwind_100239ab(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x134c) & 0x10) != 0) {
                    // WARNING: Could not recover jumptable at 0x100239c2. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x1350));
    return;
  }
  return;
}



void Unwind_100239c9(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x1348));
  return;
}



void Unwind_100239d7(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x134c) & 0x20) != 0) {
                    // WARNING: Could not recover jumptable at 0x100239ee. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x1350));
    return;
  }
  return;
}



void Unwind_100239f5(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100239fb. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Legacy::Segment::~Segment((Segment *)(unaff_EBP + -0x125c));
  return;
}



void Unwind_10023a01(void)

{
  int unaff_EBP;
  
  FUN_10019400((Config_Base *)(unaff_EBP + -0xfdc));
  return;
}



void Unwind_10023a0c(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x1350));
  return;
}



void Unwind_10023a1a(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023a23. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_istream<>::~basic_istream<>((basic_istream<> *)(*(int *)(unaff_EBP + -0x1350) + -0x54))
  ;
  return;
}



void Unwind_10023a29(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023a2f. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_streambuf<>::~basic_streambuf<>(*(basic_streambuf<> **)(unaff_EBP + -0x1344));
  return;
}



void Unwind_10023a35(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023a3e. Too many branches
                    // WARNING: Treating indirect jump as call
  std::locale::~locale((locale *)(*(int *)(unaff_EBP + -0x1344) + 0x4c));
  return;
}



void Unwind_10023a4e(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::Config_Base::~Config_Base(*(Config_Base **)(unaff_EBP + -0x49c));
  return;
}



void Unwind_10023a59(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023a5f. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x48c));
  return;
}



void Unwind_10023a65(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023a6b. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x47c));
  return;
}



void Unwind_10023a71(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023a77. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x4a4));
  return;
}



void Unwind_10023a7d(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x494));
  return;
}



void Unwind_10023a8b(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x49c) + 0xc);
  return;
}



void Unwind_10023a99(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023a9f. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x460));
  return;
}



void Unwind_10023aa5(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023aab. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x450));
  return;
}



void Unwind_10023ab1(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023ab7. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x4a4));
  return;
}



void Unwind_10023abd(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x494));
  return;
}



void Unwind_10023acb(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x49c) + 0x1c);
  return;
}



void Unwind_10023ad9(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023adf. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x48c));
  return;
}



void Unwind_10023ae5(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023aeb. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x47c));
  return;
}



void Unwind_10023af1(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023af7. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x4a4));
  return;
}



void Unwind_10023afd(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x494));
  return;
}



void Unwind_10023b0b(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x49c) + 0x2c);
  return;
}



void Unwind_10023b19(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::HTSC_Table_Base::~HTSC_Table_Base
            ((HTSC_Table_Base *)(*(int *)(unaff_EBP + -0x49c) + 0x40));
  return;
}



void Unwind_10023b27(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::HTSC_Table_Base::~HTSC_Table_Base
            ((HTSC_Table_Base *)(*(int *)(unaff_EBP + -0x49c) + 0x48));
  return;
}



void Unwind_10023b35(void)

{
  int unaff_EBP;
  
  FUN_10006000(unaff_EBP + -0x46c);
  return;
}



void Unwind_10023b40(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x4a0) & 0x40) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023b57. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_ios<>::~basic_ios<>((basic_ios<> *)(unaff_EBP + -0x3e4));
    return;
  }
  return;
}



void Unwind_10023b5e(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023b64. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_istream<>::~basic_istream<>((basic_istream<> *)(unaff_EBP + -0x438));
  return;
}



void Unwind_10023b6a(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023b70. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_filebuf<>::~basic_filebuf<>((basic_filebuf<> *)(unaff_EBP + -0x438));
  return;
}



void Unwind_10023b76(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023b7c. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_ifstream<>::_vbase_destructor_((basic_ifstream<> *)(unaff_EBP + -0x440));
  return;
}



void Unwind_10023b82(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x4a0) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023b99. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x490));
    return;
  }
  return;
}



void Unwind_10023ba0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023ba6. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Legacy::Segment::~Segment((Segment *)(unaff_EBP + -0x38c));
  return;
}



void Unwind_10023bac(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x4a0) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023bc3. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x490));
    return;
  }
  return;
}



void Unwind_10023bca(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023bd0. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Legacy::Segment::~Segment((Segment *)(unaff_EBP + -0x2ec));
  return;
}



void Unwind_10023bd6(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023bdc. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Legacy::Segment::~Segment((Segment *)(unaff_EBP + -0xac));
  return;
}



void Unwind_10023be2(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x4a0) & 4) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023bf9. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x490));
    return;
  }
  return;
}



void Unwind_10023c00(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x4a0) & 8) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023c17. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x494));
    return;
  }
  return;
}



void Unwind_10023c1e(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023c24. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Legacy::Segment::~Segment((Segment *)(unaff_EBP + -0x14c));
  return;
}



void Unwind_10023c2a(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x4a0) & 0x10) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023c41. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x4a4));
    return;
  }
  return;
}



void Unwind_10023c48(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x498));
  return;
}



void Unwind_10023c56(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x4a0) & 0x20) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023c6d. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x4a4));
    return;
  }
  return;
}



void Unwind_10023c74(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x4a4));
  return;
}



void Unwind_10023c82(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023c8b. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_istream<>::~basic_istream<>((basic_istream<> *)(*(int *)(unaff_EBP + -0x4a4) + -0x54));
  return;
}



void Unwind_10023c91(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023c97. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_streambuf<>::~basic_streambuf<>(*(basic_streambuf<> **)(unaff_EBP + -0x494));
  return;
}



void Unwind_10023c9d(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023ca6. Too many branches
                    // WARNING: Treating indirect jump as call
  std::locale::~locale((locale *)(*(int *)(unaff_EBP + -0x494) + 0x4c));
  return;
}



void Unwind_10023cb6(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::Config_Base::~Config_Base(*(Config_Base **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10023cbe(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x10) + 0xc);
  return;
}



void Unwind_10023cc9(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x10) + 0x1c);
  return;
}



void Unwind_10023cd4(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x10) + 0x2c);
  return;
}



void Unwind_10023cdf(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::HTSC_Table_Base::~HTSC_Table_Base
            ((HTSC_Table_Base *)(*(int *)(unaff_EBP + -0x10) + 0x40));
  return;
}



void Unwind_10023cea(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::HTSC_Table_Base::~HTSC_Table_Base
            ((HTSC_Table_Base *)(*(int *)(unaff_EBP + -0x10) + 0x48));
  return;
}



void Unwind_10023cff(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023d02. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Template_Library::HTSC_Node_String::~HTSC_Node_String
            ((HTSC_Node_String *)(unaff_EBP + -0x28));
  return;
}



void Unwind_10023d12(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023d15. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Template_Library::HTSC_Node_String::~HTSC_Node_String
            ((HTSC_Node_String *)(unaff_EBP + -0x28));
  return;
}



void Unwind_10023d25(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::Config_Base::~Config_Base(*(Config_Base **)(unaff_EBP + -0x1c0));
  return;
}



void Unwind_10023d30(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023d36. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x188));
  return;
}



void Unwind_10023d3c(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023d42. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1b8));
  return;
}



void Unwind_10023d48(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023d4e. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x1c4));
  return;
}



void Unwind_10023d54(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x1cc));
  return;
}



void Unwind_10023d62(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x1c0) + 0xc);
  return;
}



void Unwind_10023d70(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023d76. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1a8));
  return;
}



void Unwind_10023d7c(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023d82. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x198));
  return;
}



void Unwind_10023d88(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023d8e. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0x1cc));
  return;
}



void Unwind_10023d94(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x1bc));
  return;
}



void Unwind_10023da2(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0x1c0) + 0x1c);
  return;
}



void Unwind_10023db0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023db6. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Legacy::Segment::~Segment((Segment *)(unaff_EBP + -0xac));
  return;
}



void Unwind_10023dbc(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x1c8) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023dd3. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x1cc));
    return;
  }
  return;
}



void Unwind_10023dda(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x1bc));
  return;
}



void Unwind_10023de8(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x1c8) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023dff. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x1c4));
    return;
  }
  return;
}



void Unwind_10023e06(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023e0c. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Legacy::Segment::~Segment((Segment *)(unaff_EBP + -0x14c));
  return;
}



void Unwind_10023e12(void)

{
  int unaff_EBP;
  
  FUN_10019d80((Config_Base *)(unaff_EBP + -0x16c));
  return;
}



void Unwind_10023e1d(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023e23. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1b8));
  return;
}



void Unwind_10023e29(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023e2f. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Template_Library::HTSC_Node_String::~HTSC_Node_String
            ((HTSC_Node_String *)(unaff_EBP + -0x188));
  return;
}



void Unwind_10023e35(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x1bc));
  return;
}



void Unwind_10023e4d(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023e53. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Template_Library::HTSC_Node_String::~HTSC_Node_String
            (*(HTSC_Node_String **)(unaff_EBP + -0x110));
  return;
}



void Unwind_10023e59(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x114));
  return;
}



void Unwind_10023e67(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x130) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023e7e. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x128));
    return;
  }
  return;
}



void Unwind_10023e8f(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023e92. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Template_Library::HTSC_Node_String::~HTSC_Node_String
            (*(HTSC_Node_String **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10023ea2(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::Config_Base::~Config_Base(*(Config_Base **)(unaff_EBP + -0xb4));
  return;
}



void Unwind_10023ead(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023eb3. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xe4));
  return;
}



void Unwind_10023eb9(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023ebf. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xf4));
  return;
}



void Unwind_10023ec5(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023ecb. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0xf8));
  return;
}



void Unwind_10023ed1(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xb0));
  return;
}



void Unwind_10023edf(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0xb4) + 0xc);
  return;
}



void Unwind_10023eed(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023ef3. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xd4));
  return;
}



void Unwind_10023ef9(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023eff. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xc4));
  return;
}



void Unwind_10023f05(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023f0b. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Legacy::Segment::~Segment((Segment *)(unaff_EBP + -0xac));
  return;
}



void Unwind_10023f1b(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023f1e. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Template_Library::HTSC_Node_String::~HTSC_Node_String
            (*(HTSC_Node_String **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10023f24(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023f2a. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(*(int *)(unaff_EBP + -0x10) + 0x24));
  return;
}



void Unwind_10023f3a(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023f3d. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10023f43(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023f46. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Template_Library::HTSC_Node_String::~HTSC_Node_String
            (*(HTSC_Node_String **)(unaff_EBP + -0x2c));
  return;
}



void Unwind_10023f4c(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023f52. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(*(int *)(unaff_EBP + -0x2c) + 0x24));
  return;
}



void Unwind_10023f58(void)

{
  int unaff_EBP;
  
  FUN_10006000(unaff_EBP + -0x28);
  return;
}



void Unwind_10023f6a(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023f6d. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Template_Library::HTSC_Node_String::~HTSC_Node_String
            (*(HTSC_Node_String **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10023f73(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023f79. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(*(int *)(unaff_EBP + -0x10) + 0x24));
  return;
}



void Unwind_10023f90(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10023f93. Too many branches
                    // WARNING: Treating indirect jump as call
  Housemarque::Bone_Import::Skin::~Skin(*(Skin **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10023fa3(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x14) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x10023fb4. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0x10));
    return;
  }
  return;
}



void Unwind_10023fc5(void)

{
  int unaff_EBP;
  
  FUN_1001aac0(*(int *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10023fd7(void)

{
  int unaff_EBP;
  
  FUN_1001aac0(*(int *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10023fdf(void)

{
  int unaff_EBP;
  
  Housemarque::Threedee_Engine::Trimesh::~Trimesh((Trimesh *)(*(int *)(unaff_EBP + -0x10) + 0x10));
  return;
}



void Unwind_10024000(void)

{
  int unaff_EBP;
  
  FUN_1001b2d0((void *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10024012(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0x40) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x10024023. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + 0x1c));
    return;
  }
  return;
}



void Unwind_10024040(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x2c));
  return;
}



void Unwind_1002404b(void)

{
  int unaff_EBP;
  
  FUN_1001d400((undefined4 *)(unaff_EBP + -0x1c));
  return;
}



void Unwind_10024053(void)

{
  int unaff_EBP;
  
  FUN_1001c350(unaff_EBP + -0x28);
  return;
}



void Unwind_10024070(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10024073. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + -0x14));
  return;
}



void Unwind_10024079(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002407c. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + 0xc));
  return;
}



void Unwind_10024090(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10024093. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + -0x10));
  return;
}



void Unwind_100240b0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100240b3. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + -0x10));
  return;
}



void Unwind_100240d0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100240d3. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + -0x10));
  return;
}



void Unwind_100240f0(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100240f3. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10024110(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10024113. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10024130(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10024133. Too many branches
                    // WARNING: Treating indirect jump as call
  std::_Lockit::~_Lockit((_Lockit *)(unaff_EBP + -0x10));
  return;
}



void Unwind_10024150(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10024153. Too many branches
                    // WARNING: Treating indirect jump as call
  srMeshModel::~srMeshModel(*(srMeshModel **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10024159(void)

{
  int unaff_EBP;
  
  FUN_1001f4a0(*(srMeshModel **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10024161(void)

{
  int unaff_EBP;
  
  FUN_100215af(*(int *)(unaff_EBP + -0x10) + 0x3a0,8,4,FUN_1001f590);
  return;
}



void Unwind_10024179(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x14));
  return;
}



void Unwind_1002418e(void)

{
  int unaff_EBP;
  
  FUN_1001f4a0(*(srMeshModel **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10024196(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10024199. Too many branches
                    // WARNING: Treating indirect jump as call
  srMeshModel::~srMeshModel(*(srMeshModel **)(unaff_EBP + -0x10));
  return;
}



void Unwind_100241a9(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_100241c0(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + 4));
  return;
}



void Unwind_100241cb(void)

{
  int unaff_EBP;
  
  FUN_1001f5c0((undefined4 *)(*(int *)(unaff_EBP + 4) + 4));
  return;
}



void Unwind_100241d6(void)

{
  int unaff_EBP;
  
  FUN_1001f670((undefined4 *)(*(int *)(unaff_EBP + 4) + 0xc));
  return;
}



void Unwind_100241e1(void)

{
  int unaff_EBP;
  
  FUN_1001f720((undefined4 *)(*(int *)(unaff_EBP + 4) + 0x94));
  return;
}



void Unwind_100241ef(void)

{
  int unaff_EBP;
  
  FUN_1001f740((undefined4 *)(*(int *)(unaff_EBP + 4) + 0x9c));
  return;
}



void Unwind_100241fd(void)

{
  int unaff_EBP;
  
  FUN_1001f760((undefined4 *)(*(int *)(unaff_EBP + 4) + 0xa4));
  return;
}



void Unwind_1002420b(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10024220(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10024223. Too many branches
                    // WARNING: Treating indirect jump as call
  srMeshModel::~srMeshModel(*(srMeshModel **)(unaff_EBP + -0x10));
  return;
}



void Unwind_10024240(void)

{
  int unaff_EBP;
  
  Housemarque::Template_Library::Config_Base::~Config_Base
            ((Config_Base *)(*(int *)(unaff_EBP + -0xa0) + 4));
  return;
}



void Unwind_1002424e(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10024254. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -200));
  return;
}



void Unwind_1002425a(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xa8));
  return;
}



void Unwind_10024268(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002426e. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -200));
  return;
}



void Unwind_10024274(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xa8));
  return;
}



void Unwind_10024282(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10024288. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -200));
  return;
}



void Unwind_1002428e(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xa8));
  return;
}



void Unwind_1002429c(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100242a2. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -200));
  return;
}



void Unwind_100242a8(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xa8));
  return;
}



void Unwind_100242b6(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100242bc. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -200));
  return;
}



void Unwind_100242c2(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100242c8. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -0xb8));
  return;
}



void Unwind_100242ce(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100242d4. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(unaff_EBP + -200));
  return;
}



void Unwind_100242da(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x100242e0. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>(*(basic_string<> **)(unaff_EBP + -0xa8));
  return;
}



void Unwind_100242e6(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0xa4));
  return;
}



void Unwind_100242f4(void)

{
  int unaff_EBP;
  
  FUN_100142f0(*(int *)(unaff_EBP + -0xa0) + 0x44);
  return;
}



void Unwind_10024302(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0xa4) & 2) != 0) {
                    // WARNING: Could not recover jumptable at 0x10024316. Too many branches
                    // WARNING: Treating indirect jump as call
    std::basic_ios<>::~basic_ios<>((basic_ios<> *)(unaff_EBP + -0x40));
    return;
  }
  return;
}



void Unwind_1002431d(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10024323. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_istream<>::~basic_istream<>((basic_istream<> *)(unaff_EBP + -0x94));
  return;
}



void Unwind_10024329(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002432f. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_filebuf<>::~basic_filebuf<>((basic_filebuf<> *)(unaff_EBP + -0x94));
  return;
}



void Unwind_10024335(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x1002433b. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_ifstream<>::_vbase_destructor_((basic_ifstream<> *)(unaff_EBP + -0x9c));
  return;
}



void Unwind_10024341(void)

{
  int unaff_EBP;
  
  if ((*(uint *)(unaff_EBP + -0xa4) & 1) != 0) {
                    // WARNING: Could not recover jumptable at 0x10024358. Too many branches
                    // WARNING: Treating indirect jump as call
    Housemarque::Kernel::Inserter::~Inserter((Inserter *)(unaff_EBP + -0xa8));
    return;
  }
  return;
}



void Unwind_10024369(void)

{
  int unaff_EBP;
  
  Housemarque::Threedee_Engine::Memory_Texture_Config::~Memory_Texture_Config
            ((Memory_Texture_Config *)(unaff_EBP + -0x60));
  return;
}



void Unwind_1002437b(void)

{
  int unaff_EBP;
  
  FUN_100212a4(*(void **)(unaff_EBP + -0x20));
  return;
}



void Unwind_10024386(void)

{
  return;
}



void Unwind_1002438e(void)

{
  int unaff_EBP;
  
                    // WARNING: Could not recover jumptable at 0x10024394. Too many branches
                    // WARNING: Treating indirect jump as call
  std::basic_string<>::~basic_string<>((basic_string<> *)(*(int *)(unaff_EBP + -0x20) + 8));
  return;
}



void Unwind_1002439a(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + -0x20));
  return;
}



void Unwind_100243a5(void)

{
  int unaff_EBP;
  
  FUN_10014cd0(*(srClass **)(unaff_EBP + -0x20));
  return;
}



void Unwind_100243ad(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + 8));
  return;
}



void Unwind_100243b8(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + 8));
  return;
}



void Unwind_100243cd(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + 8));
  return;
}



void Unwind_100243d8(void)

{
  int unaff_EBP;
  
  FUN_10005eb0(*(void **)(unaff_EBP + 0xc));
  return;
}


