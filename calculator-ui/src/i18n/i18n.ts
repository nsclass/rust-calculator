import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import Backend from "i18next-http-backend";
import { DateTime } from "luxon";

i18n
  .use(Backend)
  .use(initReactI18next) // passes i18n down to react-i18next
  .init({
    lng: "en", // if you're using a language detector, do not define the lng option
    fallbackLng: "en",
    interpolation: {
      escapeValue: false, // not needed for react as it escapes by default
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
