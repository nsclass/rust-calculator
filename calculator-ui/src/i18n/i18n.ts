import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import { DateTime } from "luxon";
import enJson from "./locales/en/translation.json";
import koJson from "./locales/ko/translation.json";

i18n
  .use(initReactI18next) // passes i18n down to react-i18next
  .init({
    lng: "en", // if you're using a language detector, do not define the lng option
    fallbackLng: "en",
    interpolation: {
      escapeValue: false, // not needed for react as it escapes by default
    },
    resources: {
      en: {
        translation: {
          ...enJson,
        },
      },
      ko: {
        translation: {
          ...koJson,
        },
      },
    },
  });

// new usage
i18n.services.formatter?.add("DATE_HUGE", (value, lng, options) => {
  console.log(`Date time: ${value}, ${lng}, ${options}`);
  const lang = lng ? lng : "en";
  return DateTime.fromJSDate(value)
    .setLocale(lang)
    .toLocaleString(DateTime.DATE_HUGE);
});
