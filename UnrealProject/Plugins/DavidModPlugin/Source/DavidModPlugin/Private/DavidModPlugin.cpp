#include "DavidModPlugin.h"
#include "Modules/ModuleManager.h"

#define LOCTEXT_NAMESPACE "FDavidModPluginModule"

void FDavidModPluginModule::StartupModule()
{
    UE_LOG(LogTemp, Warning, TEXT("DavidModPlugin: Module started"));
}

void FDavidModPluginModule::ShutdownModule()
{
    UE_LOG(LogTemp, Warning, TEXT("DavidModPlugin: Module shut down"));
}

#undef LOCTEXT_NAMESPACE

IMPLEMENT_MODULE(FDavidModPluginModule, DavidModPlugin)
