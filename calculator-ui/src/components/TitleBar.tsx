import { useTranslation } from "react-i18next";
import { LangSelector } from "./LangSelector";

export const TitleBar = () => {
  const { t } = useTranslation();
  return (
    <div className="flex items-end items-center content-center w-full border-solid border-2 h-20">
      <div className="flex items-center border-2 w-4/5 m-3 p-2">
        <h1 className="text-2xl w-full ml-10 text-center">
          {t("calculator.title")}
        </h1>
        <h1 className="text-xs w-60">
          {t("calculator.date", { date: new Date() })}
        </h1>
      </div>
      <div className="w-1/5 p-1 text-center border-2">
        <LangSelector />
      </div>
    </div>
  );
};
