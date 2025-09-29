import { useState } from "react";
import { Button } from "@heroui/react";
import { useTranslation } from "../../../i18n";
import SettingsModal from "../../../components/common/settings-modal";
import AboutModal from "../../../components/common/about-modal";
import { getVersion } from "../../../utils/version";

function NotebookLeftBottom() {
  const { translate } = useTranslation();
  const [isSettingsModalOpen, setIsSettingsModalOpen] = useState(false);
  const [isAboutModalOpen, setIsAboutModalOpen] = useState(false);
  const version = getVersion();

  return (
    <div
      style={{
        display: "flex",
        flexDirection: "column",
        alignItems: "stretch",
        padding: "13px",
      }}
    >
      <Button
        variant="light"
        style={{
          borderRadius: "6px",
          padding: "8px 13px",
          marginBottom: "5px",
          fontWeight: 500,
          width: "100%",
          justifyContent: "flex-start",
          textAlign: "left",
          display: "flex",
          alignItems: "center",
          gap: "10px",
        }}
        onPress={() => setIsSettingsModalOpen(true)}
        startContent={
          <span role="img" aria-label="设置" style={{ fontSize: "16px" }}>
            ⚙️
          </span>
        }
      >
        {translate("common.settings")}
      </Button>
      <Button
        variant="light"
        style={{
          borderRadius: "6px",
          padding: "8px 13px",
          marginBottom: "5px",
          fontWeight: 500,
          width: "100%",
          justifyContent: "flex-start",
          textAlign: "left",
          display: "flex",
          alignItems: "center",
          gap: "10px",
        }}
        onPress={() => setIsAboutModalOpen(true)}
        startContent={
          <span role="img" aria-label="关于" style={{ fontSize: "16px" }}>
            ⓘ
          </span>
        }
      >
        {translate("common.about")} (v.{version})
      </Button>

      {/* 设置弹窗 */}
      <SettingsModal
        isOpen={isSettingsModalOpen}
        onClose={() => setIsSettingsModalOpen(false)}
      />

      {/* 关于弹窗 */}
      <AboutModal
        isOpen={isAboutModalOpen}
        onClose={() => setIsAboutModalOpen(false)}
      />
    </div>
  );
}

export default NotebookLeftBottom;
