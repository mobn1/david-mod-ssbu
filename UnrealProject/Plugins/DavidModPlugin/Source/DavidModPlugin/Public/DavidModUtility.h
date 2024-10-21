#pragma once

#include "CoreMinimal.h"
#include "UObject/NoExportTypes.h"
#include "DavidModUtility.generated.h"

UCLASS()
class DAVIDMODPLUGIN_API UDavidModUtility : public UObject
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintCallable, Category = "David Mod")
    static void ModifyMove(class ADavidCharacter* Character, FName MoveName);

    UFUNCTION(BlueprintCallable, Category = "David Mod")
    static void UpdateCharacterState(class ADavidCharacter* Character);
};
