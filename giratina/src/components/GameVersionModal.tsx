import { useEffect, useState } from 'react';
import { Modal, Form, Input, Select, Upload, Button, Typography, Alert } from 'antd';
import { InboxOutlined } from '@ant-design/icons';
import { useGames } from '../hooks/useGames';
import type { GameVersion } from '../types';

const { Dragger } = Upload;
const { Text } = Typography;

interface GameVersionModalProps {
  open: boolean;
  mode: 'create' | 'edit';
  version?: GameVersion;
  onSubmit: (values: any) => void;
  onCancel: () => void;
  loading?: boolean;
}

export const GameVersionModal = ({
  open,
  mode,
  version,
  onSubmit,
  onCancel,
  loading,
}: GameVersionModalProps) => {
  const [form] = Form.useForm();
  const { data: games = [] } = useGames();
  const [zipFile, setZipFile] = useState<File | null>(null);

  useEffect(() => {
    if (open) {
      if (mode === 'edit' && version) {
        form.setFieldsValue({
          game_id: version.game_id,
          version: version.version,
        });
        setZipFile(null);
      } else {
        form.resetFields();
        setZipFile(null);
      }
    }
  }, [open, mode, version, form]);

  const handleOk = async () => {
    try {
      const values = await form.validateFields();

      // For create mode, file is required
      if (mode === 'create' && !zipFile) {
        return;
      }

      onSubmit({
        ...values,
        file: zipFile,
      });
    } catch {
      // Validation failed
    }
  };

  const beforeUpload = (file: File) => {
    // Only allow .zip files
    const isZip = file.type === 'application/zip' ||
                  file.type === 'application/x-zip-compressed' ||
                  file.name.toLowerCase().endsWith('.zip');

    if (!isZip) {
      return Upload.LIST_IGNORE;
    }

    // Max 20GB
    const maxSize = 20 * 1024 * 1024 * 1024;
    if (file.size > maxSize) {
      return Upload.LIST_IGNORE;
    }

    setZipFile(file);

    // Prevent auto-upload
    return false;
  };

  const resetUpload = () => {
    setZipFile(null);
  };

  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
  };

  return (
    <Modal
      title={mode === 'create' ? 'Upload Game Version' : 'Edit Game Version'}
      open={open}
      onOk={handleOk}
      onCancel={onCancel}
      confirmLoading={loading}
      destroyOnHidden
      width={600}
      okText={mode === 'create' ? 'Upload' : 'Update'}
      okButtonProps={{
        disabled: mode === 'create' && !zipFile,
      }}
    >
      <Form
        form={form}
        layout="vertical"
      >
        <Form.Item
          name="game_id"
          label="Game"
          rules={[{ required: true, message: 'Please select a game' }]}
        >
          <Select
            placeholder="Select a game"
            disabled={mode === 'edit'}
            options={games.map((game) => ({
              label: game.name,
              value: game.id,
            }))}
            showSearch
            filterOption={(input, option) =>
              (option?.label ?? '').toLowerCase().includes(input.toLowerCase())
            }
          />
        </Form.Item>

        <Form.Item
          name="version"
          label="Version"
          rules={[
            { required: true, message: 'Please enter version' },
            {
              pattern: /^\d+\.\d+\.\d+$/,
              message: 'Version must be in format X.Y.Z (e.g., 1.0.0)',
            },
          ]}
        >
          <Input placeholder="e.g., 1.0.0" disabled={mode === 'edit'} />
        </Form.Item>

        {mode === 'create' && (
          <Form.Item
            label="Game ZIP File"
            required
            help={!zipFile ? "Upload a ZIP file containing all game files (max 20GB)" : undefined}
          >
            {!zipFile ? (
              <Dragger
                beforeUpload={beforeUpload}
                maxCount={1}
                accept=".zip"
                disabled={loading}
                showUploadList={false}
              >
                <p className="ant-upload-drag-icon">
                  <InboxOutlined />
                </p>
                <p className="ant-upload-text">Click or drag ZIP file to this area</p>
                <p className="ant-upload-hint">
                  The ZIP will be automatically extracted on GCS. Max file size: 20GB
                </p>
              </Dragger>
            ) : (
              <div style={{
                padding: '20px',
                border: '1px solid #424242',
                borderRadius: '8px',
                backgroundColor: '#1a1a1a',
                textAlign: 'center',
              }}>
                <div style={{ marginBottom: '8px' }}>
                  <Text strong>{zipFile.name}</Text>
                </div>
                <div style={{ marginBottom: '12px' }}>
                  <Text type="secondary">Size: {formatFileSize(zipFile.size)}</Text>
                </div>
                <Button onClick={resetUpload}>
                  Change File
                </Button>
              </div>
            )}
          </Form.Item>
        )}

        {mode === 'edit' && (
          <Alert
            title="Note"
            description="To upload a new version, create a new game version instead of editing."
            type="info"
            showIcon
          />
        )}
      </Form>
    </Modal>
  );
};
