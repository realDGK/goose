import React from 'react';
import { ExclamationButton, GreenCheckButton } from './buttons/CardButtons';
import {
  ConfiguredProviderTooltipMessage,
  OllamaNotConfiguredTooltipMessage,
  ProviderDescription,
} from './utils/StringUtils';

interface CardHeaderProps {
  name: string;
  description: string;
  isConfigured: boolean;
}

// Make CardTitle a proper React component
function CardTitle({ name }: { name: string }) {
  return <h3 className="text-base font-medium text-textStandard truncate mr-2">{name}</h3>;
}

// Properly type ProviderNameAndStatus props
interface ProviderNameAndStatusProps {
  name: string;
  isConfigured: boolean;
}

function ProviderNameAndStatus({ name, isConfigured }: ProviderNameAndStatusProps) {
  console.log(`Provider Name: ${name}, Is Configured: ${isConfigured}`);

  return (
    <div className="flex items-center justify-between w-full">
      <CardTitle name={name} />

      {/* Configured state: Green check */}
      {isConfigured && <GreenCheckButton tooltip={ConfiguredProviderTooltipMessage(name)} />}
    </div>
  );
}

// Add a container div to the CardHeader
export default function CardHeader({ name, description, isConfigured }: CardHeaderProps) {
  return (
    <>
      <ProviderNameAndStatus name={name} isConfigured={isConfigured} />
      <ProviderDescription description={description} />
    </>
  );
}
