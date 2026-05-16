import CoreImage
import Foundation

private func ci_qr_level(_ code: Int32) -> CIQRCodeDescriptor.ErrorCorrectionLevel? {
    switch code {
    case 0:
        return .levelL
    case 1:
        return .levelM
    case 2:
        return .levelQ
    case 3:
        return .levelH
    default:
        return nil
    }
}

private func ci_data_matrix_ecc(_ code: Int32) -> CIDataMatrixCodeDescriptor.ECCVersion? {
    switch code {
    case 0:
        return .v000
    case 50:
        return .v050
    case 80:
        return .v080
    case 100:
        return .v100
    case 140:
        return .v140
    case 200:
        return .v200
    default:
        return nil
    }
}

private func ci_descriptor_payload(_ descriptor: CIBarcodeDescriptor) -> Data {
    switch descriptor {
    case let qr as CIQRCodeDescriptor:
        return qr.errorCorrectedPayload
    case let aztec as CIAztecCodeDescriptor:
        return aztec.errorCorrectedPayload
    case let pdf as CIPDF417CodeDescriptor:
        return pdf.errorCorrectedPayload
    case let dataMatrix as CIDataMatrixCodeDescriptor:
        return dataMatrix.errorCorrectedPayload
    default:
        return Data()
    }
}

@_cdecl("ci_barcode_descriptor_new_qr")
public func ci_barcode_descriptor_new_qr(
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ symbolVersion: Int,
    _ maskPattern: UInt8,
    _ errorCorrectionLevel: Int32,
    _ outDescriptor: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let bytes, len >= 0, let level = ci_qr_level(errorCorrectionLevel), let outDescriptor else {
            throw CIBridgeError.invalidArgument("invalid QR descriptor arguments")
        }
        let data = Data(bytes: bytes, count: len)
        guard let descriptor = CIQRCodeDescriptor(
            payload: data,
            symbolVersion: symbolVersion,
            maskPattern: maskPattern,
            errorCorrectionLevel: level
        ) else {
            throw CIBridgeError.nullResult("CIQRCodeDescriptor initializer returned nil")
        }
        outDescriptor.pointee = ci_retain(descriptor)
    }
}

@_cdecl("ci_barcode_descriptor_new_aztec")
public func ci_barcode_descriptor_new_aztec(
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ isCompact: Bool,
    _ layerCount: Int,
    _ dataCodewordCount: Int,
    _ outDescriptor: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let bytes, len >= 0, let outDescriptor else {
            throw CIBridgeError.invalidArgument("invalid Aztec descriptor arguments")
        }
        let data = Data(bytes: bytes, count: len)
        guard let descriptor = CIAztecCodeDescriptor(
            payload: data,
            isCompact: isCompact,
            layerCount: layerCount,
            dataCodewordCount: dataCodewordCount
        ) else {
            throw CIBridgeError.nullResult("CIAztecCodeDescriptor initializer returned nil")
        }
        outDescriptor.pointee = ci_retain(descriptor)
    }
}

@_cdecl("ci_barcode_descriptor_new_pdf417")
public func ci_barcode_descriptor_new_pdf417(
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ isCompact: Bool,
    _ rowCount: Int,
    _ columnCount: Int,
    _ outDescriptor: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let bytes, len >= 0, let outDescriptor else {
            throw CIBridgeError.invalidArgument("invalid PDF417 descriptor arguments")
        }
        let data = Data(bytes: bytes, count: len)
        guard let descriptor = CIPDF417CodeDescriptor(
            payload: data,
            isCompact: isCompact,
            rowCount: rowCount,
            columnCount: columnCount
        ) else {
            throw CIBridgeError.nullResult("CIPDF417CodeDescriptor initializer returned nil")
        }
        outDescriptor.pointee = ci_retain(descriptor)
    }
}

@_cdecl("ci_barcode_descriptor_new_data_matrix")
public func ci_barcode_descriptor_new_data_matrix(
    _ bytes: UnsafePointer<UInt8>?,
    _ len: Int,
    _ rowCount: Int,
    _ columnCount: Int,
    _ eccVersion: Int32,
    _ outDescriptor: UnsafeMutablePointer<UnsafeMutableRawPointer?>?,
    _ outError: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    ci_run(outError) {
        guard let bytes, len >= 0, let ecc = ci_data_matrix_ecc(eccVersion), let outDescriptor else {
            throw CIBridgeError.invalidArgument("invalid Data Matrix descriptor arguments")
        }
        let data = Data(bytes: bytes, count: len)
        guard let descriptor = CIDataMatrixCodeDescriptor(
            payload: data,
            rowCount: rowCount,
            columnCount: columnCount,
            eccVersion: ecc
        ) else {
            throw CIBridgeError.nullResult("CIDataMatrixCodeDescriptor initializer returned nil")
        }
        outDescriptor.pointee = ci_retain(descriptor)
    }
}

@_cdecl("ci_barcode_descriptor_kind")
public func ci_barcode_descriptor_kind(_ handle: UnsafeMutableRawPointer?) -> Int32 {
    guard let descriptor: CIBarcodeDescriptor = ci_borrow(handle) else { return -1 }
    switch descriptor {
    case is CIQRCodeDescriptor:
        return 0
    case is CIAztecCodeDescriptor:
        return 1
    case is CIPDF417CodeDescriptor:
        return 2
    case is CIDataMatrixCodeDescriptor:
        return 3
    default:
        return -1
    }
}

@_cdecl("ci_barcode_descriptor_payload_base64")
public func ci_barcode_descriptor_payload_base64(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptor: CIBarcodeDescriptor = ci_borrow(handle) else { return nil }
    return ci_string(ci_descriptor_payload(descriptor).base64EncodedString())
}

@_cdecl("ci_barcode_descriptor_details_json")
public func ci_barcode_descriptor_details_json(_ handle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptor: CIBarcodeDescriptor = ci_borrow(handle) else { return ci_string("{}") }
    var details: [String: Any] = [
        "kind": ci_barcode_descriptor_kind(handle),
        "payloadBase64": ci_descriptor_payload(descriptor).base64EncodedString(),
    ]
    if let qr = descriptor as? CIQRCodeDescriptor {
        details["symbolVersion"] = qr.symbolVersion
        details["maskPattern"] = qr.maskPattern
        details["errorCorrectionLevel"] = qr.errorCorrectionLevel.rawValue
    }
    if let aztec = descriptor as? CIAztecCodeDescriptor {
        details["isCompact"] = aztec.isCompact
        details["layerCount"] = aztec.layerCount
        details["dataCodewordCount"] = aztec.dataCodewordCount
    }
    if let pdf = descriptor as? CIPDF417CodeDescriptor {
        details["isCompact"] = pdf.isCompact
        details["rowCount"] = pdf.rowCount
        details["columnCount"] = pdf.columnCount
    }
    if let dataMatrix = descriptor as? CIDataMatrixCodeDescriptor {
        details["rowCount"] = dataMatrix.rowCount
        details["columnCount"] = dataMatrix.columnCount
        details["eccVersion"] = dataMatrix.eccVersion.rawValue
    }
    return ci_string(ci_json_string(from: details) ?? "{}")
}
