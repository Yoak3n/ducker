/// <reference types="vite/client" />
declare interface Window{
    $loadingBar:LoadingBarInst
    $dialog:DialogApiInjection
    $message:MessageApiInjection
    $notification:NotificationApiInjection
    PIXI:typeof PIXI
} 