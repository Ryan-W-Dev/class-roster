const LANGUAGE_STORAGE_KEY = "class-roster-language";
const SUPPORTED_LANGUAGES = new Set(["en", "ar"]);

const translations = {
  en: {
    eyebrow: "Personnel Management",
    title: "Class Roster",
    subtitle: "Add personnel to a locally stored attendance workbook.",
    personnelDetails: "Personnel details",
    courseDetails: "Course details",
    fullNameLabel: "Full name",
    fullNamePlaceholder: "Enter full name",
    militaryIdLabel: "Military ID",
    militaryIdPlaceholder: "Enter military ID",
    rankLabel: "Rank",
    selectRank: "Select rank",
    unitLabel: "Unit",
    selectUnit: "Select unit",
    dateLabel: "Date",
    courseLabel: "Course",
    selectCourse: "Select course",
    save: "Save to roster",
    saving: "Saving…",
    switchLanguage: "العربية",
    switchLanguageAria: "Switch to Arabic",
    storageNote: "Records are saved locally to an Excel workbook in your Documents folder.",
    requiredMessage: "Please complete all required fields.",
    savedMessage: "Saved as roster entry {rosterNumber} (Excel row {excelRow}).",
    errorMessage: "Could not save the record: {message}",
    unknownError: "An unexpected error occurred.",

    major: "Major",
    captain: "Captain",
    firstLieutenant: "First Lieutenant",
    secondLieutenant: "Second Lieutenant",
    warrantOfficerFirstClass: "Warrant Officer First Class",
    warrantOfficer: "Warrant Officer",
    staffSergeant: "Staff Sergeant",
    sergeant: "Sergeant",
    corporalFirstClass: "Corporal First Class",
    corporal: "Corporal",
    lanceCorporal: "Lance Corporal",
    privateFirstClass: "Private First Class",
    private: "Private",

    landForces: "Land Forces",
    nationalGuard: "National Guard",
    presidentialGuard: "Presidential Guard",
    navy: "Navy",
    aviation: "Joint Aviation Command",

    masterGunner: "Master Gunners Course",
    masterArmor: "Master Armors Course",
    instructor: "Instructor",
  },

  ar: {
    eyebrow: "إدارة الأفراد",
    title: "كشف أفراد الدورة",
    subtitle: "إضافة الأفراد إلى سجل حضور محفوظ محلياً.",
    personnelDetails: "بيانات الفرد",
    courseDetails: "بيانات الدورة",
    fullNameLabel: "الاسم الكامل",
    fullNamePlaceholder: "أدخل الاسم الكامل",
    militaryIdLabel: "الرقم العسكري",
    militaryIdPlaceholder: "أدخل الرقم العسكري",
    rankLabel: "الرتبة",
    selectRank: "اختر الرتبة",
    unitLabel: "الوحدة",
    selectUnit: "اختر الوحدة",
    dateLabel: "التاريخ",
    courseLabel: "الدورة",
    selectCourse: "اختر الدورة",
    save: "حفظ في السجل",
    saving: "جارٍ الحفظ…",
    switchLanguage: "English",
    switchLanguageAria: "التبديل إلى الإنجليزية",
    storageNote: "تُحفظ السجلات محلياً في ملف إكسل داخل مجلد المستندات.",
    requiredMessage: "يرجى إكمال جميع الحقول المطلوبة.",
    savedMessage: "تم الحفظ كقيد رقم {rosterNumber} (صف إكسل {excelRow}).",
    errorMessage: "تعذر حفظ السجل: {message}",
    unknownError: "حدث خطأ غير متوقع.",

    major: "رائد",
    captain: "نقيب",
    firstLieutenant: "ملازم أول",
    secondLieutenant: "ملازم ثاني",
    warrantOfficerFirstClass: "رئيس ضباط صف",
    warrantOfficer: "ضابط صف",
    staffSergeant: "رقيب أول",
    sergeant: "رقيب",
    corporalFirstClass: "عريف أول",
    corporal: "عريف",
    lanceCorporal: "عريف",
    privateFirstClass: "جندي أول",
    private: "جندي",

    landForces: "القوات البرية",
    nationalGuard: "الحرس الوطني",
    presidentialGuard: "الحرس الرئاسي",
    navy: "البحرية",
    aviation: "قيادة الطيران المشتركة",

    masterGunner: "دورة الرامي المتقدمة",
    masterArmor: "دورة خبير السلاح المتقدمة",
    instructor: "دورة تدريب المدربين",
  },
};

let currentLanguage = "en";

function readSavedLanguage() {
  try {
    const savedLanguage = window.localStorage.getItem(LANGUAGE_STORAGE_KEY);
    return SUPPORTED_LANGUAGES.has(savedLanguage) ? savedLanguage : "en";
  } catch {
    return "en";
  }
}

function saveLanguage(language) {
  try {
    window.localStorage.setItem(LANGUAGE_STORAGE_KEY, language);
  } catch {
    // The language still works for the current session when storage is unavailable.
  }
}

export function translate(key, parameters = {}) {
  const template = translations[currentLanguage][key] ?? translations.en[key] ?? key;

  return Object.entries(parameters).reduce(
    (message, [name, value]) => message.replaceAll(`{${name}}`, String(value)),
    template,
  );
}

function applyLanguage(language, { persist = true } = {}) {
  currentLanguage = SUPPORTED_LANGUAGES.has(language) ? language : "en";

  document.documentElement.lang = currentLanguage;
  document.documentElement.dir = currentLanguage === "ar" ? "rtl" : "ltr";
  document.title = translate("title");

  document.querySelectorAll("[data-translate]").forEach((element) => {
    element.textContent = translate(element.dataset.translate);
  });

  document.querySelectorAll("[data-placeholder]").forEach((element) => {
    element.placeholder = translate(element.dataset.placeholder);
  });

  const languageButton = document.querySelector("#languageButton");
  if (languageButton) {
    languageButton.textContent = translate("switchLanguage");
    languageButton.setAttribute("aria-label", translate("switchLanguageAria"));
  }

  if (persist) {
    saveLanguage(currentLanguage);
  }

  window.dispatchEvent(
    new CustomEvent("app:languagechange", {
      detail: { language: currentLanguage },
    }),
  );
}

export function initializeLanguage() {
  applyLanguage(readSavedLanguage(), { persist: false });

  const languageButton = document.querySelector("#languageButton");
  languageButton?.addEventListener("click", () => {
    applyLanguage(currentLanguage === "en" ? "ar" : "en");
  });
}
