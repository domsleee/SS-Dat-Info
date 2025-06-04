import { BufferGeometry, CylinderGeometry, DoubleSide, Float32BufferAttribute, Group, Mesh, MeshBasicMaterial, MeshStandardMaterial, PlaneGeometry, Vector3 } from "three";

interface Coordinate {
  x: number;
  y: number;
  z: number;
}

interface JumpPoint {
  index: number;
  position: Coordinate;
  acceleration: number;
  type: 'takeoff' | 'landing';
}

export class SnowboardTrackAnalyzer {
  private readonly coordinates: Coordinate[];
  private readonly jumpDetectionConfig: {
    // velocityThreshold: number;
    // minJumpDuration: number;
    // landingThreshold: number;
    accelerationThreshold: number;
    requiredDuration: number;
  };

  constructor(coordinates: Coordinate[], config: {
    accelerationThreshold?: number;
    requiredDuration?: number;
  } = {}) {
    this.coordinates = coordinates;
    this.jumpDetectionConfig = {
      accelerationThreshold: config.accelerationThreshold ?? 4.5,
      requiredDuration: config.requiredDuration ?? 50
    };
  }

  private calculateAcceleration(index: number): number {
    if (index < 1 || index >= this.coordinates.length - 1) return 0;
        
    const dt = 1/100;
    const prevY = this.coordinates[index - 1].y;
    const currY = this.coordinates[index].y;
    const nextY = this.coordinates[index + 1].y;
        
    return -(nextY - 2*currY + prevY) / (dt * dt);
  }

  public detectJumps(): JumpPoint[] {
    const jumps: JumpPoint[] = [];
    const {
      accelerationThreshold,
      requiredDuration
    } = this.jumpDetectionConfig;
        
    let jumpStart: number | null = null;
    let sustainedCount = 0;
        
    for (let i = 1; i < this.coordinates.length - 1; i++) {
      const acceleration = this.calculateAcceleration(i);
            
      if (acceleration > accelerationThreshold) {
        sustainedCount++;
                
        if (jumpStart === null) {
          jumpStart = i;
        }
                
        if (sustainedCount === requiredDuration && jumpStart !== null) {
          jumps.push({
            index: jumpStart,
            position: this.coordinates[jumpStart],
            acceleration,
            type: 'takeoff'
          });
        }
      } else {
        if (jumpStart !== null && sustainedCount >= requiredDuration) {
          jumps.push({
            index: i,
            position: this.coordinates[i],
            acceleration,
            type: 'landing'
          });
        }
                
        jumpStart = null;
        sustainedCount = 0;
      }
    }
        
    return jumps;
  }

