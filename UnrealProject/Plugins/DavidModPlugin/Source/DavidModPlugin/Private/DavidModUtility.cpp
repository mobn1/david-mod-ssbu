#include "DavidModUtility.h"
#include "DavidCharacter.h"

void UDavidModUtility::ModifyMove(ADavidCharacter* Character, FName MoveName)
{
    if (!Character)
        return;

    // TODO: Implement move modification logic
    UE_LOG(LogTemp, Warning, TEXT("Modifying move: %s"), *MoveName.ToString());
}

void UDavidModUtility::UpdateCharacterState(ADavidCharacter* Character)
{
    if (!Character)
        return;

    // TODO: Implement character state update logic
    Character->UpdateSandevistanGauge(0.016f); // Assuming 60 FPS
    Character->UpdateChromeBalance(0.016f);
}
