import { useTranslation } from "react-i18next";
import { LangSelector } from "./LangSelector";

export const TitleBar = () => {
  const { t } = useTranslation();
  return (
    <div className="flex items-center content-center w-full h-20 m-5 border-2">
      <div className="flex items-center w-full m-3 p-2">
        <div className="w-full ml-40">
          <h1 className="text-2xl text-center">{t("calculator.title")}</h1>
        </div>
        <h1 className="text-xs w-60">
          {t("calculator.date", { date: new Date() })}
        </h1>
      </div>
      <div className="w-24 p-1 text-center">
        <LangSelector />
      </div>
    </div>
  );
};
