import { useNavigate } from "react-router-dom";
import { Button } from "@heroui/react";
import { useTranslation } from "../../../i18n";

function NotebookLeftBottom() {
  const navigate = useNavigate();
  const { translate } = useTranslation();

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
        onPress={() => navigate("/settings")}
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
        onPress={() => navigate("/about")}
        startContent={
          <span role="img" aria-label="关于" style={{ fontSize: "16px" }}>
            ⓘ
          </span>
        }
      >
        {translate("common.about")} (v.0.1.0)
      </Button>
    </div>
  );
}

export default NotebookLeftBottom;
