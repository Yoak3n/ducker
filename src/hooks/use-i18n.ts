import { useState, useCallback } from "react";
import { useTranslation } from "react-i18next";

import { changeLanguage, supportedLanguages } from "@/services/i18n";
import { useDucker } from "./use-ducker";

export const useI18n = () => {
    const { i18n, t } = useTranslation();
    const { patchDucker } = useDucker();
    const [isLoading, setIsLoading] = useState(false);
    const supportedLanguagesMap :Record<string, string> = {
            "en": "English",
            "ru": "Русский",
            "zh": "中文",
            "fa": "فارسی",
            "tt": "Татарча",
            "id": "Bahasa Indonesia",
            "ar": "العربية",
            "ko": "한국어",
            "tr": "Türkçe",
            "de": "Deutsch",
            "es": "Español",
            "jp": "日本語",
            "zhtw": "繁體中文"
        };;
    const switchLanguage = useCallback(
        async (language: string) => {
            if (!supportedLanguages.includes(language)) {
                console.warn(`Unsupported language: ${language}`);
                return;
            }

            if (i18n.language === language) {
                return;
            }

            setIsLoading(true);
            try {
                await changeLanguage(language);

                if (patchDucker) {
                    await patchDucker({ language });
                }
            } catch (error) {
                console.error("Failed to change language:", error);
            } finally {
                setIsLoading(false);
            }
        },
        [i18n.language, patchDucker],
    );

    return {
        currentLanguage: i18n.language,
        supportedLanguages,
        supportedLanguagesMap,
        switchLanguage,
        isLoading,
        t,
    };
};