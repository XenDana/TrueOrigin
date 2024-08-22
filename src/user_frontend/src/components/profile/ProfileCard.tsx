import AddressPill from "../AddressPill";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import PrincipalPill from "../PrincipalPill";
import { User } from "../../../../declarations/TrustOrigin_backend/TrustOrigin_backend.did";
import { faUser } from "@fortawesome/free-solid-svg-icons";
// import { useState } from "react";

type ProfileCardProps = {
  principal: string;
  profile: User;
};

export default function ProfileCard({ principal, profile }: ProfileCardProps) {
  // const [imageError, setImageError] = useState(false);

  // const handleImageError = () => {
  //   setImageError(true);
  // };

  return (
    <div className="flex flex-col items-center gap-5 p-5 border w-44 rounded-xl border-zinc-700 bg-zinc-800">
        <div className="w-20 h-20 border-[1px] rounded-full border-zinc-400/50 flex justify-center items-center">
          <FontAwesomeIcon
            className="w-10 h-10 text-zinc-400/50"
            icon={faUser}
          />
        </div>
      <div className="w-32 font-bold text-center overflow-clip whitespace-nowrap overflow-ellipsis">
        {profile.first_name}
      </div>
      <PrincipalPill className="w-full bg-zinc-700" principal={principal} />
      <AddressPill address={profile.address} className="w-full bg-zinc-700" />
    </div>
  );
}
