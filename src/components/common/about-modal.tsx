import {
  Button,
  Modal,
  ModalContent,
  ModalHeader,
  ModalBody,
  ModalFooter,
} from "@heroui/react";
import { useTranslation } from "../../i18n";
import { invoke } from "@tauri-apps/api/core";
import { getVersionInfo } from "../../utils/version";

interface AboutModalProps {
  isOpen: boolean;
  onClose: () => void;
}

function AboutModal({ isOpen, onClose }: AboutModalProps) {
  const { translate } = useTranslation();
  const versionInfo = getVersionInfo();

  const handleCheckUpdate = () => {
    // 这里可以添加检查更新的逻辑
    alert(translate("about.checkUpdate") + " - 当前已是最新版本");
  };

  const handleGithubClick = async () => {
    await invoke("open_url", {
      url: "https://github.com/shencangsheng/easydb_app",
    });
  };

  return (
    <Modal
      isOpen={isOpen}
      onClose={onClose}
      size="4xl"
      scrollBehavior="inside"
      classNames={{
        base: "max-h-[80vh]",
        body: "py-6",
      }}
    >
      <ModalContent>
        {(onClose) => (
          <>
            <ModalHeader className="flex flex-col gap-1">
              <h2 className="text-xl font-semibold">
                {translate("about.title")}
              </h2>
            </ModalHeader>
            <ModalBody>
              {/* 应用信息区域 */}
              <div className="mb-6">
                <div className="flex items-center mb-4">
                  <div className="w-16 h-16 bg-white rounded-xl flex items-center justify-center mr-4 text-3xl text-black border border-gray-200">
                    <svg fill="none" height="64" viewBox="0 0 32 32" width="64">
                      <path
                        clipRule="evenodd"
                        d="M17.6482 10.1305L15.8785 7.02583L7.02979 22.5499H10.5278L17.6482 10.1305ZM19.8798 14.0457L18.11 17.1983L19.394 19.4511H16.8453L15.1056 22.5499H24.7272L19.8798 14.0457Z"
                        fill="currentColor"
                        fillRule="evenodd"
                      />
                    </svg>
                  </div>
                  <div className="flex-1">
                    <h1 className="text-2xl font-semibold m-0 text-gray-800">
                      {translate("about.title")}
                    </h1>
                    <p className="text-base text-gray-500 mt-1 m-0">
                      {translate("about.description")}
                    </p>
                  </div>
                  <div className="ml-auto">
                    <Button
                      color="primary"
                      variant="solid"
                      onPress={handleCheckUpdate}
                      className="min-w-[120px]"
                    >
                      {translate("about.checkUpdate")}
                    </Button>
                  </div>
                </div>

                <p className="text-sm text-gray-500 leading-relaxed mb-4">
                  {translate("about.descriptionDetail")}
                </p>

                {/* 版本详细信息 */}
                <div className="bg-gray-50 border border-gray-200 rounded-lg p-4 mb-4">
                  <h4 className="text-sm font-semibold mb-3 m-0 text-gray-700">
                    版本信息
                  </h4>
                  <div className="grid grid-cols-2 gap-2">
                    <div>
                      <span className="text-xs text-gray-500">版本号：</span>
                      <span className="text-xs font-medium">
                        {versionInfo.version}
                      </span>
                    </div>
                    <div>
                      <span className="text-xs text-gray-500">构建号：</span>
                      <span className="text-xs font-medium">
                        {versionInfo.buildNumber}
                      </span>
                    </div>
                    <div>
                      <span className="text-xs text-gray-500">发布日期：</span>
                      <span className="text-xs font-medium">
                        {versionInfo.releaseDate}
                      </span>
                    </div>
                    <div>
                      <span className="text-xs text-gray-500">许可证：</span>
                      <span className="text-xs font-medium">
                        {versionInfo.license}
                      </span>
                    </div>
                  </div>
                </div>
              </div>

              {/* 社交媒体和联系方式 */}
              <div className="mb-6">
                <h3 className="text-lg font-semibold mb-4 text-gray-800">
                  {translate("about.socialMedia")}
                </h3>

                <div className="flex flex-col gap-3">
                  {/* Github */}
                  <div
                    onClick={handleGithubClick}
                    className="flex items-center p-3 bg-gray-50 rounded-lg cursor-pointer transition-colors hover:bg-gray-100"
                  >
                    <span className="text-xl mr-3">
                      <svg
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="currentColor"
                        aria-label="GitHub"
                        className="inline-block align-middle"
                      >
                        <path d="M12 2C6.477 2 2 6.484 2 12.021c0 4.428 2.865 8.184 6.839 9.504.5.092.682-.217.682-.483 0-.237-.009-.868-.014-1.703-2.782.605-3.369-1.342-3.369-1.342-.454-1.154-1.11-1.462-1.11-1.462-.908-.62.069-.608.069-.608 1.004.07 1.532 1.032 1.532 1.032.892 1.53 2.341 1.088 2.91.832.091-.647.35-1.088.636-1.339-2.221-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.025A9.564 9.564 0 0 1 12 6.844c.85.004 1.705.115 2.504.337 1.909-1.295 2.748-1.025 2.748-1.025.546 1.378.202 2.397.1 2.65.64.7 1.028 1.595 1.028 2.688 0 3.847-2.337 4.695-4.566 4.944.359.309.678.919.678 1.853 0 1.337-.012 2.419-.012 2.749 0 .268.18.579.688.481C19.138 20.2 22 16.447 22 12.021 22 6.484 17.523 2 12 2z" />
                      </svg>
                    </span>
                    <span className="text-sm font-medium">
                      {translate("about.github")}
                    </span>
                    <span className="ml-auto text-sm text-gray-500">
                      EasyDB
                    </span>
                    <span className="ml-2 text-sm">→</span>
                  </div>

                  {/* 邮箱 */}
                  <div className="flex items-center p-3 bg-gray-50 rounded-lg">
                    <span className="text-xl mr-3">✉️</span>
                    <span className="text-sm font-medium">
                      {translate("about.email")}
                    </span>
                    <span className="ml-auto text-sm text-gray-500">
                      shencangsheng@126.com
                    </span>
                    <span className="ml-2 text-sm">→</span>
                  </div>
                </div>
              </div>

              {/* 其他链接 */}
              <div>
                <h3 className="text-lg font-semibold mb-4 text-gray-800">
                  {translate("common.general")}
                </h3>

                <div className="flex flex-col gap-3">
                  {/* 首页 */}
                  <div
                    onClick={onClose}
                    className="flex items-center p-3 bg-gray-50 rounded-lg cursor-pointer transition-colors hover:bg-gray-100"
                  >
                    <span className="text-xl mr-3">🏠</span>
                    <span className="text-sm font-medium">
                      {translate("about.homepage")}
                    </span>
                    <span className="ml-auto text-sm">→</span>
                  </div>
                </div>
              </div>
            </ModalBody>
            <ModalFooter>
              <Button color="primary" onPress={onClose}>
                关闭
              </Button>
            </ModalFooter>
          </>
        )}
      </ModalContent>
    </Modal>
  );
}

export default AboutModal;