  public createSlopeMesh(options: {
    width?: number;
    roughness?: number;
    metalness?: number;
    playerYOffset?: number;
    debug?: boolean;
  } = {}): Group {
    const {
      width = 30,
      roughness = 0.8,
      metalness = 0.2,
      playerYOffset = 1,
      debug = false
    } = options;

    const group = new Group();
    const jumps = this.detectJumps();

    const getPerpendicularVector = (direction: Vector3) => {
      return new Vector3(-direction.z, 0, direction.x).normalize().multiplyScalar(width/2);
    };

    const createContinuousGeometry = (startIndex: number, endIndex: number) => {
      const vertices: number[] = [];
      const indices: number[] = [];

      for (let i = startIndex; i <= endIndex; i++) {
        const curr = this.coordinates[i];
        const next = this.coordinates[Math.min(i + 1, endIndex)];
                
        const direction = new Vector3(
          next.x - curr.x,
          next.y - curr.y,
          next.z - curr.z
        ).normalize();

        const perpVector = getPerpendicularVector(direction);

        // Add left and right vertices
        vertices.push(
          curr.x + perpVector.x,
          curr.y - playerYOffset,
          curr.z + perpVector.z,

          curr.x - perpVector.x,
          curr.y - playerYOffset,
          curr.z - perpVector.z
        );

        // Create triangles (except for last point)
        if (i < endIndex) {
          const vertexIndex = (i - startIndex) * 2;
                    
          indices.push(
            vertexIndex,
            vertexIndex + 1,
            vertexIndex + 2,
                        
            vertexIndex + 1,
            vertexIndex + 3,
            vertexIndex + 2
          );
        }
      }

      const geometry = new BufferGeometry();
      geometry.setAttribute('position', new Float32BufferAttribute(vertices, 3));
      geometry.setIndex(indices);
      geometry.computeVertexNormals();
      return geometry;
    };

    const regularMaterial = debug 
      ? new MeshBasicMaterial({
        color: 0xFFFFFF,
        wireframe: true,
        side: DoubleSide
      })
      : new MeshStandardMaterial({
        color: 0xF0F0F0, // Snow white
        roughness: 0.9,  // More rough for snow-like appearance
        metalness: 0.1,  // Less metallic for snow
        side: DoubleSide
      });

    const jumpMaterial = debug
      ? new MeshBasicMaterial({
        color: 0x404040,
        wireframe: true,
        side: DoubleSide
      })
      : new MeshStandardMaterial({
        color: 0x404040, // Darker grey
        roughness,
        metalness,
        side: DoubleSide
      });

    let currentIndex = 0;

    jumps.forEach(jump => {
      if (jump.type === 'takeoff') {
        // Add regular section before jump
        if (currentIndex < jump.index) {
          const regularGeometry = createContinuousGeometry(currentIndex, jump.index);
          const regularMesh = new Mesh(regularGeometry, regularMaterial);
          regularMesh.receiveShadow = true;
          group.add(regularMesh);
        }

        const nextLanding = jumps.find(j => j.type === 'landing' && j.index > jump.index);
        if (nextLanding) {
          // Create straight jump section from takeoff to landing
          const vertices: number[] = [];
          const indices: number[] = [];
                    
          // Just use takeoff and landing points for straight section
          const takeoff = this.coordinates[jump.index];
          const landing = this.coordinates[nextLanding.index];
                    
          const direction = new Vector3(
            landing.x - takeoff.x,
            landing.y - takeoff.y,
            landing.z - takeoff.z
          ).normalize();

          const takeoffPerp = getPerpendicularVector(direction);
          const landingPerp = getPerpendicularVector(direction);

          // Add vertices for just takeoff and landing points
          vertices.push(
            // Takeoff point vertices
            takeoff.x + takeoffPerp.x,
            takeoff.y - playerYOffset,
            takeoff.z + takeoffPerp.z,

            takeoff.x - takeoffPerp.x,
            takeoff.y - playerYOffset,
            takeoff.z - takeoffPerp.z,

            // Landing point vertices
            landing.x + landingPerp.x,
            landing.y - playerYOffset,
            landing.z + landingPerp.z,

            landing.x - landingPerp.x,
            landing.y - playerYOffset,
            landing.z - landingPerp.z
          );

          // Create triangles for the straight section
          indices.push(
            0, 1, 2,  // First triangle
            1, 3, 2   // Second triangle
          );

          const jumpGeometry = new BufferGeometry();
          jumpGeometry.setAttribute('position', new Float32BufferAttribute(vertices, 3));
          jumpGeometry.setIndex(indices);
          jumpGeometry.computeVertexNormals();
                    
          group.add(new Mesh(jumpGeometry, jumpMaterial));
                    
          currentIndex = nextLanding.index;
        }
      }
    });

    // Add final regular section
    if (currentIndex < this.coordinates.length - 1) {
      const finalGeometry = createContinuousGeometry(currentIndex, this.coordinates.length - 1);
      group.add(new Mesh(finalGeometry, regularMaterial));
    }

    return group;
  }

  public createJumpMarkers(options: {
    width?: number;
    takeoffColor?: number;
    landingColor?: number;
  } = {}): Group[] {
    const {
      width = 30, // Track width to properly position flags on sides
      takeoffColor = 0x00ff00,
      landingColor = 0xff0000
    } = options;

    const jumps = this.detectJumps();
        
    return jumps.map(jump => {
      const group = new Group();
            
      // Create appropriately sized flag for on-track placement
      const flagGeometry = new PlaneGeometry(4, 3); // More reasonable size
      const flagMaterial = new MeshBasicMaterial({ 
        color: jump.type === 'takeoff' ? takeoffColor : landingColor,
        side: DoubleSide,
        transparent: false,
      });
      const flag = new Mesh(flagGeometry, flagMaterial);
            
      // Position flag at a reasonable height above the track
      flag.position.y = 5;
      flag.position.x = 2;
            
      // Create proportional pole that extends from ground to flag
      const poleHeight = 12;
      const poleGeometry = new CylinderGeometry(0.1, 0.1, poleHeight);
      const poleMaterial = new MeshBasicMaterial({ 
        color: 0xCCCCCC, // Light grey
      });
      const pole = new Mesh(poleGeometry, poleMaterial);
            
      // Position pole to start at track level and extend up
      pole.position.y = -poleHeight/2 + 5; // Half below to reach ground, offset to flag height
            
      group.add(pole);
      group.add(flag);
            
      // Get direction vector
      const prevIndex = Math.max(0, jump.index - 1);

      const direction = new Vector3(
        this.coordinates[jump.index].x - this.coordinates[prevIndex].x,
        this.coordinates[jump.index].y - this.coordinates[prevIndex].y,
        this.coordinates[jump.index].z - this.coordinates[prevIndex].z
      ).normalize();

      // Calculate perpendicular vector (same as in createSegmentGeometry)
      const perpVector = new Vector3(-direction.z, 0, direction.x)
        .normalize()
        .multiplyScalar(width/2); // Track width plus small gap

      // Position the flag along the perpendicular vector
      const sideDirection = jump.type === 'takeoff' ? -1 : 1; // Left for takeoff, right for landing
      group.position.set(
        jump.position.x + (perpVector.x * sideDirection),
        jump.position.y,
        jump.position.z + (perpVector.z * sideDirection)
      );
            
      return group;
    });
  }
}