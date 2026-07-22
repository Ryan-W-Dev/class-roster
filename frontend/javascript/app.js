import { invoke } from "@tauri-apps/api/core";
import { initializeLanguage, translate } from "./language.js";

function getLocalDate() {
  const now = new Date();
  const localTime = now.getTime() - now.getTimezoneOffset() * 60_000;
  return new Date(localTime).toISOString().slice(0, 10);
}

function getErrorMessage(error) {
  if (typeof error === "string" && error.trim()) {
    return error;
  }

  if (error instanceof Error && error.message) {
    return error.message;
  }

  return translate("unknownError");
}

function initializeApp() {
  initializeLanguage();

  const form = document.querySelector("#personnelForm");
  const dateField = document.querySelector("#currentDate");
  const status = document.querySelector("#formStatus");
  const submitButton = form?.querySelector('button[type="submit"]');
  const submitLabel = submitButton?.querySelector("[data-submit-label]");

  if (!form || !dateField || !status || !submitButton || !submitLabel) {
    return;
  }

  let currentStatus = null;

  const setCurrentDate = () => {
    dateField.value = getLocalDate();
  };

  const renderStatus = () => {
    if (!currentStatus) {
      status.hidden = true;
      status.textContent = "";
      delete status.dataset.state;
      return;
    }

    status.textContent = translate(currentStatus.key, currentStatus.parameters);
    status.dataset.state = currentStatus.state;
    status.hidden = false;
  };

  const setStatus = (key, state, parameters = {}) => {
    currentStatus = { key, state, parameters };
    renderStatus();
  };

  const setSaving = (isSaving) => {
    submitButton.disabled = isSaving;
    submitButton.setAttribute("aria-busy", String(isSaving));
    submitLabel.textContent = translate(isSaving ? "saving" : "save");
  };

  const findFirstEmptyField = () =>
    [...form.querySelectorAll("[required]")].find((field) => !field.value.trim());

  const readFormData = () => ({
    fullName: form.elements.fullName.value.trim(),
    rank: form.elements.rank.value.trim(),
    militaryId: form.elements.militaryId.value.trim(),
    unit: form.elements.unit.value.trim(),
    currentDate: form.elements.currentDate.value.trim(),
    course: form.elements.course.value.trim(),
  });

  const resetForm = () => {
    form.reset();
    setCurrentDate();
    form.elements.fullName.focus();
  };

  setCurrentDate();

  window.addEventListener("app:languagechange", () => {
    renderStatus();
    submitLabel.textContent = translate(submitButton.disabled ? "saving" : "save");
  });

  form.addEventListener("input", () => {
    if (currentStatus?.state === "error") {
      currentStatus = null;
      renderStatus();
    }
  });

  form.addEventListener("submit", async (event) => {
    event.preventDefault();

    const emptyField = findFirstEmptyField();
    if (emptyField) {
      setStatus("requiredMessage", "error");
      emptyField.focus();
      return;
    }

    setSaving(true);
    currentStatus = null;
    renderStatus();

    try {
      const result = await invoke("save_personnel", { data: readFormData() });

      resetForm();
      setStatus("savedMessage", "success", {
        rosterNumber: result.rosterNumber,
        excelRow: result.excelRow,
      });
    } catch (error) {
      setStatus("errorMessage", "error", {
        message: getErrorMessage(error),
      });
    } finally {
      setSaving(false);
    }
  });
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", initializeApp, { once: true });
} else {
  initializeApp();
}
