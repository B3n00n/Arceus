import { useEffect, useState, useRef } from 'react';
import { Modal, Form, Input, Select, Button, Typography, Alert } from 'antd';
import { FolderOpenOutlined } from '@ant-design/icons';
import { useGames } from '../hooks/useGames';
import type { GameVersion } from '../types';

const { Text } = Typography;

interface FileWithPath {
  file: File;
  relativePath: string;
}

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
  const [selectedFiles, setSelectedFiles] = useState<FileWithPath[]>([]);
  const folderInputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (open) {
      if (mode === 'edit' && version) {
        form.setFieldsValue({
          game_id: version.game_id,
          version: version.version,
        });
        setSelectedFiles([]);
      } else {
        form.resetFields();
        setSelectedFiles([]);
      }
    }
  }, [open, mode, version, form]);

  const handleOk = async () => {
    try {
      const values = await form.validateFields();

      if (mode === 'create' && selectedFiles.length === 0) {
        return;
      }

      onSubmit({
        ...values,
        files: selectedFiles,
      });
    } catch {
      // Validation failed
    }
  };

  const handleFolderSelect = (event: React.ChangeEvent<HTMLInputElement>) => {
    const fileList = event.target.files;
    if (!fileList || fileList.length === 0) return;

    const files: FileWithPath[] = [];

    for (let i = 0; i < fileList.length; i++) {
      const file = fileList[i];
      const fullPath = file.webkitRelativePath;
      const parts = fullPath.split('/');
      const relativePath = parts.slice(1).join('/');

      if (relativePath) {
        files.push({ file, relativePath });
      }
    }

    setSelectedFiles(files);
    event.target.value = '';
  };

  const resetFolder = () => {
    setSelectedFiles([]);
  };

  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + ' ' + sizes[i];
  };

  const totalSize = selectedFiles.reduce((sum, f) => sum + f.file.size, 0);

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
        disabled: mode === 'create' && selectedFiles.length === 0,
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
            label="Game Folder"
            required
            help={selectedFiles.length === 0 ? "Select a folder containing all game files" : undefined}
          >
            {/* Hidden folder input */}
            <input
              ref={folderInputRef}
              type="file"
              // @ts-expect-error - webkitdirectory is not in the types
              webkitdirectory=""
              directory=""
              multiple
              onChange={handleFolderSelect}
              style={{ display: 'none' }}
              disabled={loading}
            />

            {selectedFiles.length === 0 ? (
              <div
                onClick={() => folderInputRef.current?.click()}
                style={{
                  padding: '32px 20px',
                  border: '1px dashed #424242',
                  borderRadius: '8px',
                  backgroundColor: '#1a1a1a',
                  textAlign: 'center',
                  cursor: loading ? 'not-allowed' : 'pointer',
                  transition: 'border-color 0.2s',
                }}
                onMouseEnter={(e) => {
                  if (!loading) e.currentTarget.style.borderColor = '#1890ff';
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.borderColor = '#424242';
                }}
              >
                <div style={{ marginBottom: '8px' }}>
                  <FolderOpenOutlined style={{ fontSize: 48, color: '#1890ff' }} />
                </div>
                <p style={{ margin: '8px 0', fontSize: 16 }}>
                  Click to select a folder
                </p>
                <p style={{ margin: 0, color: '#666', fontSize: 12 }}>
                  All files in the folder will be uploaded directly to GCS
                </p>
              </div>
            ) : (
              <div style={{
                padding: '20px',
                border: '1px solid #424242',
                borderRadius: '8px',
                backgroundColor: '#1a1a1a',
                textAlign: 'center',
              }}>
                <div style={{ marginBottom: '8px' }}>
                  <FolderOpenOutlined style={{ fontSize: 32, color: '#52c41a' }} />
                </div>
                <div style={{ marginBottom: '8px' }}>
                  <Text strong>{selectedFiles.length} files selected</Text>
                </div>
                <div style={{ marginBottom: '12px' }}>
                  <Text type="secondary">Total size: {formatFileSize(totalSize)}</Text>
                </div>
                <Button onClick={resetFolder}>
                  Change Folder
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
