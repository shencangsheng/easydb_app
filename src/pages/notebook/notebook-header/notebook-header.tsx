import {
  faChevronLeft,
  faBars,
  faSearch,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  Button,
  Input,
  Link,
  Navbar,
  NavbarBrand,
  NavbarContent,
  NavbarItem,
  NavbarMenu,
  NavbarMenuItem,
  NavbarMenuToggle,
} from "@heroui/react";
import { memo, useState } from "react";

const MENU_ITEMS = [
  "Profile",
  "Dashboard",
  "Activity",
  "Analytics",
  "System",
  "Deployments",
  "My Settings",
  "Team Settings",
  "Help & Feedback",
  "Log Out",
];

const LOGO = () => {
  return (
    <svg fill="none" height="36" viewBox="0 0 32 32" width="36">
      <path
        clipRule="evenodd"
        d="M17.6482 10.1305L15.8785 7.02583L7.02979 22.5499H10.5278L17.6482 10.1305ZM19.8798 14.0457L18.11 17.1983L19.394 19.4511H16.8453L15.1056 22.5499H24.7272L19.8798 14.0457Z"
        fill="currentColor"
        fillRule="evenodd"
      />
    </svg>
  );
};

function NotebookHeader() {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  return (
    <Navbar
      isBordered
      isMenuOpen={isMenuOpen}
      onMenuOpenChange={setIsMenuOpen}
      maxWidth="full"
      className="px-8 data-[menu-open=true]:border-solid"
    >
      <NavbarContent justify="start">
        <NavbarMenuToggle
          aria-label={isMenuOpen ? "Close menu" : "Open menu"}
          icon={() => (
            <FontAwesomeIcon
              icon={isMenuOpen ? faChevronLeft : faBars}
              style={{ height: "1.2em" }}
            />
          )}
        />
      </NavbarContent>

      <NavbarContent className="hidden sm:flex gap-8" justify="center">
        <NavbarBrand style={{ width: "100px" }}>
          <LOGO />
          <p className="font-bold text-inherit">EasyDB</p>
        </NavbarBrand>
        <NavbarItem>
          <Input
            labelPlacement="outside"
            placeholder="Search data and saved documents..."
            startContent={<FontAwesomeIcon icon={faSearch} />}
            variant="bordered"
            style={{ width: "600px", textAlign: "left" }}
          />
        </NavbarItem>
      </NavbarContent>

      <NavbarContent justify="end" style={{ paddingLeft: "200px" }}>
        <NavbarItem className="hidden lg:flex">
          <Link href="#">Login</Link>
        </NavbarItem>
        <NavbarItem>
          <Button as={Link} color="warning" href="#" variant="flat">
            Sign Up
          </Button>
        </NavbarItem>
      </NavbarContent>

      <NavbarMenu
        className="border border-gray-300 rounded-md p-2 w-1/5"
        style={{ width: "300px", border: "none" }}
      >
        {MENU_ITEMS.map((item, index) => (
          <NavbarMenuItem key={`${item}-${index}`}>
            <Link
              className="w-full block p-2 rounded hover:bg-gray-200"
              color={
                index === 2
                  ? "warning"
                  : index === MENU_ITEMS.length - 1
                  ? "danger"
                  : "foreground"
              }
              href="#"
              size="lg"
            >
              {item}
            </Link>
          </NavbarMenuItem>
        ))}
      </NavbarMenu>
    </Navbar>
  );
}

export default memo(NotebookHeader);
