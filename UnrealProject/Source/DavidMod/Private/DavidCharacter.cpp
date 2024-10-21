#include "DavidCharacter.h"

ADavidCharacter::ADavidCharacter()
{
	PrimaryActorTick.bCanEverTick = true;

	SandevistanGauge = 100.0f;
	ChromeBalance = 50.0f;
}

void ADavidCharacter::BeginPlay()
{
	Super::BeginPlay();
}

void ADavidCharacter::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);

	UpdateSandevistanGauge(DeltaTime);
	UpdateChromeBalance(DeltaTime);
}

void ADavidCharacter::SetupPlayerInputComponent(UInputComponent* PlayerInputComponent)
{
	Super::SetupPlayerInputComponent(PlayerInputComponent);
}

void ADavidCharacter::ActivateSandevistan()
{
	if (SandevistanGauge >= 30.0f)
	{
		SandevistanGauge -= 30.0f;
		// TODO: Implement Sandevistan effect
		UE_LOG(LogTemp, Warning, TEXT("Sandevistan activated!"));
	}
}

void ADavidCharacter::UseChromOverload()
{
	ChromeBalance = FMath::Min(ChromeBalance + 20.0f, 100.0f);
	// TODO: Implement Chrome Overload effect
	UE_LOG(LogTemp, Warning, TEXT("Chrome Overload used! New balance: %f"), ChromeBalance);
}

void ADavidCharacter::UpdateSandevistanGauge(float DeltaTime)
{
	SandevistanGauge = FMath::Min(SandevistanGauge + 0.5f * DeltaTime, 100.0f);
}

void ADavidCharacter::UpdateChromeBalance(float DeltaTime)
{
	ChromeBalance = FMath::Max(ChromeBalance - 0.1f * DeltaTime, 0.0f);
}
