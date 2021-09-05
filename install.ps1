function Write-Log {
    param (
        [Parameter(Mandatory=$true)]
        [string]
        $Message
    )
    
    Write-Host $Message -ForegroundColor Gray
}

function Write-Warning {
    param (
        [Parameter(Mandatory=$true)]
        [string]
        $Message
    )
    
    Write-Host $Message -ForegroundColor Yellow
}

function Write-Success {
    param (
        [Parameter(Mandatory=$true)]
        [string]
        $Message
    )

    Write-Host $Message -ForegroundColor Green
}

function Write-Error {
    param (
        [Parameter(Mandatory=$true)]
        [string]
        $Message
    )

    Write-Host $Message -ForegroundColor Red
}

function Install-Windows-Service {
    param (
        [Parameter(Mandatory=$true)]
        [string]
        $ServiceName
    )
    
    Write-Log -Message "We will now install windows service: $service_name"
    
}

function Stop-Windows-Service {
    param (
        [Parameter(Mandatory=$true)]
        [string]
        $ServiceName
    )

    Write-Log -Message "Let's try stop the service first..."
    Stop-Service -Name $ServiceName

    $is_service_stopped = $false;
    for ($i = 0; $i -lt 3; $i++) {
        Write-Log -Message "Wait until the service ($ServiceName) is stopped..."
        Start-Sleep -Seconds 1
        $service_controller = Get-Service -Name $service_name
        if ($null -eq $service_controller) {
            Write-Log -Message "The service ($ServiceName) is accidentially removed. Skip uninstalling..."
            return $true;
        }

        if ($service_controller.Status -eq 'Stopped') {
            Write-Log -Message "The service ($ServiceName) was stopped."
            $is_service_stopped = $true;
            break;
        }
    }

    if (-not $is_service_stopped) {
        Write-Log -Message "The service ($ServiceName) cannot be stopped in certain time period."
        return $false;
    }

    return $true;
}

$service_name = 'sample_service'

Write-Log -Message "Querying windows service installation status: $service_name..."
$query_service_result = Get-WmiObject win32_service | Where-Object{ $_.Name -eq $service_name };
Write-Success -Message "Querying windows service installation status: $service_name...Done"

if ($null -eq $query_service_result) {
    Write-Log -Message "The Windows service '$service_name' does not exist. No worries. Will install service."
} else {
    Write-Log -Message "The Windows service '$service_name' currently exist. We need to uninstall the service."
    $stop_result = Stop-Windows-Service -ServiceName $service_name
    if (-not $stop_result) {
        Write-Error -Message "The Windows service '$service_name' cannot be stopped. We will exit."
        return;
    }
}