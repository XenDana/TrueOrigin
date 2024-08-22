import { FormEvent, useEffect, useState } from "react";

import Button from "../ui/Button";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faCircleNotch } from "@fortawesome/free-solid-svg-icons";
import toast from "react-hot-toast";
import { useActor } from "../../ic/Actors";

type EditProfileProps = {
  className?: string;
  allwaysShow?: boolean;
};

export default function EditProfile({
  className,
  allwaysShow,
}: EditProfileProps) {
  const { actor } = useActor();

  // Local state
  const [name, setName] = useState("");
  // const [avatarUrl, setAvatarUrl] = useState("");
  const [hasProfile, setHasProfile] = useState(true);
  const [saving, setSaving] = useState(false);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    (async () => {
      if (!actor) return;
      console.log("get_my_profile");
      const response = await actor.get_my_profile();
      console.log("profile", response)
      if (response && "Ok" in response) {
        setName(response.Ok.first_name[0]!);
        // setAvatarUrl(response.Ok.avatar_url);
        if (response.Ok.first_name[0] === "No Name") {
          setHasProfile(false);
        }
      } else {
        setHasProfile(false);
      }
      setLoading(false);
    })();
  }, [actor]);
  console.log("hasProfile", hasProfile);
  // Don't render if we already have a profile unless allwaysShow is true
  if (hasProfile && !allwaysShow) return null;

  // Don't render form when loading profile
  if (loading)
    return (
      <div className={className}>
        <div className="flex flex-col items-center w-full gap-5 h-72">
          <div className="text-2xl font-bold">User Profile</div>
          <div className="flex flex-col items-center justify-center w-full h-full">
            <FontAwesomeIcon className="w-4 h-4" icon={faCircleNotch} spin />
          </div>
        </div>
      </div>
    );

  async function submit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!actor) return;
    setSaving(true);
    toast.success("saving profile");
    const response = await actor.save_my_profile(name);
    toast.success("aaa");
    if (response && "Ok" in response) {
      toast.success("Profile saved");
      setTimeout(() => {
        window.location.reload();
      }, 1000);
    } else {
      if (response && "Err" in response) console.error(response.Err);
      toast.error("Error saving profile");
    }
    setSaving(false);
  }

  const submitIcon = saving ? faCircleNotch : undefined;

  const submitText = saving ? "Saving" : "Save";

  const submitDisabled = saving || !name;

  return (
    <div className={className}>
      <div className="flex flex-col items-center w-full gap-5">
        <div className="text-2xl font-bold">User Profile</div>
        <form
          className="flex flex-col items-center w-full gap-5"
          onSubmit={submit}
        >
          <div className="flex flex-col items-center w-full gap-3">
            <label htmlFor="name">What do you want to be called?</label>
            <input
              className="w-full px-3 py-2 leading-tight border rounded appearance-none text-zinc-500 focus:outline-none focus:bg-emerald-200 bg-slate-200"
              name="name"
              onChange={(e) => setName(e.target.value)}
              placeholder="Name"
              type="text"
              value={name}
            />
          </div>
          <Button
            className="w-full mt-5"
            disabled={submitDisabled}
            icon={submitIcon}
            spin
            type="submit"
            variant="primary"
          >
            {submitText}
          </Button>
        </form>
      </div>
    </div>
  );
}
