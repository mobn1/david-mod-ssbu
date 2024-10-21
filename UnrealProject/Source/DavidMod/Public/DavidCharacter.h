#pragma once

#include "CoreMinimal.h"
#include "GameFramework/Character.h"
#include "DavidCharacter.generated.h"

UCLASS()
class DAVIDMOD_API ADavidCharacter : public ACharacter
{
	GENERATED_BODY()

public:
	ADavidCharacter();

protected:
	virtual void BeginPlay() override;

public:	
	virtual void Tick(float DeltaTime) override;

	virtual void SetupPlayerInputComponent(class UInputComponent* PlayerInputComponent) override;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "David")
	float SandevistanGauge;

	UPROPERTY(EditAnywhere, BlueprintReadWrite, Category = "David")
	float ChromeBalance;

	UFUNCTION(BlueprintCallable, Category = "David")
	void ActivateSandevistan();

	UFUNCTION(BlueprintCallable, Category = "David")
	void UseChromOverload();

private:
	void UpdateSandevistanGauge(float DeltaTime);
	void UpdateChromeBalance(float DeltaTime);
};
