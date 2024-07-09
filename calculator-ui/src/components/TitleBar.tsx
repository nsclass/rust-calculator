import { useTranslation } from "react-i18next";
import { LangSelector } from "./LangSelector";

export const TitleBar = () => {
  const { t } = useTranslation();
  return (
    <div className="flex items-end items-center content-center w-full h-20">
      <div className="flex items-center w-4/5 m-3 p-2">
        <h1 className="text-2xl w-full ml-10 text-center">
          {t("calculator.title")}
        </h1>
        <h1 className="text-xs w-1/5">
          {t("calculator.date", { date: new Date() })}
        </h1>
      </div>
      <div className="w-1/5 p-1 text-center">
        <LangSelector />
      </div>
    </div>
  );
};
